use super::api_auth;
use anyhow::Context;
use buildkite_rust::{Build, Buildkite};
use std::{future::Future, time::Duration};
use tokio::time::delay_for;
use url::Url;

pub async fn for_build<F, Fut>(
    build_fn: F,
    runtime_args: crate::cli::RuntimeArgs,
    output: crate::cli::OutputArgs,
) -> anyhow::Result<i32>
where
    F: FnOnce(Buildkite) -> Fut,
    Fut: Future<Output = anyhow::Result<Build>>,
{
    let client = api_auth::client().context("Unable to prepare a Buildkite client")?;

    let mut build = build_fn(client.clone())
        .await
        .context("Unable to find details for a matching build")?;

    output.found_build(&build);

    let timeout_duration = Duration::from_secs(u64::from(runtime_args.timeout));
    debug!(
        "Waiting a maximum of {:?} for build completion",
        timeout_duration
    );
    let mut timeout = delay_for(timeout_duration);

    while !build.is_finished() {
        let poll_pause =
            Duration::from_secs(u64::from(runtime_args.request_cooldown.unwrap_or(60)));

        debug!("Waiting {:?} to poll build", poll_pause);

        delay_for(poll_pause).await;

        info!("Checking build status");

        let build_url = Url::parse(&build.url)?;

        let get_result = tokio::select! {
            _ = &mut timeout => {
                anyhow::bail!("Timed out waiting for build.");
            },
            get_result = client.build().by_url(build_url) => get_result,
        };

        build = get_result
            .context("Unable to retrieve build details")?
            .error_for_status()
            .context("Server response was not successful")?
            .body()
            .context("Unable to deserialize response body")?
            .clone();
    }

    Ok(output.on_completion(&build))
}

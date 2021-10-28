mod retry;

use anyhow::Context;
use buildkite_waiter::{Build, Buildkite};
use std::{future::Future, time::Duration};
use tokio::time::sleep;
use url::Url;

pub async fn by_url(
    url: &str,
    runtime_args: crate::cli::RuntimeArgs,
    output: crate::cli::OutputArgs,
) -> anyhow::Result<i32> {
    for_build(
        |client| async move {
            let resp = client
                .build_by_url(url)
                .await
                .context("Failed to retrieve build")?;

            Ok(resp)
        },
        runtime_args,
        output,
    )
    .await
}

pub async fn for_build<F, Fut>(
    build_fn: F,
    runtime_args: crate::cli::RuntimeArgs,
    output: crate::cli::OutputArgs,
) -> anyhow::Result<i32>
where
    F: FnOnce(Buildkite) -> Fut,
    Fut: Future<Output = anyhow::Result<Build>>,
{
    if output.notification {
        warn!("--notification is deprecated: os notifications are now sent by default");
    }

    let client = crate::app::auth::client().context("Unable to prepare a Buildkite client")?;

    let mut build = build_fn(client.clone())
        .await
        .context("Unable to find details for a matching build")?;

    output.found_build(&build);

    let timeout_duration = Duration::from_secs(u64::from(runtime_args.timeout));
    debug!(
        "Waiting a maximum of {:?} for build completion",
        timeout_duration
    );

    while !build.is_finished() {
        let poll_pause = Duration::from_secs(30);

        debug!("Waiting {:?}s to poll build", poll_pause);

        sleep(poll_pause).await;

        info!("Checking build status");

        let build_url = Url::parse(&build.url)?;

        if let Ok(get_result) = tokio::time::timeout(
            timeout_duration,
            retry::attempt_build_by_url(&client, &build_url, 3),
        )
        .await
        {
            build = get_result.context("Unable to retrieve build details")?;
        } else {
            anyhow::bail!("Timed out waiting for build details");
        }
    }

    Ok(output.on_completion(&build).await)
}

// TODO: handle rate limiting
// headers:
// "rate-limit-remaining": "99",
// "rate-limit-limit": "100",
// "rate-limit-reset": "58",

use anyhow::Context;
use crate::cli;
use buildkite_rust::BuildState;
use output::NotificationContent;
use url::Url;

mod find;
mod output;

pub async fn wait(client: buildkite_rust::Buildkite, args: &cli::WaitArgs) -> anyhow::Result<(BuildState, NotificationContent)> {
    let mut build = find::build_by_args(&client, &args).await.context("Unable to find referenced build")?;

    output::print_build_info(&build);

    let mut timeout = tokio::time::delay_for(std::time::Duration::from_secs((args.timeout as u64) * 60));

    while !build.is_finished() {
        tokio::time::delay_for(std::time::Duration::from_secs(30)).await;

        info!("Checking build status");

        let build_url = Url::parse(&build.url)?;

        let get_result = tokio::select! {
            _ = &mut timeout => {
                anyhow::bail!("Timed out waiting for build.");
            },
            get_result = client.build().by_url(build_url) => get_result,
        };

        build = get_result.context("Unable to retrieve build details")?
            .error_for_status().context("Server response was not successful")?
            .body().context("Unable to deserialize response body")?
            .clone();
    }

    Ok((build.state, (&build).into()))
}

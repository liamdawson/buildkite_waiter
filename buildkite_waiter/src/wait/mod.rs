use anyhow::Context;
use crate::cli;
use buildkite_rust::BuildState;
use output::NotificationContent;

mod find;
mod output;

pub async fn wait(client: buildkite_rust::Buildkite, args: &cli::WaitArgs) -> anyhow::Result<(BuildState, NotificationContent)> {
    let (organization, build_response) = find::build_by_args(&client, &args).await.context("Unable to find referenced build")?;

    let mut build_response = build_response.error_for_status().context("Server response was not successful")?;
    let mut build = build_response.body().context("Unable to deserialize response body")?;

    output::print_build_info(&build);

    let mut timeout = tokio::time::delay_for(std::time::Duration::from_secs((args.timeout as u64) * 60));

    while !build.is_finished() {
        tokio::time::delay_for(std::time::Duration::from_secs(30)).await;

        info!("Checking build status");

        let get_result = tokio::select! {
            _ = &mut timeout => {
                anyhow::bail!("Timed out waiting for build.");
            },
            get_result = client.build().get(&organization, &build.pipeline.slug, &build.number) => get_result,
        };

        build_response = get_result.context("Unable to retrieve build details")?
            .error_for_status().context("Server response was not successful")?;
        build = build_response.body().context("Unable to deserialize response body")?;
    }

    Ok((build.state, build.into()))
}

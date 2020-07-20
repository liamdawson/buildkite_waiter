use buildkite_rust::{Buildkite, Build};
use crate::cli;
use anyhow::bail;

const EXAMPLE_URL: &str = "https://buildkite.com/my-great-org/my-pipeline/builds/1";

pub async fn build_by_args(client: &Buildkite, args: &cli::WaitArgs) -> anyhow::Result<(String, buildkite_rust::ApiResponse<Build>)> {
    if let Some(url) = &args.url {
        // match by URL
        // expected format: https://buildkite.com/<org>/<pipeline>/builds/<number>

        // TODO: validate hostname
        // TODO(maybe): parse api urls too

        let segments: Vec<_> = match url.path_segments() {
            Some(segments) => segments.collect(),
            None => bail!("Unable to parse path from URL"),
        };

        match segments.as_slice() {
            [org, pipeline, "builds", number] => {
                Ok((org.to_string(), client.build().get(org, pipeline, number).await?))
            },
            _ => bail!("Unable to parse URL, expected format like {}", EXAMPLE_URL),
        }
    } else if let Some(number) = &args.number {
        let organization = match &args.organization {
            Some(organization) => organization,
            _ => bail!("The organization parameter is required when finding a build by number"),
        };

        let pipeline = match &args.pipeline {
            Some(pipeline) => pipeline,
            _ => bail!("The pipeline parameter is required when finding a build by number"),
        };

        Ok((organization.to_string(), client.build().get(organization, pipeline, number).await?))
    } else {
        bail!("Unable to identify a build from the command line arguments")
    }
}

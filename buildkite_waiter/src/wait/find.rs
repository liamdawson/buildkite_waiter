use buildkite_rust::{Buildkite, Build, ApiResponse};
use crate::cli;
use anyhow::{bail, Context, ensure};

const EXAMPLE_URL: &str = "https://buildkite.com/my-great-org/my-pipeline/builds/1";

pub fn single_build(build_response: ApiResponse<Build>) -> anyhow::Result<Build> {
    let build_response = build_response.error_for_status().context("Server response was not successful")?;
    let build = build_response.body().context("Unable to deserialize response body")?;

    Ok(build.clone())
}

pub fn first_build(build_response: ApiResponse<Vec<Build>>) -> anyhow::Result<Build> {
    let build_response = build_response.error_for_status().context("Server response was not successful")?;
    let builds = build_response.body().context("Unable to deserialize response body")?;

    ensure!(!builds.is_empty(), "No builds found matching the filters");

    Ok(builds.first().unwrap().clone())
}

pub async fn build_by_args(client: &Buildkite, args: &cli::WaitArgs) -> anyhow::Result<Build> {
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
            [organization, pipeline, "builds", number] => {
                single_build(client.build().get(organization, pipeline, number).await?)
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

        single_build(client.build().get(organization, pipeline, number).await?)
    } else if (&args).latest {
        let req = if let Some(pipeline) = &args.pipeline {
            if let Some(organization) = &args.organization {
                client.builds().pipeline(organization, pipeline)
            } else {
                bail!("The organization parameter is required when finding a build for a pipeline");
            }
        } else if let Some(organization) = &args.pipeline {
            client.builds().organization(organization)
        } else {
            client.builds().all()
        };

        let resp = req.get().await?;

        first_build(resp)
    } else {
        bail!("Unable to identify a build from the command line arguments")
    }
}

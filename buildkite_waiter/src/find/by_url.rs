use anyhow::{bail, ensure, Context};
use buildkite_rust::{Build, Buildkite};

impl crate::cli::ByUrlStrategyArgs {
    pub async fn find_build(&self, client: &Buildkite) -> anyhow::Result<Build> {
        ensure!(
            !self.url.cannot_be_a_base(),
            "URL was not in an expected format"
        );

        let path_segments: Vec<_> = match self.url.path_segments() {
            None => bail!("Unable to interpret path from URL"),
            Some(val) => val.collect(),
        };

        let (organization, pipeline, raw_number) = match path_segments.as_slice() {
            [organization, pipeline, "builds", raw_number, ..] => {
                (organization, pipeline, raw_number)
            }
            _ => bail!("URL Path was not in an expected format"),
        };

        let build_number = raw_number
            .parse()
            .context("Unable to determine build number")?;

        let response = client
            .build()
            .get(organization, pipeline, build_number)
            .await
            .context("Request could not complete")?;

        let success_response = response
            .error_for_status()
            .context("Request was unsuccessful")?;

        let build = success_response
            .body()
            .context("Unexpected response body")?;

        Ok(build.clone())
    }
}

use anyhow::Context;
use buildkite_rust::{Build, Buildkite};

impl crate::cli::ByNumberStrategyArgs {
    pub async fn find_build(&self, client: &Buildkite) -> anyhow::Result<Build> {
        let response = client
            .build()
            .get(&self.organization, &self.pipeline, self.number)
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

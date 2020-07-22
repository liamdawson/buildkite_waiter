use anyhow::{bail, Context};
use buildkite_rust::{Build, Buildkite};

impl crate::cli::LatestStrategyArgs {
    pub async fn find_build(&self, client: &Buildkite) -> anyhow::Result<Build> {
        let mut req = if let Some(pipeline) = &self.pipeline {
            if let Some(organization) = &self.organization {
                client.builds().pipeline(organization, pipeline)
            } else {
                bail!("Organization is required if pipeline is provided");
            }
        } else if let Some(organization) = &self.organization {
            client.builds().organization(organization)
        } else {
            client.builds().all()
        };

        req = req.per_page(1);

        if !self.branch.is_empty() {
            req = req.branches(&self.branch);
        }

        if !self.state.is_empty() {
            req = req.states(&self.state);
        }

        if let Some(creator) = &self.creator {
            req = req.creator(creator);
        }

        if let Some(commit) = &self.commit {
            req = req.commit(commit);
        }

        let response = req.get().await.context("Request could not complete")?;

        let success_response = response
            .error_for_status()
            .context("Request was unsuccessful")?;

        let builds = success_response
            .body()
            .context("Unexpected response body")?;

        match builds.first() {
            Some(build) => Ok(build.clone()),
            None => bail!("No matching builds were found"),
        }
    }
}

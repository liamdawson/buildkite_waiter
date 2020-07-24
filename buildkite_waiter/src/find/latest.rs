use anyhow::{bail, Context};
use buildkite_rust::{Build, Buildkite};

async fn get_current_user_id(client: &Buildkite) -> anyhow::Result<String> {
    Ok(client
        .user()
        .get_access_token_holder()
        .await?
        .error_for_status()?
        .body()?
        .id
        .clone())
}

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
        } else if self.mine {
            let user_id = get_current_user_id(&client).await.context("Unable to determine the current user (the API Access Token may need the \"Read User\" permission)")?;

            debug!("Current user's ID: {}", &user_id);

            req = req.creator(&user_id);
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

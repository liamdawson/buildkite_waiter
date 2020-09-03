use super::{Buildkite, BuildkiteCredentials};
use reqwest::Method;

pub enum BuildScope {
    All,
    Organization(String),
    Pipeline(String, String),
}

#[derive(serde::Deserialize)]
pub struct BuildInfo {
    pub number: u64,
    pub url: String,
}

type FindResult = Result<BuildInfo, reqwest::Error>;
type OptionalFindResult = Result<Option<BuildInfo>, reqwest::Error>;

impl Buildkite {
    pub async fn build_by_url(&self, credentials: BuildkiteCredentials, url: &str) -> FindResult {
        Ok(self
            .request(Method::GET, url, credentials)?
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn build_by_number(
        &self,
        credentials: BuildkiteCredentials,
        organization: &str,
        pipeline: &str,
        number: &str,
    ) -> FindResult {
        Ok(self
            .path_request(
                Method::GET,
                &format!(
                    "organizations/{}/pipelines/{}/builds/{}",
                    organization, pipeline, number
                ),
                credentials,
            )?
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn latest_build(
        &self,
        credentials: BuildkiteCredentials,
        scope: BuildScope,
        branches: &[&str],
        creator: Option<&str>,
        commit: Option<&str>,
        states: &[&str],
    ) -> OptionalFindResult {
        let path = match scope {
            BuildScope::All => format!("{}", "builds"),
            BuildScope::Organization(organization) => format!("organizations/{}/builds", &organization),
            BuildScope::Pipeline(organization, pipeline) => format!("organizations/{}/pipelines/{}/builds", &organization, &pipeline),
        };

        let mut query = Vec::new();

        query.extend(branches.into_iter().map(|&branch| ("branch[]", branch)));
        query.extend(states.into_iter().map(|&state| ("state[]", state)));

        if let Some(creator) = creator {
            query.push(("creator", creator));
        }

        if let Some(commit) = commit {
            query.push(("commit", commit));
        }

        let mut builds: Vec<BuildInfo> = self
            .path_request(
                Method::GET,
                &path,
                credentials,
            )?
            .query(query.as_slice())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if !builds.is_empty() {
            Ok(Some(builds.remove(0)))
        } else {
            Ok(None)
        }
    }
}

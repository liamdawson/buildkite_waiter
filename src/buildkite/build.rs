use super::error::RequestError;
use crate::Buildkite;
use chrono::{DateTime, Utc};

pub enum BuildScope {
    All,
    Organization(String),
    Pipeline(String, String),
}

#[derive(serde::Deserialize)]
pub struct Build {
    pub number: u64,
    pub url: String,
    pub web_url: String,
    pub state: String,
    pub pipeline: BuildPipeline,
    pub creator: Option<BuildCreator>,
    pub branch: String,
    pub message: Option<String>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize)]
pub struct BuildPipeline {
    pub slug: String,
}

#[derive(serde::Deserialize)]
pub struct BuildCreator {
    pub name: Option<String>,
}

impl Build {
    pub fn is_finished(&self) -> bool {
        // Corresponds to the `finished` state for the Buildkite API:
        super::build_states::FINISHED_BUILD_STATES.contains(&self.state.as_str())
    }
}

type FindResult = Result<Build, RequestError>;
type OptionalFindResult = Result<Option<Build>, RequestError>;

impl Buildkite {
    pub fn build_by_url(&self, url: &str) -> FindResult {
        Ok(self.request("GET", url).call()?.into_json()?)
    }

    pub fn build_by_number(&self, organization: &str, pipeline: &str, number: &str) -> FindResult {
        Ok(self
            .path_request(
                "GET",
                &format!(
                    "organizations/{}/pipelines/{}/builds/{}",
                    organization, pipeline, number
                ),
            )
            .call()?
            .into_json()?)
    }

    pub fn latest_build(
        &self,
        scope: BuildScope,
        branches: &[&str],
        creator: Option<&str>,
        commit: Option<&str>,
        states: &[&str],
    ) -> OptionalFindResult {
        let path = match scope {
            BuildScope::All => format!("{}", "builds"),
            BuildScope::Organization(organization) => {
                format!("organizations/{}/builds", &organization)
            }
            BuildScope::Pipeline(organization, pipeline) => format!(
                "organizations/{}/pipelines/{}/builds",
                &organization, &pipeline
            ),
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

        let mut req = self.path_request("GET", &path);

        for (key, value) in query {
            req = req.query(key, value);
        }

        let mut builds: Vec<Build> = req.call()?.into_json()?;

        if !builds.is_empty() {
            Ok(Some(builds.remove(0)))
        } else {
            Ok(None)
        }
    }
}

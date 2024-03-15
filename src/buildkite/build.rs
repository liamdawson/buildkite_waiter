use super::error::RequestError;
use crate::Buildkite;
use chrono::{DateTime, Utc};
use reqwest::Method;

// Corresponds to the `finished` state for the Buildkite API:
const FINISHED_BUILD_STATES: &[&str] = &["passed", "failed", "blocked", "canceled"];

pub enum BuildScope {
    All,
    Organization(String),
    Pipeline(String, String),
}

#[derive(Debug, Clone, Copy)]
pub enum BuildStateGroup {
    Successful,
    Blocked,
    Unsuccessful,
    Unfinished,
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
        FINISHED_BUILD_STATES.contains(&self.state.as_str())
    }

    pub fn state_group(&self) -> BuildStateGroup {
        match self.state.as_str() {
            "passed" => BuildStateGroup::Successful,
            "blocked" => BuildStateGroup::Blocked,
            "failed" | "canceled" => BuildStateGroup::Unsuccessful,
            _ => BuildStateGroup::Unfinished,
        }
    }
}

type FindResult = Result<Build, RequestError>;
type OptionalFindResult = Result<Option<Build>, RequestError>;

impl Buildkite {
    pub async fn build_by_url(&self, url: &str) -> FindResult {
        Ok(self
            .request(Method::GET, url)?
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn build_by_number(
        &self,
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
            )?
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn latest_build(
        &self,
        scope: BuildScope,
        branches: &[&str],
        creator: Option<&str>,
        commit: Option<&str>,
        states: &[&str],
    ) -> OptionalFindResult {
        let path = match scope {
            BuildScope::All => "builds".to_string(),
            BuildScope::Organization(organization) => {
                format!("organizations/{}/builds", &organization)
            }
            BuildScope::Pipeline(organization, pipeline) => format!(
                "organizations/{}/pipelines/{}/builds",
                &organization, &pipeline
            ),
        };

        let mut query = Vec::new();

        query.extend(branches.iter().map(|&branch| ("branch[]", branch)));
        query.extend(states.iter().map(|&state| ("state[]", state)));

        if let Some(creator) = creator {
            query.push(("creator", creator));
        }

        if let Some(commit) = commit {
            query.push(("commit", commit));
        }

        let mut builds: Vec<Build> = self
            .path_request(Method::GET, &path)?
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

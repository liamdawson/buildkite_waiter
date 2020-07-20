use super::job::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{Utc, DateTime};
use strum_macros::*;

pub type BuildNumber = u32;

type NullableDateTime = Option<chrono::DateTime<chrono::Utc>>;
// TODO: enum-ify
// GraphQL docs say:
//   API
//   Frontend
//   Schedule
//   TriggerJob
//   Webhook
type BuildSource = String;
type BuildPullRequest = Value;
type BuildPipelineProvider = Value;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BuildCreator {
    pub avatar_url: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub created_at: NullableDateTime,

    #[cfg(debug_assertions)]
    #[serde(flatten)]
    pub unknown_properties: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BuildPipeline {
    pub id: String,
    pub url: String,
    pub name: String,
    pub slug: String,
    pub repository: Option<String>,
    pub provider: Option<BuildPipelineProvider>,
    pub skip_queued_branch_builds: bool,
    pub skip_queued_branch_builds_filter: Value,
    pub cancel_running_branch_builds: bool,
    pub cancel_running_branch_builds_filter: Value,
    pub builds_url: String,
    pub badge_url: String,
    pub created_at: DateTime<Utc>,
    pub scheduled_builds_count: u32,
    pub running_builds_count: u32,
    pub scheduled_jobs_count: u32,
    pub running_jobs_count: u32,
    pub waiting_jobs_count: u32,

    #[cfg(debug_assertions)]
    #[serde(flatten)]
    pub unknown_properties: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Build {
    pub blocked: Option<bool>,
    pub branch: String,
    pub canceled_at: NullableDateTime,
    pub commit: String,
    pub created_at: NullableDateTime,
    pub creator: Option<BuildCreator>,
    pub env: Option<HashMap<String, String>>,
    pub finished_at: NullableDateTime,
    pub message: Option<String>,
    pub meta_data: Value,
    pub number: BuildNumber,
    pub pipeline: BuildPipeline,
    pub pull_request: Option<BuildPullRequest>,
    pub scheduled_at: NullableDateTime,
    pub source: BuildSource,
    pub started_at: NullableDateTime,
    pub state: BuildState,
    pub url: String,
    pub id: String,
    pub jobs: Vec<BuildJob>,
    pub web_url: String,

    #[cfg(debug_assertions)]
    #[serde(flatten)]
    pub unknown_properties: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Display, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
/// State of the build
pub enum BuildState {
    /// The build was skipped
    Skipped,
    /// The build has yet to start running jobs
    Scheduled,
    /// The build is currently running jobs
    Running,
    /// The build passed
    Passed,
    /// The build failed
    Failed,
    /// The build is currently being cancelled
    Canceling,
    /// The build was canceled
    Canceled,
    /// The build is blocked
    Blocked,
    /// The build wasn't run
    NotRun,
}

impl Build {
    pub fn is_finished(&self) -> bool {
        // Corresponds to the `finished` state for the Buildkite API:
        match &self.state {
            BuildState::Passed
            | BuildState::Failed
            | BuildState::Blocked
            | BuildState::Canceled => true,

            _ => false,
        }
    }

    pub fn short_message(&self) -> Option<&str> {
        if let Some(message) = &self.message {
            message.lines().next()
        } else {
            None
        }
    }
}

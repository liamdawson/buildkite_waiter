use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use chrono::{Utc, DateTime};

type NullableDateTime = Option<DateTime<Utc>>;
type TriggerBuildJob = Value;
type BlockBuildJob = Value;
type WaitBuildJob = Value;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BuildJob {
    #[serde(rename = "script")]
    Command(CommandBuildJob),
    Trigger(TriggerBuildJob),
    #[serde(rename = "waiter")]
    Wait(WaitBuildJob),
    #[serde(rename = "manual")]
    Block(BlockBuildJob),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CommandBuildJob {
    pub id: String,
    pub name: Option<String>,
    pub step_key: Option<String>,
    pub agent_query_rules: Value,
    // TODO: are REST docs wrong, or are the names different from the GraphQL versions?
    // pub state: BuildJobState,
    pub state: String,
    pub web_url: String,
    pub log_url: String,
    pub raw_log_url: String,
    pub command: Value,
    pub exit_status: Option<i32>,
    pub artifact_paths: Value,
    pub agent: Value,
    pub created_at: NullableDateTime,
    pub scheduled_at: NullableDateTime,
    pub runnable_at: NullableDateTime,
    pub started_at: NullableDateTime,
    pub finished_at: NullableDateTime,
    pub soft_failed: Option<bool>,

    #[cfg(debug_assertions)]
    #[serde(flatten)]
    pub unknown_properties: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
/// State of a build's job
pub enum BuildJobState {
    /// The job has just been created and doesn't have a state yet
    Pending,
    /// The job is waiting on a wait step to finish
    Waiting,
    /// The job was in a Waiting state when the build failed
    WaitingFailed,
    /// The job is waiting on a Block step to finish
    Blocked,
    /// The job was in a Blocked state when the build failed
    BlockedFailed,
    /// This Block job has been manually unblocked
    Unblocked,
    /// This Block job was in a Blocked state when the build failed
    UnblockedFailed,
    /// The job is waiting on a concurrency group check before becoming either Limited or Scheduled
    Limiting,
    /// The job is waiting for jobs with the same concurrency group to finish
    Limited,
    /// The job is scheduled and waiting for an agent
    Scheduled,
    /// The job has been assigned to an agent, and it's waiting for it to accept
    Assigned,
    /// The job was accepted by the agent, and now it's waiting to start running
    Accepted,
    /// The job is running
    Running,
    /// The job has finished
    Finished,
    /// The job is currently canceling
    Canceling,
    /// The job was canceled
    Canceled,
    /// The job is timing out for taking too long
    TimingOut,
    /// The job timed out
    TimedOut,
    /// The job was skipped
    Skipped,
    /// The job's configuration means that it can't be run
    Broken,
}

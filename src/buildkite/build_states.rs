// manually included in `build.rs`,

pub const KNOWN_BUILD_STATES: &[&str] = &["running", "scheduled", "passed", "failed", "blocked", "canceled", "canceling", "skipped", "not_run"];
// Corresponds to the `finished` state for the Buildkite API:
pub const FINISHED_BUILD_STATES: &[&str] = &["passed", "failed", "blocked", "canceled"];

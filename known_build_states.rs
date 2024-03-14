// this file is manually included to support CLI completions at build time

pub const KNOWN_BUILD_STATES: &[&str] = &[
    "running",
    "scheduled",
    "passed",
    "failed",
    "blocked",
    "canceled",
    "canceling",
    "skipped",
    "not_run",
];

#![deny(missing_docs)]

use clap::builder::PossibleValuesParser;
use clap::{Args, Parser, Subcommand, ValueEnum, ValueHint};
use url::Url;

const ALLOWED_BUILD_STATE_VALUES: &[&str] = &[
    "running",
    "scheduled",
    "passed",
    "failed",
    "blocked",
    "canceled",
    "canceling",
    "skipped",
    "not_run",
    "finished"
];

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputType {
    /// Notification title and message
    NotificationLines,
    #[default]
    /// Build state and browser URL
    StateUrl,
    /// No output
    None
}

#[derive(Debug, Parser, PartialEq, Clone)] // requires `derive` feature
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, PartialEq, Clone)]
#[command(version, about, long_about)]
pub enum Commands {
    /// Save a Buildkite API Access Token
    Login,
    /// Remove the saved Buildkite API token
    Logout,
    /// Wait for a build specified by Buildkite web URL
    ByUrl {
        #[command(flatten)]
        output: OutputArgs,
        #[command(flatten)]
        runtime: RuntimeArgs,
        #[command(flatten)]
        strategy: ByUrlStrategyArgs,
    },
    /// Wait for a build specified by organization, pipeline and number
    ByNumber {
        #[command(flatten)]
        output: OutputArgs,
        #[command(flatten)]
        runtime: RuntimeArgs,
        #[command(flatten)]
        strategy: ByNumberStrategyArgs,
    },
    /// Wait for the latest build matching certain filter criteria
    Latest {
        #[command(flatten)]
        output: OutputArgs,
        #[command(flatten)]
        runtime: RuntimeArgs,
        #[command(flatten)]
        strategy: LatestStrategyArgs,
    },
}

#[derive(Args, Debug, PartialEq, Clone)]
pub struct ByNumberStrategyArgs {
    #[arg(value_hint = ValueHint::Other)]
    /// Organization slug
    pub organization: String,
    #[arg(value_hint = ValueHint::Other)]
    /// Pipeline slug
    pub pipeline: String,
    #[arg(value_hint = ValueHint::Other)]
    /// Build number
    pub number: u32,
}

#[derive(Args, Debug, PartialEq, Clone)]
pub struct ByUrlStrategyArgs {
    #[arg(value_hint = ValueHint::Url)]
    pub url: Url,
}

#[derive(Args, Debug, PartialEq, Clone)]
pub struct LatestStrategyArgs {
    #[arg(long, value_hint = ValueHint::Other)]
    pub organization: Option<String>,
    #[arg(long, requires("organization"), value_hint = ValueHint::Other)]
    pub pipeline: Option<String>,
    #[arg(long, value_hint = ValueHint::Other)]
    pub branch: Vec<String>,
    #[arg(long)]
    /// Find build by owner of the API Access Token (requires the "Read User" permission on the token)
    pub mine: bool,
    #[arg(long, conflicts_with("mine"), value_hint = ValueHint::Other)]
    /// Find build by creator ID
    pub creator: Option<String>,
    #[arg(long, value_hint = ValueHint::Other)]
    /// Find build by (long) commit hash
    pub commit: Option<String>,
    #[arg(long, value_parser = PossibleValuesParser::new(ALLOWED_BUILD_STATE_VALUES))]
    pub state: Vec<String>,
}

#[derive(Args, Debug, PartialEq, Clone)]
pub struct RuntimeArgs {
    #[arg(long, default_value = "3600", value_hint = ValueHint::Other)]
    /// Maximum time to wait for the build, in seconds
    pub timeout: u32,

    // TODO: expose or hardcode?
    #[arg(skip)]
    pub request_cooldown: Option<u32>,
}

#[derive(Args, Debug, PartialEq, Clone)]
pub struct OutputArgs {
    // deprecated, now default behaviour
    #[arg(long, hide = true)]
    pub notification: bool,

    #[arg(long, hide(!cfg!(feature = "os-notifications")))]
    /// Never send a system notification
    pub no_notification: bool,

    #[arg(long, value_enum)]
    pub output: OutputType,
}

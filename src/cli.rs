use structopt::StructOpt;
use url::Url;

fn allowed_build_states() -> Vec<&'static str> {
    let mut states = Vec::from(buildkite_waiter::build_states::KNOWN_BUILD_STATES);
    states.push("finished");

    states
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub enum Command {
    /// Save a Buildkite API Access Token
    Login,
    /// Remove the saved Buildkite API token
    Logout,
    /// Wait for a build specified by Buildkite web URL
    ByUrl {
        #[structopt(flatten)]
        output: OutputArgs,
        #[structopt(flatten)]
        runtime: RuntimeArgs,
        #[structopt(flatten)]
        strategy: ByUrlStrategyArgs,
    },
    /// Wait for a build specified by organization, pipeline and number
    ByNumber {
        #[structopt(flatten)]
        output: OutputArgs,
        #[structopt(flatten)]
        runtime: RuntimeArgs,
        #[structopt(flatten)]
        strategy: ByNumberStrategyArgs,
    },
    /// Wait for the latest build matching certain filter criteria
    Latest {
        #[structopt(flatten)]
        output: OutputArgs,
        #[structopt(flatten)]
        runtime: RuntimeArgs,
        #[structopt(flatten)]
        strategy: LatestStrategyArgs,
    },
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub struct ByNumberStrategyArgs {
    pub organization: String,
    pub pipeline: String,
    pub number: u32,
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub struct ByUrlStrategyArgs {
    pub url: Url,
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub struct LatestStrategyArgs {
    #[structopt(long)]
    pub organization: Option<String>,
    #[structopt(long, requires("organization"))]
    pub pipeline: Option<String>,
    #[structopt(long)]
    pub branch: Vec<String>,
    #[structopt(long)]
    /// Find build by owner of the API Access Token (requires the "Read User" permission on the token)
    pub mine: bool,
    #[structopt(long, conflicts_with("mine"))]
    /// Find build by creator ID
    pub creator: Option<String>,
    #[structopt(long)]
    /// Find build by (long) commit hash
    pub commit: Option<String>,
    #[structopt(long, possible_values = allowed_build_states().as_slice())]
    pub state: Vec<String>,
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub struct RuntimeArgs {
    #[structopt(long, default_value = "3600")]
    /// Maximum time to wait for the build, in seconds
    pub timeout: u64,
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub struct OutputArgs {
    // deprecated, now default behaviour
    #[structopt(long, hidden(true))]
    pub notification: bool,

    // if notification support wasn't compiled
    // keep for compatibility (but don't show in help output)
    #[cfg(not(feature = "os-notifications"))]
    #[structopt(long, hidden(true))]
    pub no_notification: bool,

    #[cfg(feature = "os-notifications")]
    #[structopt(long)]
    /// Never send a system notification
    pub no_notification: bool,

    #[structopt(long, possible_values = &["notification-lines", "state-url", "none"], default_value = "state-url")]
    pub output: String,
}

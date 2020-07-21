use url::Url;
use structopt::{clap::ArgGroup,StructOpt};
use clap::arg_enum;

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub enum Command {
    /// Save a Buildkite API Access Token
    Login,
    /// Remove the saved Buildkite API token
    Logout,
    /// Wait for a build to finish
    Wait(WaitArgs),
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
#[structopt(group = ArgGroup::with_name("build").required(true))]
#[structopt(group = ArgGroup::with_name("output").required(false))]
pub struct WaitArgs {
    #[structopt(long, group = "build")]
    /// Specify build by the web URL
    pub url: Option<Url>,

    #[structopt(long)]
    /// Specify the organization containing the build
    pub organization: Option<String>,
    #[structopt(long, requires("organization"))]
    /// Specify the pipeline containing the build
    pub pipeline: Option<String>,
    #[structopt(long, requires("pipeline"), group = "build")]
    /// Specify the build number within the pipeline
    pub number: Option<u32>,

    #[structopt(long, group = "build")]
    /// Find the latest build matching the other filters
    pub latest: bool,

    #[structopt(long)]
    /// Display a system notification when the app finishes waiting for a build
    pub notification: bool,

    #[structopt(long, default_value = "480")]
    /// Maximum time to wait for the build, in minutes
    pub timeout: u32,

    #[structopt(long, group = "output")]
    /// Output the contents of the default notification to stdout as a JSON object
    pub output_notification_json: bool,

    // TODO: nicer value format
    #[structopt(long, possible_values = &ExitStatusStrategy::variants(), default_value = "CheckOnly", case_insensitive = true)]
    /// Choose when to return an error exit code
    pub error_on: ExitStatusStrategy,
}

arg_enum! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum ExitStatusStrategy {
        CheckOnly,
        BuildFailedOrCanceled,
    }
}

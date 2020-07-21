use url::Url;
use structopt::StructOpt;
use structopt::clap::{ArgGroup, AppSettings, arg_enum};

#[derive(StructOpt, Debug, PartialEq, Clone)]
pub enum Command {
    /// Save a Buildkite API Access Token
    Login,
    /// Remove the saved Buildkite API token
    Logout,
    /// Wait for a build specified by URL
    ByUrl,
    /// Wait for a build specified by organization, pipeline and number
    ByNumber,
    /// Wait for the latest build matching certain filter criteria
    Latest,
    /// Wait for a build to finish
    #[structopt(setting(AppSettings::Hidden), setting(AppSettings::TrailingVarArg), setting(AppSettings::AllowLeadingHyphen))]
    Wait {
        raw_parameters: Vec<String>
    },
}

#[derive(StructOpt, Debug, PartialEq, Clone)]
#[structopt(group = ArgGroup::with_name("build").required(true))]
#[structopt(group = ArgGroup::with_name("output").required(false))]
pub struct WaitArgs {
    // Build selectors

    #[structopt(long, group = "build")]
    /// Specify build by the web URL
    pub url: Option<Url>,
    #[structopt(long, requires("pipeline"), group = "build")]
    /// Specify the build number within the pipeline
    pub number: Option<u32>,
    #[structopt(long, group = "build")]
    /// Find the latest build matching the other filters
    pub latest: bool,

    // Filters

    #[structopt(long)]
    /// Specify the organization containing the build
    pub organization: Option<String>,
    #[structopt(long, requires("organization"))]
    /// Specify the pipeline containing the build
    pub pipeline: Option<String>,
    #[structopt(long, requires("latest"))]
    /// Filter to builds on one or more branches
    pub branch: Vec<String>,

    // Behaviour

    #[structopt(long, default_value = "480")]
    /// Maximum time to wait for the build, in minutes
    pub timeout: u32,
    // TODO: nicer value format
    #[structopt(long, possible_values = &ExitStatusStrategy::variants(), default_value = "CheckOnly", case_insensitive = true)]
    /// Choose when to return an error exit code
    pub error_on: ExitStatusStrategy,

    // Outputs

    #[structopt(long)]
    /// Display a system notification when the app finishes waiting for a build
    pub notification: bool,
    #[structopt(long, group = "output")]
    /// Output the contents of the default notification to stdout as a JSON object
    pub output_notification_json: bool,
}

arg_enum! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum ExitStatusStrategy {
        CheckOnly,
        BuildFailedOrCanceled,
    }
}

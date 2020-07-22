mod notification;

pub use notification::NotificationContent;

use buildkite_rust::{Build, BuildState};
use console::style;
use std::io::Write;

impl crate::cli::OutputArgs {
    pub fn found_build(&self, build: &Build) {
        let mut line = format!(
            "{} {}",
            style("Waiting for").dim(),
            style(format!("{}/{}", &build.pipeline.slug, &build.number)).green()
        );
        if let Some(creator) = &build.creator {
            if let Some(name) = &creator.name {
                line = format!("{} {} {}", line, style("by").dim(), style(name).cyan());
            }
        }
        line = format!(
            "{} {}",
            line,
            format!(
                "{} {}",
                style("on branch").dim(),
                style(&build.branch).cyan()
            )
        );

        // use writeln! and .ok(), because it's fine if the output couldn't be written
        writeln!(std::io::stderr(), "{}", line).ok();

        if let Some(message) = &build.message {
            if let Some(first_line) = message.lines().next() {
                writeln!(std::io::stderr(), "  {}", first_line).ok();
            }
        }
    }

    pub fn on_completion(&self, build: &Build) -> i32 {
        let notification_content: NotificationContent = build.into();

        match self.output.as_str() {
            "none" => { },
            "notification-json" => {
                match serde_json::to_string(&notification_content) {
                    Ok(json) => println!("{}", json),
                    Err(e) => warn!("Unable to serialize JSON output: {}", e),
                }
            },
            _ => {
                // should only occur if a new possible_value is added to cli.rs
                unreachable!("Output format has not been defined")
            },
        }

        if self.notification {
            if let Err(e) = notification_content.show_notification() {
                warn!("Unable to send system notification: {}", e);
            }
        }

        match build.state {
            BuildState::Passed | BuildState::Blocked => 0,
            _ => 2,
        }
    }
}

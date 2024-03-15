mod notification;
mod state;

pub use notification::NotificationContent;
pub(crate) use state::*;

use buildkite_waiter::Build;
use console::style;
use std::io::Write;

use crate::cli::OutputType;

impl crate::cli::OutputArgs {
    pub fn should_notify(&self) -> bool {
        !self.no_notification
    }

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
            "{} {} {}",
            line,
            style("on branch").dim(),
            style(&build.branch).cyan()
        );

        // use writeln! and .ok(), because it's fine if the output couldn't be written
        writeln!(std::io::stderr(), "{}", line).ok();
        writeln!(
            std::io::stderr(),
            "{}",
            style(&build.web_url).underlined().dim()
        )
        .ok();

        if let Some(message) = &build.message {
            writeln!(std::io::stderr()).ok();

            if let Some(first_line) = message.lines().next() {
                writeln!(std::io::stderr(), "  {}", first_line).ok();
            }
        }

        writeln!(std::io::stderr()).ok();
    }

    pub async fn on_completion(&self, build: &Build) -> i32 {
        let notification_content: NotificationContent = build.into();

        match self.output {
            OutputType::None => {}
            OutputType::StateUrl => {
                writeln!(std::io::stderr()).ok();
                println!(
                    "{} {}",
                    build
                        .colored_state_case(state::FormatBuildStateCase::Title)
                        .bold(),
                    build.web_url
                );
            }
            OutputType::NotificationLines => {
                println!("{}", notification_content.title);
                println!("{}", notification_content.message);
            }
        }

        #[cfg(feature = "os-notifications")]
        if self.should_notify() {
            if let Err(e) = notification_content.send_os_notification().await {
                warn!("Unable to send system notification: {}", e);
            }
        }

        match build.state.as_str() {
            "passed" | "blocked" => 0,
            _ => 2,
        }
    }
}

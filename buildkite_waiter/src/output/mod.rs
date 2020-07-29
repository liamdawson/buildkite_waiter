mod notification;

pub use notification::NotificationContent;

use buildkite_rust::{Build, BuildState};
use console::style;
use std::io::Write;

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

    pub async fn on_completion(&self, build: &Build) -> i32 {
        let notification_content: NotificationContent = build.into();

        match self.output.as_str() {
            "none" => {}
            "state-url" => {
                writeln!(std::io::stderr()).ok();
                println!("{} {}", style_state(build.state), build.web_url);
            },
            "notification-lines" => {
                println!("{}", notification_content.title);
                println!("{}", notification_content.message);
            },
            _ => {
                // should only occur if a new possible_value is added to cli.rs
                unreachable!("Output format has not been defined")
            }
        }

        #[cfg(feature = "os-notifications")]
        if self.should_notify() {
            if let Err(e) = notification_content.send_os_notification().await {
                warn!("Unable to send system notification: {}", e);
            }
        }

        match build.state {
            BuildState::Passed | BuildState::Blocked => 0,
            _ => 2,
        }
    }
}

fn style_state(state: BuildState) -> String {
    let state_str = format!("{:?}", state);

    let colored = match state {
        BuildState::Passed => style(state_str).green(),
        BuildState::Blocked => style(state_str).yellow(),
        BuildState::Failed | BuildState::Canceled => style(state_str).red(),
        _ => style(state_str).magenta(),
    };

    format!("{}", colored.bold())
}

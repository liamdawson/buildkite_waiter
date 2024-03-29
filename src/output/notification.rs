use buildkite_waiter::Build;
use heck::ToTitleCase;
use serde::Serialize;

#[derive(Serialize)]
pub struct NotificationContent {
    pub title: String,
    pub message: String,
}

impl From<&Build> for NotificationContent {
    fn from(build: &Build) -> Self {
        let finished_ago = chrono::Utc::now()
            - build
                .finished_at
                .expect("Apparently this build didn't finish");

        let human = chrono_humanize::HumanTime::from(finished_ago);

        Self {
            title: format!(
                "{}: {}/{} {}",
                build.state.to_title_case(),
                build.pipeline.slug,
                build.number,
                build.branch
            ),
            message: format!(
                "Finished {}",
                human.to_text_en(
                    chrono_humanize::Accuracy::Rough,
                    chrono_humanize::Tense::Past
                )
            ),
        }
    }
}

impl NotificationContent {
    #[cfg(feature = "os-notifications")]
    pub async fn send_os_notification(&self) -> Result<(), notifica::Error> {
        let result = notifica::notify(&self.title, &self.message);

        // Clue from https://stackoverflow.com/questions/62753205/threadsleep-is-required-for-my-toast-notification-program-in-rust-winrt
        // suggests some delay is necessary to ensure the notification is displayed on Windows
        // (or maybe some cleanup isn't called before exit?)
        if cfg!(windows) {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }

        result
    }
}

use buildkite_rust::Build;
use notify_rust::Notification;
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
                "{:?}: {}/{} {}",
                build.state, build.pipeline.slug, build.number, build.branch
            ),
            message: format!(
                "Finished {}",
                human.to_text_en(
                    chrono_humanize::Accuracy::Precise,
                    chrono_humanize::Tense::Past
                )
            ),
        }
    }
}

impl Into<Notification> for &NotificationContent {
    fn into(self) -> Notification {
        notify_rust::Notification::new()
            .summary(&self.title)
            .body(&self.message)
            .finalize()
    }
}

impl NotificationContent {
    pub fn show_notification(
        &self,
    ) -> Result<notify_rust::NotificationHandle, notify_rust::error::Error> {
        let notification: Notification = self.into();

        notification.show()
    }
}

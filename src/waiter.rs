use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::time::Duration;

use crate::{error::RequestError, Build, Buildkite};

const MAX_RETRIES: u64 = 3;

pub struct Waiter {
    url: String,
    client: Buildkite,
    tx: SyncSender<WaitStatus>,
}

pub enum WaitStatus {
    Continue(Option<String>, Duration),
    Abort(RequestError),
    Finished(Build),
}

fn failure_delay(attempts: u64) -> Duration {
    Duration::from_secs(attempts.pow(2))
}

impl Waiter {
    pub fn start(client: Buildkite, url: &str) -> Receiver<WaitStatus> {
        let (tx, rx) = sync_channel(0);

        let waiter = Waiter {
            url: url.to_string(),
            client,
            tx,
        };

        std::thread::spawn(|| waiter.wait());

        rx
    }

    fn wait(self) {
        let mut attempts = 0u64;

        loop {
            match self.client.build_by_url(&self.url) {
                Ok(build) => {
                    if build.is_finished() {
                        self.tx.send(WaitStatus::Finished(build)).ok();

                        return;
                    } else {
                        let retry_in = Duration::from_secs(30);

                        if self
                            .tx
                            .send(WaitStatus::Continue(Some(build.state), retry_in))
                            .is_err()
                        {
                            return;
                        }

                        std::thread::sleep(retry_in);
                    }
                }
                Err(e) => {
                    attempts += 1;

                    if attempts == MAX_RETRIES {
                        self.tx.send(WaitStatus::Abort(e)).ok();

                        return;
                    } else {
                        let retry_in = failure_delay(attempts);

                        if self.tx.send(WaitStatus::Continue(None, retry_in)).is_err() {
                            return;
                        }

                        std::thread::sleep(retry_in);
                    }
                }
            }
        }
    }
}

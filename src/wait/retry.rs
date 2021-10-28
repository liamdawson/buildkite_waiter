use buildkite_waiter::{Build, Buildkite};
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

pub async fn attempt_build_by_url(
    client: &Buildkite,
    url: &Url,
    max_attempts: u64,
) -> Result<Build, buildkite_waiter::error::RequestError> {
    let mut current_retry = 0u64;

    let mut last_result = client.build_by_url(url.as_str()).await;

    while current_retry < max_attempts {
        if let Ok(build) = last_result {
            return Ok(build);
        }

        current_retry += 1;

        let delay = current_retry.pow(2);

        debug!("Retrying after {}s", delay);

        sleep(Duration::from_secs(delay)).await;

        last_result = client.build_by_url(url.as_str()).await;
    }

    last_result
}

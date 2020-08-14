use buildkite_rust::{ApiResponse, Buildkite, Build};
use url::Url;
use tokio::time::delay_for;
use std::time::Duration;

pub async fn attempt_build_by_url(client: &Buildkite, url: &Url, max_attempts: u64) -> Result<ApiResponse<Build>, buildkite_rust::ResponseError> {
    let mut current_retry = 0u64;

    let mut last_result = client
        .build().by_url(url.clone())
        .await;

    while current_retry < max_attempts {
        if let Ok(response) = last_result {
            if response.is_success() {
                return Ok(response);
            }
        }

        current_retry += 1;

        let delay = current_retry.pow(2);

        debug!("Retrying after {}s", delay);

        delay_for(Duration::from_secs(delay)).await;

        last_result = client
            .build().by_url(url.clone())
            .await;
    }

    last_result
}

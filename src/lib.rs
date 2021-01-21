mod buildkite;
mod waiter;

use std::time::Duration;

pub use buildkite::*;
pub use waiter::*;

use once_cell::sync::Lazy;

// allow compile-time overrides
pub static PUBLIC_BUILDKITE_API_URL: Lazy<&'static str> =
    Lazy::new(|| option_env!("BUILDKITE_WAITER_API_URL").unwrap_or("https://api.buildkite.com/v2"));

#[derive(Clone)]
pub struct Buildkite {
    agent: ureq::Agent,
    pub api_url: String,
    pub(crate) credentials: Option<BuildkiteCredentials>,
}

impl Buildkite {
    pub fn new(api_url: &str) -> Self {
        let agent = ureq::builder()
            // wait 5s to connect, 10s to read a response
            .timeout_connect(Duration::from_millis(5_000))
            .timeout_read(Duration::from_millis(10_000))
            .build();
        Self {
            agent,
            api_url: api_url.to_string(),
            credentials: None,
        }
    }

    pub fn credentials(&mut self, credentials: BuildkiteCredentials) -> &mut Self {
        self.credentials = Some(credentials);

        self
    }
}

impl Default for Buildkite {
    fn default() -> Self {
        Self::new(*PUBLIC_BUILDKITE_API_URL)
    }
}

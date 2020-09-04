mod credentials;
mod buildkite;
mod http;

pub use credentials::BuildkiteCredentials;
pub use buildkite::*;

use once_cell::sync::Lazy;

// allow compile-time overrides
pub static PUBLIC_BUILDKITE_API_URL: Lazy<&'static str> = Lazy::new(|| option_env!("BUILDKITE_WAITER_API_URL")
    .unwrap_or("https://api.buildkite.com/v2"));

#[derive(Clone)]
pub struct Buildkite {
    pub api_url: String,
    pub(crate) credentials: Option<BuildkiteCredentials>,
}

impl Buildkite {
    pub fn new(api_url: &str) -> Self {
        Self {
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

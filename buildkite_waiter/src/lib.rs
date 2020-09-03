mod credentials;
mod build;
mod http;

pub use credentials::BuildkiteCredentials;

use once_cell::sync::Lazy;

// allow compile-time overrides
pub static PUBLIC_BUILDKITE_API_URL: Lazy<&'static str> = Lazy::new(|| option_env!("BUILDKITE_WAITER_API_URL")
    .unwrap_or("https://api.buildkite.com/v2"));

pub struct Buildkite {
    pub api_url: String,
}

impl Default for Buildkite {
    fn default() -> Self {
        Self {
            api_url: PUBLIC_BUILDKITE_API_URL.to_string(),
        }
    }
}

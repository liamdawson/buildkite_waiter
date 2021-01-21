use super::BuildkiteCredentials;
use crate::Buildkite;
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;

// allow compile-time overrides
pub static USER_AGENT: Lazy<&'static str> =
    Lazy::new(|| option_env!("BUILDKITE_WAITER_USER_AGENT").unwrap_or(DEFAULT_USER_AGENT));

#[cfg(not(debug_assertions))]
pub const DEFAULT_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[cfg(debug_assertions)]
pub const DEFAULT_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    "-dev"
);

impl Buildkite {
    pub(crate) fn request(&self, method: &str, url: &str) -> ureq::Request {
        let mut req = self.agent.request(method, url);

        if let Some(credentials) = &self.credentials {
            match credentials {
                BuildkiteCredentials::ApiAccessToken(token) => {
                    req = req.set(
                        "Authorization",
                        &format!("Bearer {}", token.expose_secret()),
                    )
                }
            };
        }

        req.set("User-Agent", &USER_AGENT)
    }

    pub(crate) fn path_request(&self, method: &str, path: &str) -> ureq::Request {
        let url = format!("{}/{}", self.api_url, path);

        self.request(method, &url)
    }
}

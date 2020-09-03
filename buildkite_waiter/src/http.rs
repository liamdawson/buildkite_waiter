use reqwest::{Method, RequestBuilder, Client};
use super::{Buildkite, BuildkiteCredentials};
use std::time::Duration;
use secrecy::ExposeSecret;
use once_cell::sync::Lazy;

// allow compile-time overrides
pub static USER_AGENT: Lazy<&'static str> = Lazy::new(|| option_env!("BUILDKITE_WAITER_USER_AGENT")
    .unwrap_or(DEFAULT_USER_AGENT));

#[cfg(not(debug_assertions))]
pub const DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[cfg(debug_assertions)]
pub const DEFAULT_USER_AGENT: &str = concat!( env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"), "-dev");

fn build_client() -> reqwest::Result<Client> {
    Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(15))
        .user_agent(*USER_AGENT)
        .build()
}

impl Buildkite {
    pub(crate) fn request(&self, method: Method, url: &str, credentials: BuildkiteCredentials) -> reqwest::Result<RequestBuilder> {
        let client = build_client()?;

        let mut builder = client.request(method, url);

        builder = match credentials {
            BuildkiteCredentials::ApiAccessToken(token) => builder.bearer_auth(&token.expose_secret()),
        };

        Ok(builder)
    }

    pub(crate) fn path_request(&self, method: Method, path: &str, credentials: BuildkiteCredentials) -> reqwest::Result<RequestBuilder> {
        let url = format!("{}/{}", self.api_url, path);

        self.request(method, &url, credentials)
    }
}

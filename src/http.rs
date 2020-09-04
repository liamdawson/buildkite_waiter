use super::{error::RequestError, Buildkite, BuildkiteCredentials};
use once_cell::sync::Lazy;
use reqwest::{Client, Method, RequestBuilder};
use secrecy::ExposeSecret;
use std::time::Duration;

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

fn build_client() -> reqwest::Result<Client> {
    Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(15))
        .user_agent(*USER_AGENT)
        .build()
}

impl Buildkite {
    pub(crate) fn request(
        &self,
        method: Method,
        url: &str,
    ) -> Result<RequestBuilder, RequestError> {
        let client = build_client()?;

        let credentials = if let Some(credentials) = &self.credentials {
            credentials
        } else {
            return Err(RequestError::CredentialsRequired);
        };

        let mut builder = client.request(method, url);

        builder = match credentials {
            BuildkiteCredentials::ApiAccessToken(token) => {
                builder.bearer_auth(&token.expose_secret())
            }
        };

        Ok(builder)
    }

    pub(crate) fn path_request(
        &self,
        method: Method,
        path: &str,
    ) -> Result<RequestBuilder, RequestError> {
        let url = format!("{}/{}", self.api_url, path);

        self.request(method, &url)
    }
}

use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder, Url};
use secrecy::{ExposeSecret, SecretString};
use std::time::Duration;

pub static PUBLIC_BUILDKITE_API_URL: Lazy<Url> = Lazy::new(|| {
    Url::parse("https://api.buildkite.com/v2/")
        .expect("Failed to parse the Buildkite API base URL constant")
});

#[cfg(debug_assertions)]
pub const DEFAULT_LIB_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    "-dev"
);

#[cfg(not(debug_assertions))]
pub const DEFAULT_LIB_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone)]
pub struct Buildkite {
    pub(crate) api_token: SecretString,
    pub(crate) api_url: Url,
    pub(crate) client: Client,
}

fn http_client<S: AsRef<str>>(user_agent: S) -> Client {
    ClientBuilder::new()
        .user_agent(user_agent.as_ref())
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to create the base HTTP client")
}

impl Buildkite {
    pub fn authenticated<S: Into<String>>(token: S) -> Self {
        Self {
            api_token: SecretString::new(token.into()),
            api_url: PUBLIC_BUILDKITE_API_URL.clone(),
            client: http_client(DEFAULT_LIB_USER_AGENT),
        }
    }

    pub fn api_url<U: Into<Url>>(mut self, new_base: U) -> Self {
        self.api_url = new_base.into();

        self
    }

    pub(crate) fn request_by_url(
        self,
        method: reqwest::Method,
        url: Url,
    ) -> reqwest::RequestBuilder {
        self.client
            .request(method, url)
            .bearer_auth(self.api_token.expose_secret())
    }

    pub(crate) fn request<I>(
        self,
        method: reqwest::Method,
        path_segments: I,
    ) -> reqwest::RequestBuilder
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let mut url = self.api_url.clone();

        url.path_segments_mut()
            .expect("path_segments_mut failed on Buildkite.api_url")
            .extend(path_segments);

        self.request_by_url(method, url)
    }
}

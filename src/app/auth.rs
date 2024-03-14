use anyhow::Context;
use buildkite_waiter::{Buildkite, BuildkiteCredentials};
use keyring::Entry;
use secrecy::SecretString;

pub fn keyring_entry() -> anyhow::Result<Entry> {
    let service_username = match buildkite_waiter::PUBLIC_BUILDKITE_API_URL {
        "https://api.buildkite.com/v2" => "https://api.buildkite.com/v2/",
        other => other,
    };

    Entry::new(crate::APP_ID, service_username).context("failed to find keyring entry")
}

pub fn fetch_credentials() -> anyhow::Result<BuildkiteCredentials> {
    let token = keyring_entry()?
        .get_password()
        .context("failed to load access token")?;

    Ok(BuildkiteCredentials::ApiAccessToken(SecretString::new(
        token,
    )))
}

pub fn client() -> anyhow::Result<Buildkite> {
    let mut client = Buildkite::default();
    client.credentials(fetch_credentials()?);

    Ok(client)
}

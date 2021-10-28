use buildkite_waiter::{Buildkite, BuildkiteCredentials};
use keyring::Keyring;
use secrecy::SecretString;

pub fn keyring_entry() -> Keyring<'static> {
    Keyring::new(crate::APP_ID, "https://api.buildkite.com/v2/")
}

pub fn fetch_credentials() -> anyhow::Result<BuildkiteCredentials> {
    let token = keyring_entry().get_password()?;

    Ok(BuildkiteCredentials::ApiAccessToken(SecretString::new(
        token,
    )))
}

pub fn client() -> anyhow::Result<Buildkite> {
    let mut client = Buildkite::default();
    client.credentials(fetch_credentials()?);

    Ok(client)
}

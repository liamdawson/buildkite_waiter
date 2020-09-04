use anyhow::Context;
// use buildkite_rust::Buildkite;
use keyring::Keyring;
use buildkite_waiter::BuildkiteCredentials;
use secrecy::SecretString;

pub fn keyring_entry() -> Keyring<'static> {
    Keyring::new(crate::APP_ID, "https://api.buildkite.com/v2/")
}

// Currently, keyring uses dbus 0.2.3, which doesn't impl Sync on the error type
// This serialization of the error allows context to work, hopefully without
// losing too much context
pub fn serialize_error(e: impl std::error::Error) -> anyhow::Error {
    anyhow::anyhow!("{}", e)
}

// pub fn client() -> anyhow::Result<Buildkite> {
//     let access_token = keyring_entry()
//         .get_password()
//         .map_err(serialize_error)
//         .context("Unable to retrieve a saved API token")?;
//
//     Ok(Buildkite::authenticated(access_token))
// }

pub fn fetch_credentials() -> anyhow::Result<BuildkiteCredentials> {
    let token = keyring_entry()
        .get_password()?;

    Ok(BuildkiteCredentials::ApiAccessToken(SecretString::new(token)))
}

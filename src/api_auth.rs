use buildkite_waiter::BuildkiteCredentials;
use keyring::Keyring;
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

pub fn fetch_credentials() -> anyhow::Result<BuildkiteCredentials> {
    let token = keyring_entry().get_password()?;

    Ok(BuildkiteCredentials::ApiAccessToken(SecretString::new(
        token,
    )))
}

use anyhow::Context;
use buildkite_rust::Buildkite;
use keyring::Keyring;

pub fn keyring_entry() -> Keyring<'static> {
    Keyring::new(crate::APP_ID, "https://api.buildkite.com/v2/")
}

pub fn client() -> anyhow::Result<Buildkite> {
    let access_token = keyring_entry()
        .get_password()
        .context("Unable to retrieve a saved API token")?;

    Ok(Buildkite::authenticated(access_token))
}

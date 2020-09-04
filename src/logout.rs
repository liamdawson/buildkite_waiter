use crate::api_auth;
use anyhow::Context;

pub fn logout() -> anyhow::Result<i32> {
    api_auth::keyring_entry()
        .delete_password()
        .map_err(api_auth::serialize_error)
        .context("Failed to delete saved API token")?;

    println!("{}", console::style("OK").green());

    Ok(0)
}

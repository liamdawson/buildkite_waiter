use crate::api_auth;
use anyhow::Context;

pub fn logout() -> anyhow::Result<i32> {
    api_auth::keyring_entry()
        .delete_password()
        .context("Failed to delete saved API token")?;

    println!("{}", console::style("OK").green());

    Ok(0)
}

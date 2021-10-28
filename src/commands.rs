use crate::app::auth;
use anyhow::Context;

pub fn login() -> anyhow::Result<i32> {
    eprintln!("Generate an API Access Token at https://buildkite.com/user/api-access-tokens/new");
    eprintln!(
        r#"Ensure you enable all relevant organizations, and enable the "Read Builds" (and optionally "Read User") permissions."#
    );

    let access_token = dialoguer::Password::new()
        .with_prompt("Buildkite API Access Token")
        .interact()?;

    auth::keyring_entry()
        .set_password(&access_token)
        .context("Failed to save API token")?;

    println!("{}", console::style("OK").green());

    Ok(0)
}

pub fn logout() -> anyhow::Result<i32> {
    auth::keyring_entry()
        .delete_password()
        .context("Failed to delete saved API token")?;

    println!("{}", console::style("OK").green());

    Ok(0)
}

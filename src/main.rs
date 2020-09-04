#[macro_use]
extern crate log;

use cli::Command;
use structopt::StructOpt;
use anyhow::Context;

mod api_auth;
mod app;
mod cli;
mod output;
mod wait;

// commands
mod login;
mod logout;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const DEVELOPER_IDENTIFIER: &str = "com.ldaws";
pub const APP_ID: &str = concat!("com.ldaws.", env!("CARGO_PKG_NAME"));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    configure_logger()?;

    let exit_code = match Command::from_args() {
        Command::ByNumber {
            output,
            runtime,
            strategy,
        } => {
            let client = app::auth::client()?;

            let build = client
                .build_by_number(
                    &strategy.organization,
                    &strategy.pipeline,
                    &format!("{}", strategy.number),
                )
                .await?;

            wait::by_url(&build.url, runtime, output).await
        }
        Command::ByUrl {
            output,
            runtime,
            strategy,
        } => {
            let client = app::auth::client()?;

            let (organization, pipeline, number) =
                buildkite_waiter::url::build_number(strategy.url.as_str())?;

            let build = client
                .build_by_number(&organization, &pipeline, &number)
                .await?;

            wait::by_url(&build.url, runtime, output).await
        }
        Command::Latest {
            output,
            runtime,
            strategy,
        } => {
            let client = app::auth::client()?;

            let scope = if let Some(organization) = strategy.organization {
                if let Some(pipeline) = strategy.pipeline {
                    buildkite_waiter::BuildScope::Pipeline(organization, pipeline)
                } else {
                    buildkite_waiter::BuildScope::Organization(organization)
                }
            } else {
                buildkite_waiter::BuildScope::All
            };

            let creator = if let Some(creator) = strategy.creator {
                Some(creator)
            } else if strategy.mine {
                Some(client.get_access_token_holder().await.context("Unable to determine the current user (the API Access Token may need the \"Read User\" permission)")?.id)
            } else {
                None
            };

            let build = client
                .latest_build(
                    scope,
                    strategy.branch.iter().map(|x| &**x).collect::<Vec<_>>().as_slice(),
                    creator.iter().map(|x| &**x).collect::<Vec<_>>().pop(),
                    strategy.commit.iter().map(|x| &**x).collect::<Vec<_>>().pop(),
                    strategy.state.iter().map(|x| &**x).collect::<Vec<_>>().as_slice()
                )
                .await?; //  &strategy.organization, &strategy.pipeline, &format!("{}", strategy.number)).await?;

            if let Some(build) = build {
                wait::by_url(&build.url, runtime, output).await
            } else {
                Err(anyhow::anyhow!("No matching builds were found"))
            }
        }
        Command::Login => login::login(),
        Command::Logout => logout::logout(),
    }?;

    std::process::exit(exit_code);
}

fn configure_logger() -> Result<(), anyhow::Error> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%dT%H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stderr())
        .apply()?;

    Ok(())
}

#[macro_use]
extern crate log;

use std::time::{Duration, Instant};

use anyhow::Context;
use buildkite_waiter::{Build, WaitStatus, Waiter};
use cli::Command;
use console::style;
use heck::TitleCase;
use structopt::StructOpt;

mod app;
mod cli;
mod output;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const DEVELOPER_IDENTIFIER: &str = "com.ldaws";
pub const APP_ID: &str = concat!("com.ldaws.", env!("CARGO_PKG_NAME"));

fn main() -> anyhow::Result<()> {
    configure_logger()?;

    let exit_code = match Command::from_args() {
        Command::ByNumber {
            output,
            runtime,
            strategy,
        } => {
            let client = app::auth::client()?;

            let build = client.build_by_number(
                &strategy.organization,
                &strategy.pipeline,
                &format!("{}", strategy.number),
            )?;

            wait(client, build, runtime, output)
        }
        Command::ByUrl {
            output,
            runtime,
            strategy,
        } => {
            let client = app::auth::client()?;

            let (organization, pipeline, number) =
                buildkite_waiter::url::build_number(strategy.url.as_str())?;

            let build = client.build_by_number(&organization, &pipeline, &number)?;

            wait(client, build, runtime, output)
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
                let id = client.get_access_token_holder()
                    .context("Unable to determine the current user (the API Access Token may need the \"Read User\" permission)")?
                    .id;

                Some(id)
            } else {
                None
            };

            let build = client.latest_build(
                scope,
                strategy
                    .branch
                    .iter()
                    .map(|x| &**x)
                    .collect::<Vec<_>>()
                    .as_slice(),
                creator.iter().map(|x| &**x).collect::<Vec<_>>().pop(),
                strategy
                    .commit
                    .iter()
                    .map(|x| &**x)
                    .collect::<Vec<_>>()
                    .pop(),
                strategy
                    .state
                    .iter()
                    .map(|x| &**x)
                    .collect::<Vec<_>>()
                    .as_slice(),
            )?;

            if let Some(build) = build {
                wait(client, build, runtime, output)
            } else {
                Err(anyhow::anyhow!("No matching builds were found"))
            }
        }
        Command::Login => app::commands::login(),
        Command::Logout => app::commands::logout(),
    }?;

    std::process::exit(exit_code);
}

fn wait(
    client: buildkite_waiter::Buildkite,
    seed_build: Build,
    runtime: cli::RuntimeArgs,
    output: cli::OutputArgs,
) -> anyhow::Result<i32> {
    output.found_build(&seed_build);

    let start_time = Instant::now();
    let timeout = Duration::from_secs(runtime.timeout);

    let rx = Waiter::start(client, &seed_build.url);

    let mut build = seed_build;

    while !build.is_finished() {
        let timeout = timeout - start_time.elapsed();

        let status = rx.recv_timeout(timeout)?;
        match status {
            WaitStatus::Abort(err) => return Err(err.into()),
            WaitStatus::Finished(finished_build) => build = finished_build,
            WaitStatus::Continue(state, retry_in) => match state {
                Some(state) => info!(
                    "Build status: {}. Retrying in {}s",
                    state.to_title_case(),
                    retry_in.as_secs()
                ),
                None => warn!(
                    "Unable to get build status, retrying in {}s",
                    retry_in.as_secs()
                ),
            },
        }
    }

    Ok(output.on_completion(&build))
}

fn configure_logger() -> Result<(), anyhow::Error> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {}",
                style(format!(
                    "[{}] {}",
                    record.level(),
                    chrono::Local::now().format("%H:%M:%S")
                ))
                .dim(),
                message
            ))
        })
        .level_for("ureq", log::LevelFilter::Warn)
        .level(log::LevelFilter::Info)
        .chain(std::io::stderr())
        .apply()?;

    Ok(())
}

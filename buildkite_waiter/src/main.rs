#[macro_use]
extern crate log;

use cli::Command;
use structopt::StructOpt;

mod api_auth;
mod cli;
mod find;
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
        Command::Wait { raw_parameters: _ } => {
            println!("Wait has been replaced with by-url and by-number.");
            println!("Try `buildkite_waiter help` to see available commands.");

            Ok(1)
        }
        Command::ByNumber {
            output,
            runtime,
            strategy,
        } => {
            wait::for_build(
                |client| async move { strategy.find_build(&client).await },
                runtime,
                output,
            )
            .await
        }
        Command::ByUrl {
            output,
            runtime,
            strategy,
        } => {
            wait::for_build(
                |client| async move { strategy.find_build(&client).await },
                runtime,
                output,
            )
            .await
        }
        Command::Latest {
            output,
            runtime,
            strategy,
        } => {
            wait::for_build(
                |client| async move { strategy.find_build(&client).await },
                runtime,
                output,
            )
            .await
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

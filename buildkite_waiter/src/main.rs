#[macro_use] extern crate log;

use structopt::StructOpt;
use cli::Command;
use buildkite_rust::BuildState;

mod api_auth;
mod cli;

// commands
mod login;
mod logout;
mod wait;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const DEVELOPER_IDENTIFIER: &str = "com.ldaws";
pub const APP_ID: &str = concat!("com.ldaws.", env!("CARGO_PKG_NAME"));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    configure_logger()?;

    let exit_code = match Command::from_args() {
        Command::ByNumber => {
            todo!();
        },
        Command::ByUrl => {
            todo!();
        },
        Command::Latest => {
            todo!();
        },
        Command::Wait { raw_parameters } => {
            println!("Wait has been replaced by more specific subcommands.");

            if raw_parameters.iter().any(|p| p == "--url") {
                println!("You may want to try by-url instead.");
            } else if raw_parameters.iter().any(|p| p == "--number") {
                println!("You may want to try by-number instead.");
            } else {
                println!("Try `buildkite_waiter help` to see available commands.");
            }

            Ok(1)
        },
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

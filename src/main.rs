#[macro_use]
extern crate log;

use cli::Command;
use structopt::StructOpt;

mod app;
mod cli;
mod commands;
mod output;
mod wait;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const DEVELOPER_IDENTIFIER: &str = "com.ldaws";
pub const APP_ID: &str = concat!("com.ldaws.", env!("CARGO_PKG_NAME"));

fn main() -> anyhow::Result<()> {
    configure_logger()?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let exit_code = runtime.block_on(inner_main())?;

    std::process::exit(exit_code);
}

async fn inner_main() -> anyhow::Result<i32> {
    match Command::from_args() {
        Command::ByNumber {
            output,
            runtime,
            strategy,
        } => app::wait_by::number(output, runtime, strategy).await,
        Command::ByUrl {
            output,
            runtime,
            strategy,
        } => app::wait_by::url(output, runtime, strategy).await,
        Command::Latest {
            output,
            runtime,
            strategy,
        } => app::wait_by::latest(output, runtime, strategy).await,
        Command::Login => commands::login(),
        Command::Logout => commands::logout(),
    }
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

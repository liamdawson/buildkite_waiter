#[macro_use]
extern crate log;

use clap::{crate_name, Parser};
use cli::{Cli, Commands};

mod app;
mod cli;
mod commands;
mod output;
mod wait;

pub const APP_NAME: &str = crate_name!();
pub const DEVELOPER_IDENTIFIER: &str = "com.ldaws";
pub const APP_ID: &str = concat!("com.ldaws.", crate_name!());

fn main() -> anyhow::Result<()> {
    configure_logger()?;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let exit_code = runtime.block_on(inner_main())?;

    std::process::exit(exit_code);
}

async fn inner_main() -> anyhow::Result<i32> {
    let args = Cli::parse();

    match args.command {
        Commands::ByNumber {
            output,
            runtime,
            strategy,
        } => app::wait_by::number(output, runtime, strategy).await,
        Commands::ByUrl {
            output,
            runtime,
            strategy,
        } => app::wait_by::url(output, runtime, strategy).await,
        Commands::Latest {
            output,
            runtime,
            strategy,
        } => app::wait_by::latest(output, runtime, strategy).await,
        Commands::Login => commands::login(),
        Commands::Logout => commands::logout(),
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

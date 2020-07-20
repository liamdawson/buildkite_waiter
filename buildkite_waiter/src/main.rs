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
        Command::Wait(args) => {
            let should_notify = args.notification;
            let error_exit_strategy = args.error_on;
            let output_notification_json = args.output_notification_json;

            match wait::wait(api_auth::client()?, &args).await {
                Ok((state, notification_content)) => {
                    let notification: notify_rust::Notification = (&notification_content).into();

                    if should_notify {
                        if let Err(e) = notification.show() {
                            log::warn!("Unable to display notification: {}", e);
                        }
                    }

                    if output_notification_json {
                        println!("{}", serde_json::to_string(&notification_content)?);
                    }

                    match error_exit_strategy {
                        cli::ExitStatusStrategy::BuildFailedOrCanceled => {
                            match state {
                                BuildState::Failed | BuildState::Canceled => Ok(2),
                                _ => Ok(0),
                            }
                        },
                        _ => Ok(0)
                    }
                },
                Err(e) => {
                    if should_notify {
                        notify_rust::Notification::new()
                            .summary("Wait failed")
                            .body(&format!("{}", e))
                            .show()
                            .ok(); // don't check if the notification could be sent
                    }

                    Err(e)
                },
            }
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

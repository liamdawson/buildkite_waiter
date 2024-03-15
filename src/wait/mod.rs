mod retry;

use anyhow::Context;
use buildkite_waiter::{Build, Buildkite};
use heck::ToTitleCase;
use indicatif::ProgressStyle;
use std::{future::Future, time::Duration};
use tokio::time::sleep;
use url::Url;

pub async fn by_url(
    url: &str,
    runtime_args: crate::cli::RuntimeArgs,
    output: crate::cli::OutputArgs,
) -> anyhow::Result<i32> {
    for_build(
        |client| async move {
            let resp = client
                .build_by_url(url)
                .await
                .context("Failed to retrieve build")?;

            Ok(resp)
        },
        runtime_args,
        output,
    )
    .await
}

pub async fn for_build<F, Fut>(
    build_fn: F,
    runtime_args: crate::cli::RuntimeArgs,
    output: crate::cli::OutputArgs,
) -> anyhow::Result<i32>
where
    F: FnOnce(Buildkite) -> Fut,
    Fut: Future<Output = anyhow::Result<Build>>,
{
    if output.notification {
        warn!("--notification is deprecated: os notifications are now sent by default");
    }

    let client = crate::app::auth::client().context("Unable to prepare a Buildkite client")?;

    let mut build = build_fn(client.clone())
        .await
        .context("Unable to find details for a matching build")?;

    output.found_build(&build);

    let timeout_duration = Duration::from_secs(u64::from(runtime_args.timeout));
    debug!(
        "Waiting a maximum of {:?} for build completion",
        timeout_duration
    );

    let mut checked_at = chrono::Local::now();
    let mut last_state = build.state.clone();
    let mut check_count = 1usize;

    let m = indicatif::MultiProgress::new();

    let check_spinner = m.add(
        indicatif::ProgressBar::new_spinner()
            .with_style(ProgressStyle::with_template("{spinner} {elapsed:<8.dim} {msg}").unwrap()),
    );

    check_spinner.enable_steady_tick(Duration::from_millis(100));

    while !build.is_finished() {
        let poll_pause = Duration::from_secs(30);

        debug!("Waiting {:?}s to poll build", poll_pause);

        check_spinner.set_message(spinner_message(&last_state, &checked_at, check_count));

        sleep(poll_pause).await;

        debug!("Checking build status");

        let build_url = Url::parse(&build.url)?;

        let checking_at = chrono::Local::now();

        if let Ok(get_result) = tokio::time::timeout(
            timeout_duration,
            retry::attempt_build_by_url(&client, &build_url, 3),
        )
        .await
        {
            build = get_result.context("Unable to retrieve build details")?;
            checked_at = checking_at;
            check_count += 1;

            if last_state != build.state {
                m.println(format!(
                    "[{}] Build changed state from {} to {}",
                    checking_at.format("%X"),
                    last_state.to_title_case().to_ascii_lowercase(),
                    build.state.to_title_case().to_ascii_lowercase()
                ))
                .ok();

                last_state = build.state.clone();
            }
        } else {
            check_spinner.abandon();
            anyhow::bail!("Timed out waiting for build details");
        }
    }

    check_spinner.finish_and_clear();

    Ok(output.on_completion(&build).await)
}

fn spinner_message(
    state: &str,
    checked_at: &chrono::DateTime<chrono::Local>,
    check_count: usize,
) -> String {
    let prefix = console::style("Build is").dim();
    let state = console::style(state.to_title_case().to_ascii_lowercase()).bold();
    let as_of = console::style("as of").dim();
    let checked = console::style(checked_at.format("%X"));
    let suffix = console::style(format!("(checked {} time(s))", check_count)).dim();

    format!("{} {} {} {} {}", prefix, state, as_of, checked, suffix)
}

// TODO: handle rate limiting
// headers:
// "rate-limit-remaining": "99",
// "rate-limit-limit": "100",
// "rate-limit-reset": "58",

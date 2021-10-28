use anyhow::Context;

use crate::{
    app,
    cli::{ByNumberStrategyArgs, ByUrlStrategyArgs, LatestStrategyArgs, OutputArgs, RuntimeArgs},
    wait,
};

pub async fn number(
    output: OutputArgs,
    runtime: RuntimeArgs,
    strategy: ByNumberStrategyArgs,
) -> anyhow::Result<i32> {
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

pub async fn url(
    output: OutputArgs,
    runtime: RuntimeArgs,
    strategy: ByUrlStrategyArgs,
) -> anyhow::Result<i32> {
    let client = app::auth::client()?;

    let (organization, pipeline, number) =
        buildkite_waiter::url::build_number(strategy.url.as_str())?;

    let build = client
        .build_by_number(&organization, &pipeline, &number)
        .await?;

    wait::by_url(&build.url, runtime, output).await
}

pub async fn latest(
    output: OutputArgs,
    runtime: RuntimeArgs,
    strategy: LatestStrategyArgs,
) -> anyhow::Result<i32> {
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
            .await
            .context("Unable to determine the current user (the API Access Token may need the \"Read User\" permission)")?
            .id;

        Some(id)
    } else {
        None
    };

    let branches: Vec<&str> = strategy.branch.iter().map(|s| s.as_str()).collect();
    let states: Vec<&str> = strategy.state.iter().map(|s| s.as_str()).collect();

    let build = client
        .latest_build(
            scope,
            branches.as_slice(),
            creator.as_deref(),
            strategy.commit.clone().as_deref(),
            states.as_slice(),
        )
        .await?;
    if let Some(build) = build {
        wait::by_url(&build.url, runtime, output).await
    } else {
        Err(anyhow::anyhow!("No matching builds were found"))
    }
}

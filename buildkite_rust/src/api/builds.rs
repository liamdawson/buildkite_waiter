use crate::{ApiResponse, Build};
use reqwest::Method;
use std::fmt::Display;

pub struct BuildsApi {
    buildkite: crate::Buildkite,
}

enum ListScope {
    All,
    Organization(String),
    Pipeline(String, String),
}

pub struct ListRequestBuilder {
    buildkite: crate::Buildkite,
    scope: ListScope,
}

impl crate::Buildkite {
    pub fn builds(&self) -> BuildsApi {
        BuildsApi {
            buildkite: self.clone(),
        }
    }
}

impl BuildsApi {
    pub fn all(self) -> ListRequestBuilder {
        ListRequestBuilder {
            buildkite: self.buildkite,
            scope: ListScope::All,
        }
    }

    pub fn organization<O>(self, organization: O) -> ListRequestBuilder where O: Display {
        ListRequestBuilder {
            buildkite: self.buildkite,
            scope: ListScope::Organization(format!("{}", organization)),
        }
    }

    pub fn pipeline<O, P>(self, organization: O, pipeline: P) -> ListRequestBuilder where O: Display, P: Display {
        ListRequestBuilder {
            buildkite: self.buildkite,
            scope: ListScope::Pipeline(format!("{}", organization), format!("{}", pipeline)),
        }
    }
}

impl ListRequestBuilder {
    pub async fn get(self) -> Result<ApiResponse<Vec<Build>>, reqwest::Error> {
        let req = match self.scope {
            ListScope::All => self.buildkite.request(Method::GET, &["builds"]),
            ListScope::Organization(org) => self.buildkite.request(Method::GET, &["organizations", &org, "builds"]),
            ListScope::Pipeline(org, pipeline) => self.buildkite.request(Method::GET, &["organizations", &org, "pipelines", &pipeline, "builds"]),
        };

        let resp = req.send().await?;

        Ok(ApiResponse::from_reqwest(resp).await?)
    }
}

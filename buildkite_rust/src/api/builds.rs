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
    pub per_page: u16,
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
        ListRequestBuilder::scoped(self.buildkite, ListScope::All)
    }

    pub fn organization<O>(self, organization: O) -> ListRequestBuilder where O: Display {
        ListRequestBuilder::scoped(self.buildkite, ListScope::Organization(format!("{}", organization)))
    }

    pub fn pipeline<O, P>(self, organization: O, pipeline: P) -> ListRequestBuilder where O: Display, P: Display {
        ListRequestBuilder::scoped(self.buildkite,  ListScope::Pipeline(format!("{}", organization), format!("{}", pipeline)))
    }
}

impl ListRequestBuilder {
    fn scoped(buildkite: crate::Buildkite, scope: ListScope) -> Self {
        ListRequestBuilder {
            per_page: 30,
            buildkite,
            scope,
        }
    }

    pub async fn get(self) -> Result<ApiResponse<Vec<Build>>, reqwest::Error> {
        let mut req = match self.scope {
            ListScope::All => self.buildkite.request(Method::GET, &["builds"]),
            ListScope::Organization(org) => self.buildkite.request(Method::GET, &["organizations", &org, "builds"]),
            ListScope::Pipeline(org, pipeline) => self.buildkite.request(Method::GET, &["organizations", &org, "pipelines", &pipeline, "builds"]),
        };

        req = req.query(&[("per_page", format!("{}", self.per_page))]);

        let resp = req.send().await?;

        Ok(ApiResponse::from_reqwest(resp).await?)
    }

    pub fn per_page(mut self, per_page: u16) -> Self {
        self.per_page = per_page;

        self
    }
}

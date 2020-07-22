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
    per_page: u16,
    branches: Vec<String>,
    creator: Option<String>,
    commit: Option<String>,
    states: Vec<String>,
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
            buildkite,
            scope,
            branches: Vec::default(),
            commit: None,
            creator: None,
            per_page: 30,
            states: Vec::default(),
        }
    }

    pub async fn get(self) -> Result<ApiResponse<Vec<Build>>, reqwest::Error> {
        let mut req = match self.scope {
            ListScope::All => self.buildkite.request(Method::GET, &["builds"]),
            ListScope::Organization(org) => self.buildkite.request(Method::GET, &["organizations", &org, "builds"]),
            ListScope::Pipeline(org, pipeline) => self.buildkite.request(Method::GET, &["organizations", &org, "pipelines", &pipeline, "builds"]),
        };

        req = req.query(&[("per_page", format!("{}", self.per_page))]);

        if !self.branches.is_empty() {
            req = req.query(self.branches.into_iter()
                .map(|branch| ("branch[]", branch))
                .collect::<Vec<_>>()
                .as_slice());
        }

        if !self.states.is_empty() {
            req = req.query(self.states.into_iter()
                .map(|state| ("state[]", state))
                .collect::<Vec<_>>()
                .as_slice());
        }

        if let Some(creator) = self.creator {
            req = req.query(&[("creator", &creator)]);
        }

        if let Some(commit) = self.commit {
            req = req.query(&[("commit", &commit)]);
        }

        let resp = req.send().await?;

        Ok(ApiResponse::from_reqwest(resp).await?)
    }

    pub fn per_page(mut self, per_page: u16) -> Self {
        self.per_page = per_page;

        self
    }

    pub fn branch<B: AsRef<str>>(mut self, branch: B) -> Self {
        self.branches.push(branch.as_ref().to_string());

        self
    }

    pub fn branches<B>(mut self, branches: B) -> Self where B: IntoIterator, B::Item: AsRef<str> {
        for branch in branches {
            self.branches.push(branch.as_ref().to_string());
        }

        self
    }

    pub fn creator<C: AsRef<str>>(mut self, creator: C) -> Self {
        self.creator = Some(creator.as_ref().to_string());

        self
    }

    pub fn commit<C: AsRef<str>>(mut self, commit: C) -> Self {
        self.commit = Some(commit.as_ref().to_string());

        self
    }

    // TODO: type strictness on states?
    pub fn state<S: AsRef<str>>(mut self, state: S) -> Self {
        self.states.push(state.as_ref().to_string());

        self
    }

    pub fn states<I>(mut self, states: I) -> Self where I: IntoIterator, I::Item: AsRef<str> {
        for state in states {
            self.states.push(state.as_ref().to_string());
        }

        self
    }
}

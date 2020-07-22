use crate::{ApiResponse, Build};
use reqwest::{Url, Method};

pub struct BuildApi {
    buildkite: crate::Buildkite,
}

impl crate::Buildkite {
    pub fn build(&self) -> BuildApi {
        BuildApi {
            buildkite: self.clone(),
        }
    }
}

impl BuildApi {
    pub async fn get<O, P>(self, organization: O, pipeline: P, number: u32) -> Result<ApiResponse<Build>, reqwest::Error> where O: AsRef<str>, P: AsRef<str> {
        let resp = self.buildkite
            .request(Method::GET, &["organizations", &format!("{}", organization.as_ref()), "pipelines", &format!("{}", pipeline.as_ref()), "builds", &format!("{}", number)])
            .send().await?;

        Ok(ApiResponse::from_reqwest(resp).await?)
    }

    pub async fn by_url<U: Into<Url>>(self, url: U) -> Result<ApiResponse<Build>, reqwest::Error> {
        let resp = self.buildkite
            .request_by_url(Method::GET, url.into())
            .send().await?;

        Ok(ApiResponse::from_reqwest(resp).await?)
    }
}

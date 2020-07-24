use crate::{ApiResponse, User};
use reqwest::Method;

pub struct UserApi {
    buildkite: crate::Buildkite,
}

impl crate::Buildkite {
    pub fn user(&self) -> UserApi {
        UserApi {
            buildkite: self.clone(),
        }
    }
}

impl UserApi {
    pub async fn get_access_token_holder(self) -> Result<ApiResponse<User>, reqwest::Error> {
        let resp = self
            .buildkite
            .request(Method::GET, &["user"])
            .send()
            .await?;

        Ok(ApiResponse::from_reqwest(resp).await?)
    }
}

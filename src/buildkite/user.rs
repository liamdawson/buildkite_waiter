use crate::Buildkite;
use reqwest::Method;
use super::error::RequestError;
use chrono::{Utc, DateTime};

#[derive(serde::Deserialize)]
pub struct ApiUser {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Buildkite {
    pub async fn get_access_token_holder(&self) -> Result<ApiUser, RequestError> {
        Ok(self
            .path_request(Method::GET, "user")?
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

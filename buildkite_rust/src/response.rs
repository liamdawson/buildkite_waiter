use serde::de::DeserializeOwned;
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct ApiResponse<TExpected: DeserializeOwned> {
    // TODO: reduce implementation exposure?
    pub headers: reqwest::header::HeaderMap,
    pub status: reqwest::StatusCode,
    error_for_status: Option<reqwest::Error>,

    raw_body: bytes::Bytes,
    body: OnceCell<TExpected>,
}

impl<TExpected: DeserializeOwned> ApiResponse<TExpected> {
    pub(crate) async fn from_reqwest(response: reqwest::Response) -> Result<Self, reqwest::Error> {
        let error_for_status = response.error_for_status_ref().err();

        Ok(Self {
            headers: response.headers().clone(),
            status: response.status().clone(),
            raw_body: response.bytes().await?,
            body: OnceCell::default(),

            error_for_status,
        })
    }

    pub fn error_for_status(self) -> Result<Self, reqwest::Error> {
        match self.error_for_status {
            Some(e) => Err(e),
            _ => Ok(self),
        }
    }

    pub fn error_for_status_ref(&self) -> Result<&Self, &reqwest::Error> {
        match self.error_for_status.as_ref() {
            Some(e) => Err(e),
            _ => Ok(&self),
        }
    }

    pub fn body(&self) -> Result<&TExpected, serde_json::Error> {
        self.body.get_or_try_init(|| serde_json::from_slice(&self.raw_body))
    }
}

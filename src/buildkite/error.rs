#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error("this call requires credentials")]
    CredentialsRequired,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error)
}

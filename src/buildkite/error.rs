#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error("this call requires credentials")]
    CredentialsRequired,
    // TODO: better handling
    #[error(transparent)]
    UreqError(#[from] ureq::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

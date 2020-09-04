use secrecy::SecretString;

#[derive(Clone)]
pub enum BuildkiteCredentials {
    ApiAccessToken(SecretString),
}

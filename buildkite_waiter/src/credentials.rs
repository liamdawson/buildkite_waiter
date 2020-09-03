use secrecy::SecretString;

pub enum BuildkiteCredentials {
    ApiAccessToken(SecretString),
}

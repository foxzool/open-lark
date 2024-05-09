use thiserror::Error;

#[derive(Error, Debug)]
pub enum LarkAPIError {
    #[error("Authorization failed: {0}")]
    NoAuthorization(String),
    #[error("Failed to obtain access token: {0}, code: {1}, msg: {2}")]
    ObtainAccessTokenException(String, i32, String),
    #[error("Deserialize error: {0}")]
    DeserializeError(#[from] serde_json::Error),
}

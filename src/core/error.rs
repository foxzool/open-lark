use crate::core::api_resp::CodeMsg;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LarkAPIError {
    #[error("IO error: {0}")]
    IOErr(#[from] std::io::Error),
    #[error("{0}")]
    IllegalParamError(String),
    #[error("Authorization failed: {0}")]
    NoAuthorization(String),
    #[error("Failed to obtain access token: {0}, code: {1}, msg: {2}")]
    ObtainAccessTokenException(String, u16, String),
    #[error("Deserialize error: {0}")]
    DeserializeError(#[from] serde_json::Error),
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Code: {0}")]
    CodeError(CodeMsg),
    #[error("App ticket is empty")]
    AppTicketEmpty,
}
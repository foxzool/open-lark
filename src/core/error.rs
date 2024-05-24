use thiserror::Error;

#[derive(Error, Debug)]
pub enum LarkAPIError {
    #[error("IO error: {0}")]
    IOErr(#[from] std::io::Error),
    #[error("{0}")]
    IllegalParamError(String),
    #[error("Deserialize error: {0}")]
    DeserializeError(#[from] serde_json::Error),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

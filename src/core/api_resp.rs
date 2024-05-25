use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

/// 业务返回值
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    /// 响应头
    #[serde(flatten)]
    pub raw_response: RawResponse,
    /// 具体数据
    pub data: Option<T>,
}

impl<T> BaseResponse<T> {
    pub fn success(&self) -> bool {
        self.raw_response.code == 0
    }

    pub fn code(&self) -> i32 {
        self.raw_response.code
    }

    pub fn msg(&self) -> &str {
        &self.raw_response.msg
    }

    pub fn err(&self) -> Option<&ErrorInfo> {
        self.raw_response.err.as_ref()
    }
}

/// 业务返回值格式
pub trait ApiResponseTrait: for<'a> Deserialize<'a> + Send + Sync + 'static + Debug {
    /// 是否是标准数据格式, 既是用data包裹数据
    fn data_format() -> ResponseFormat;

    fn from_binary(_file_name: String, _body: Vec<u8>) -> Option<Self> {
        None
    }
}

pub enum ResponseFormat {
    /// 标准数据格式, 既是用data包裹数据
    Data,
    /// 扁平数据格式, 既是直接返回数据
    Flatten,
    /// 二进制数据格式
    Binary,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RawResponse {
    pub code: i32,
    pub msg: String,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub err: Option<ErrorInfo>,
}

impl ApiResponseTrait for RawResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

impl Display for RawResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {}, msg: {}", self.code, self.msg)
    }
}

/// 二进制数据响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryResponse {
    pub file_name: String,
    pub body: Vec<u8>,
}

impl ApiResponseTrait for BinaryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Binary
    }

    fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
        Some(BinaryResponse { file_name, body })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorInfo {
    #[serde(rename = "key", default, skip_serializing_if = "Option::is_none")]
    pub log_id: Option<String>,
    #[serde(rename = "details", default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CodeErrorDetail>,
    #[serde(
        rename = "permission_violations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub permission_violations: Vec<CodeErrorPermissionViolation>,
    #[serde(
        rename = "field_violations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub field_violations: Vec<CodeErrorFieldViolation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeErrorDetail {
    #[serde(rename = "key", default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeErrorPermissionViolation {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeErrorFieldViolation {
    #[serde(rename = "field", default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}



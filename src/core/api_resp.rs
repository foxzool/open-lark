use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

/// 业务返回值
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResp<T> {
    /// 响应头
    #[serde(flatten)]
    pub raw_response: RawResponse,
    /// 具体数据
    pub data: T,
}

/// 业务返回值格式
pub trait ApiResponseTrait:
    Serialize + for<'a> Deserialize<'a> + Send + Sync + 'static + Debug
{
    /// 是否是标准数据格式, 既是用data包裹数据
    fn standard_data_format() -> bool;
}

#[derive(Debug)]
pub enum ApiResponse<T> {
    Success {
        data: T,
        status_code: u16,
        header: Vec<String>,
    },
    Error(RawResponse),
}

// impl<T: for<'a> Deserialize<'a>> TryInto<BaseResp<T>> for ApiResponse {
//     type Error = serde_json::Error;
//
//     fn try_into(self) -> Result<BaseResp<T>, Self::Error> {
//         match serde_json::from_slice::<BaseResp<T>>(&self.raw_body) {
//             Ok(mut resp) => {
//                 resp.api_resp = self;
//                 Ok(resp)
//             }
//             Err(_) => {
//                 let code_msg = serde_json::from_slice::<CodeMsg>(&self.raw_body)?;
//                 Ok(BaseResp {
//                     api_resp: self,
//                     code_msg,
//                     data: None,
//                 })
//             }
//         }
//     }
// }

// impl ApiResponse {request_id
//     pub fn success(&self) -> bool {
//         200 <= self.status_code && self.status_code < 300
//     }
//     pub fn request_id(&self) -> String {
//         match self.header.iter().find(|v| *v == HTTP_HEADER_KEY_LOG_ID) {request_id
//             None => self
//                 .header
//                 .iter()
//                 .find(|v| HTTP_HEADER_KEY_REQUEST_ID == *v)
//                 .unwrap()
//                 .to_string(),
//             Some(log_id) => log_id.to_string(),
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RawResponse {
    pub code: i32,
    pub msg: String,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub err: Option<ErrorInfo>,
}

impl ApiResponseTrait for RawResponse {
    fn standard_data_format() -> bool {
        false
    }
}

impl Display for RawResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {}, msg: {}", self.code, self.msg)
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

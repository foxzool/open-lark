//! API响应类型定义
//!
//! 独立的响应处理系统，替代api_resp模块

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 原始响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawResponse {
    /// 响应代码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 请求数据ID
    pub request_id: Option<String>,
    /// 额外数据
    pub data: Option<serde_json::Value>,
    /// 错误信息
    pub error: Option<ErrorInfo>,
}

/// 错误信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// 错误代码
    pub code: i32,
    /// 错误消息
    pub message: String,
    /// 错误详情
    pub details: Option<HashMap<String, serde_json::Value>>,
}

impl Default for RawResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: "success".to_string(),
            request_id: None,
            data: None,
            error: None,
        }
    }
}

impl RawResponse {
    /// 创建成功响应
    pub fn success() -> Self {
        Self::default()
    }

    /// 创建带数据的成功响应
    pub fn success_with_data(data: serde_json::Value) -> Self {
        Self {
            data: Some(data),
            ..Default::default()
        }
    }

    /// 创建错误响应
    pub fn error(code: i32, msg: impl Into<String> + Clone) -> Self {
        let msg_str = msg.into();
        Self {
            code,
            msg: msg_str.clone(),
            error: Some(ErrorInfo {
                code,
                message: msg_str,
                details: None,
            }),
            ..Default::default()
        }
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    /// 获取错误信息
    pub fn get_error(&self) -> Option<&ErrorInfo> {
        self.error.as_ref()
    }
}

/// 响应格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseFormat {
    /// 标准数据格式
    #[serde(rename = "data")]
    Data,
    /// 扁平格式
    #[serde(rename = "flatten")]
    Flatten,
    /// 二进制数据
    #[serde(rename = "binary")]
    Binary,
    /// 文本数据
    #[serde(rename = "text")]
    Text,
    /// 自定义格式
    #[serde(rename = "custom")]
    Custom,
}

/// API响应特征
pub trait ApiResponseTrait: Send + Sync + 'static {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 通用响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    /// 响应数据
    pub data: Option<T>,
    /// 原始响应
    pub raw_response: RawResponse,
}

impl<T> Response<T> {
    /// 创建新响应
    pub fn new(data: Option<T>, raw_response: RawResponse) -> Self {
        Self { data, raw_response }
    }

    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            raw_response: RawResponse::success(),
        }
    }

    /// 创建空成功响应
    pub fn success_empty() -> Self {
        Self {
            data: None,
            raw_response: RawResponse::success(),
        }
    }

    /// 创建错误响应
    pub fn error(code: i32, msg: impl Into<String> + Clone) -> Self {
        Self {
            data: None,
            raw_response: RawResponse::error(code, msg),
        }
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.raw_response.is_success()
    }

    /// 获取响应代码
    pub fn code(&self) -> i32 {
        self.raw_response.code
    }

    /// 获取响应消息
    pub fn message(&self) -> &str {
        &self.raw_response.msg
    }

    /// 获取数据
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// 获取原始响应
    pub fn raw(&self) -> &RawResponse {
        &self.raw_response
    }

    /// 转换为结果类型
    pub fn into_result(self) -> Result<T, crate::error::LarkAPIError> {
        let is_success = self.is_success();
        let code = self.raw_response.code;
        let request_id = self.raw_response.request_id.clone();

        if is_success {
            match self.data {
                Some(data) => Ok(data),
                None => Err(crate::error::api_error_v3(
                    code as u16,
                    "response",
                    "响应数据为空",
                    request_id,
                )),
            }
        } else {
            Err(crate::error::api_error_v3(
                code as u16,
                "response",
                self.raw_response.msg.clone(),
                request_id,
            ))
        }
    }
}

// 为常见类型实现ApiResponseTrait
impl ApiResponseTrait for serde_json::Value {}
impl ApiResponseTrait for String {}
impl ApiResponseTrait for Vec<u8> {}
impl ApiResponseTrait for () {}

// 类型别名，用于向后兼容
pub type BaseResponse<T> = Response<T>;

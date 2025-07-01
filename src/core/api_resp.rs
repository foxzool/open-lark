use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::core::{error::LarkAPIError, error_codes::LarkErrorCode, SDKResult};

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

    /// 获取语义化的错误码
    pub fn error_code(&self) -> Option<LarkErrorCode> {
        LarkErrorCode::from_code(self.code())
    }

    /// 检查是否为特定错误码
    pub fn is_error_code(&self, code: LarkErrorCode) -> bool {
        self.code() == code as i32
    }

    /// 检查是否为权限相关错误
    pub fn is_permission_error(&self) -> bool {
        self.error_code()
            .map(|c| c.is_permission_error())
            .unwrap_or(false)
    }

    /// 检查是否为认证相关错误
    pub fn is_auth_error(&self) -> bool {
        self.error_code()
            .map(|c| c.is_auth_error())
            .unwrap_or(false)
    }

    /// 检查是否为服务器错误
    pub fn is_server_error(&self) -> bool {
        self.error_code()
            .map(|c| c.is_server_error())
            .unwrap_or(false)
    }

    /// 检查是否为客户端错误
    pub fn is_client_error(&self) -> bool {
        self.error_code()
            .map(|c| c.is_client_error())
            .unwrap_or(false)
    }

    /// 检查是否可以重试
    pub fn is_retryable(&self) -> bool {
        self.error_code().map(|c| c.is_retryable()).unwrap_or(false)
    }

    /// 获取建议的重试延迟时间
    pub fn suggested_retry_delay(&self) -> Option<u64> {
        self.error_code().and_then(|c| c.suggested_retry_delay())
    }

    /// 获取数据或返回友好错误
    pub fn data_or_error(self) -> Result<T, String> {
        if self.success() {
            self.data.ok_or_else(|| "响应成功但数据为空".to_string())
        } else {
            let error_msg = if let Some(code) = self.error_code() {
                code.detailed_description().to_string()
            } else {
                format!("{} (错误码: {})", self.msg(), self.code())
            };
            Err(error_msg)
        }
    }

    /// 获取数据或转换为LarkAPIError
    pub fn data_or_api_error(self) -> SDKResult<T> {
        if self.success() {
            self.data
                .ok_or_else(|| LarkAPIError::api_error(0, "响应成功但数据为空", None))
        } else {
            Err(LarkAPIError::api_error(
                self.code(),
                self.msg(),
                None, // TODO: 可以从响应头中提取request_id
            ))
        }
    }

    /// 处理通用错误，返回处理后的响应或错误
    pub fn handle_common_errors(self) -> SDKResult<Self> {
        if self.success() {
            return Ok(self);
        }

        match self.error_code() {
            Some(LarkErrorCode::AccessTokenInvalid) => Err(LarkAPIError::illegal_param(
                "访问令牌已过期，请重新获取用户授权",
            )),
            Some(LarkErrorCode::AppAccessTokenInvalid) => Err(LarkAPIError::illegal_param(
                "应用访问令牌无效，请检查应用配置",
            )),
            Some(LarkErrorCode::TenantAccessTokenInvalid) => Err(LarkAPIError::illegal_param(
                "租户访问令牌无效，请检查应用权限",
            )),
            Some(LarkErrorCode::Forbidden) => Err(LarkAPIError::illegal_param(
                "权限不足，请检查应用权限配置或用户权限",
            )),
            Some(LarkErrorCode::TooManyRequests) => {
                Err(LarkAPIError::illegal_param("请求过于频繁，请稍后重试"))
            }
            Some(LarkErrorCode::NotFound) => Err(LarkAPIError::illegal_param("请求的资源不存在")),
            _ => {
                // 对于其他错误，返回原始响应让调用者处理
                Ok(self)
            }
        }
    }

    /// 获取用户友好的错误描述
    pub fn user_friendly_error(&self) -> Option<String> {
        if self.success() {
            return None;
        }

        Some(
            self.error_code()
                .map(|c| c.detailed_description().to_string())
                .unwrap_or_else(|| format!("{} (错误码: {})", self.msg(), self.code())),
        )
    }

    /// 获取错误的建议解决方案
    pub fn error_solutions(&self) -> Vec<String> {
        if self.success() {
            return vec![];
        }

        match self.error_code() {
            Some(LarkErrorCode::AccessTokenInvalid) => vec![
                "重新获取用户访问令牌".to_string(),
                "检查令牌是否在有效期内".to_string(),
            ],
            Some(LarkErrorCode::AppAccessTokenInvalid) => vec![
                "检查应用ID和应用密钥".to_string(),
                "确认应用类型配置正确".to_string(),
            ],
            Some(LarkErrorCode::TenantAccessTokenInvalid) => vec![
                "检查租户权限配置".to_string(),
                "确认应用已正确安装到企业".to_string(),
            ],
            Some(LarkErrorCode::Forbidden) => vec![
                "检查应用权限范围设置".to_string(),
                "确认用户具有相应的操作权限".to_string(),
                "联系管理员添加必要权限".to_string(),
            ],
            Some(LarkErrorCode::TooManyRequests) => vec![
                "降低请求频率".to_string(),
                "实现请求重试机制".to_string(),
                "考虑使用请求缓存".to_string(),
            ],
            Some(LarkErrorCode::NotFound) => vec![
                "检查资源ID是否正确".to_string(),
                "确认资源是否存在".to_string(),
            ],
            _ => vec![
                "检查请求参数是否正确".to_string(),
                "参考API文档确认调用方式".to_string(),
            ],
        }
    }

    /// 获取相关的帮助链接
    pub fn help_links(&self) -> Vec<(&'static str, &'static str)> {
        if self.success() {
            return vec![];
        }

        match self.error_code() {
            Some(code) => vec![
                (
                    "官方文档",
                    code.help_url()
                        .unwrap_or("https://open.feishu.cn/document/"),
                ),
                (
                    "开发者社区",
                    "https://getfeishu.cn/hc/zh-cn/categories/360000150856",
                ),
            ],
            None => vec![
                ("API文档", "https://open.feishu.cn/document/"),
                (
                    "开发者社区",
                    "https://getfeishu.cn/hc/zh-cn/categories/360000150856",
                ),
            ],
        }
    }

    /// 打印详细的错误信息（用于调试）
    pub fn print_error_details(&self) {
        if self.success() {
            println!("✅ 请求成功");
            return;
        }

        println!("❌ 请求失败");
        println!("错误码: {}", self.code());

        if let Some(error_code) = self.error_code() {
            println!("错误类型: {}", error_code.description());
            println!("详细说明: {}", error_code.detailed_description());
        }

        println!("错误消息: {}", self.msg());

        let solutions = self.error_solutions();
        if !solutions.is_empty() {
            println!("\n💡 建议解决方案:");
            for (i, solution) in solutions.iter().enumerate() {
                println!("   {}. {}", i + 1, solution);
            }
        }

        let help_links = self.help_links();
        if !help_links.is_empty() {
            println!("\n🔗 相关链接:");
            for (name, url) in help_links {
                println!("   {name}: {url}");
            }
        }

        if let Some(delay) = self.suggested_retry_delay() {
            println!("\n⏱️ 建议重试延迟: {delay}秒");
        }
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

/// 响应格式类型
///
/// 定义API响应的不同格式类型
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

/// 空响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyResponse {}

impl ApiResponseTrait for EmptyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// JSON响应体
pub type JsonResponse = serde_json::Value;

impl ApiResponseTrait for JsonResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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

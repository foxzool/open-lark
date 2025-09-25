use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::admin,
        error::LarkAPIError,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_full_service,
    service::admin::models::{PasswordResetRequest, PasswordResetResponse},
};

// Re-export builders
pub use builders::PasswordResetRequestBuilder;

mod builders;

/// 登录密码管理服务
pub struct PasswordService {
    pub config: Config,
}

// Service 抽象接入：Admin PasswordService
impl_full_service!(PasswordService, "admin.password", "v1");

/// 重置用户企业邮箱密码响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetApiResponse {
    /// 重置密码结果
    #[serde(flatten)]
    pub reset_result: PasswordResetResponse,
}

impl ApiResponseTrait for PasswordResetApiResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PasswordService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 重置用户的企业邮箱密码
    ///
    /// 该接口用于重置企业用户的邮箱登录密码。
    ///
    /// # 参数
    ///
    /// - `request`: 重置密码请求参数
    /// - `option`: 可选的请求配置
    pub async fn reset_password(
        &self,
        request: PasswordResetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PasswordResetApiResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: admin::ADMIN_V1_PASSWORD_RESET.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: match serde_json::to_vec(&serde_json::json!({
                "user_id": request.user_id,
                "password": request.password
            })) {
                Ok(body) => body,
                Err(e) => {
                    return Err(LarkAPIError::DeserializeError(format!(
                        "Failed to serialize password reset request: {}",
                        e
                    )));
                }
            },
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 使用构建器重置用户的企业邮箱密码
    ///
    /// 该接口使用构建器模式重置企业用户的邮箱登录密码，
    /// 自动验证密码强度和必填字段。
    ///
    /// # 参数
    ///
    /// - `builder`: 密码重置请求构建器
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let response = client.admin.password.reset_password_with_builder(
    ///     PasswordResetRequestBuilder::new()
    ///         .user_id("user_id")
    ///         .password("SecurePass123!")
    ///         .build()?,
    ///     None
    /// ).await?;
    /// ```
    pub async fn reset_password_with_builder(
        &self,
        builder_result: SDKResult<PasswordResetRequest>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PasswordResetApiResponse>> {
        let request = builder_result?;
        self.reset_password(request, option).await
    }
}

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::admin::models::{PasswordResetRequest, PasswordResetResponse},
};

/// 登录密码管理服务
pub struct PasswordService {
    pub config: Config,
}

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
            api_path: "/open-apis/admin/v1/password/reset".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(&serde_json::json!({
                "user_id": request.user_id,
                "password": request.password
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

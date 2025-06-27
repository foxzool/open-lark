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
    service::verification::models::VerificationInfo,
};

/// Verification API v1版本服务
pub struct V1 {
    pub config: Config,
}

/// 获取认证信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetVerificationResponse {
    /// 认证信息
    pub verification: VerificationInfo,
}

impl ApiResponseTrait for GetVerificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取认证信息
    ///
    /// 该接口用于获取应用的认证信息，包括认证状态、权限范围等信息。
    ///
    /// 注意事项：
    /// - 需要有效的访问令牌
    /// - 返回当前应用的认证相关信息
    ///
    /// # 错误
    ///
    /// - 99991000: 系统错误
    /// - 99991001: 参数错误
    /// - 99991002: 权限不足
    /// - 99991003: 认证信息不存在
    pub async fn get(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetVerificationResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/verification/v1/get".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

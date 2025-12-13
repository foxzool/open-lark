//! 自建应用获取 tenant_access_token API
use crate::models::auth::{TenantAccessTokenInternalRequest, TenantAccessTokenResponse};
///
/// API文档: https://open.feishu.cn/document/server-docs/authentication-management/access-token/tenant_access_token_internal
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 自建应用获取 tenant_access_token 请求
pub struct TenantAccessTokenInternalRequestBuilder {
    app_id: String,
    app_secret: String,
    /// 配置信息
    config: Config,
}

/// 自建应用获取 tenant_access_token 响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TenantAccessTokenInternalResponseData {
    /// 租户访问令牌响应
    pub data: TenantAccessTokenResponse,
}

impl ApiResponseTrait for TenantAccessTokenInternalResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TenantAccessTokenInternalRequestBuilder {
    /// 创建 tenant_access_token_internal 请求
    pub fn new(config: Config) -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            config,
        }
    }

    /// 设置应用 ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.app_secret = app_secret.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TenantAccessTokenInternalResponseData> {
        // 验证必填字段
        validate_required!(self.app_id, "应用ID不能为空");
        validate_required!(self.app_secret, "应用密钥不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::AuthApiV3;
        let api_endpoint = AuthApiV3::TenantAccessTokenInternal;

        // 构建请求体
        let request_body = TenantAccessTokenInternalRequest {
            app_id: self.app_id.clone(),
            app_secret: self.app_secret.clone(),
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<TenantAccessTokenInternalResponseData> =
            ApiRequest::post(&api_endpoint.to_url()).body(openlark_core::api::RequestData::Json(
                serde_json::to_value(&request_body)?,
            ));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

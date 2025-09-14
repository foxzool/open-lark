use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::TENANT_V2_QUERY,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::tenant::models::Tenant,
};

/// 企业信息服务
pub struct TenantService {
    pub config: Config,
}

/// 获取企业信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTenantResponse {
    /// 企业信息
    pub tenant: Tenant,
}

impl ApiResponseTrait for GetTenantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TenantService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取企业信息
    ///
    /// 该接口用于获取企业的基本信息，包括企业名称、头像等信息。
    ///
    /// 注意事项：
    /// - 需要申请 访问企业信息 权限
    ///
    /// # 错误
    ///
    /// - 99991000: 系统错误
    /// - 99991001: 参数错误
    /// - 99991002: 权限不足
    pub async fn query(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetTenantResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: TENANT_V2_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

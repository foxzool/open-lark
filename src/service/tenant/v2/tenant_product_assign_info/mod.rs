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
    service::tenant::models::TenantProductAssignInfo,
};

/// 企业席位信息服务
pub struct TenantProductAssignInfoService {
    pub config: Config,
}

/// 获取企业席位信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTenantProductAssignInfoResponse {
    /// 企业席位信息
    pub tenant_product_assign_info: TenantProductAssignInfo,
}

impl ApiResponseTrait for GetTenantProductAssignInfoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TenantProductAssignInfoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取企业席位信息
    ///
    /// 该接口用于获取企业的席位信息，包括总席位数、已分配席位数等信息。
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
    ) -> SDKResult<BaseResponse<GetTenantProductAssignInfoResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/tenant/v2/tenant_product_assign_info/query".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

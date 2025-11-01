use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::core::{,
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
    constants::AccessTokenType,
    endpoints::{EndpointBuilder, Endpointshttp::Transport,
    req_option::RequestOption,
    SDKResult,
};
/// 规则看板管理服务
pub struct RuleViewService {
}

impl RuleViewService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 移除规则看板
    ///,
/// 删除指定的规则看板。
    ///,
/// # Arguments
    ///,
/// * `view_id` - 看板ID
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回移除结果
pub async fn remove(,
        &self,
        view_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleViewRemoveResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::REPORT_V1_RULE_VIEWS_OPERATION,
                "view_id",
                view_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 规则看板移除响应
#[derive(Debug, Clone)]
pub struct RuleViewRemoveResponse {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}}}}}
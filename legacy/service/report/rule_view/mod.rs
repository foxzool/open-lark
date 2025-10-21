use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{EndpointBuilder, Endpoints},
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 规则看板管理服务
pub struct RuleViewService {
    pub config: Config,
}

impl RuleViewService {
    /// 创建规则看板管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 移除规则看板
    ///
    /// 删除指定的规则看板。
    ///
    /// # Arguments
    ///
    /// * `view_id` - 看板ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回移除结果
    pub async fn remove(
        &self,
        view_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleViewRemoveResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                Endpoints::REPORT_V1_RULE_VIEWS_OPERATION,
                "view_id",
                view_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 规则看板移除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleViewRemoveResponse {
    /// 移除是否成功
    pub success: bool,
    /// 被移除的看板ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ApiResponseTrait for RuleViewRemoveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

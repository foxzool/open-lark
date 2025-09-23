use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 应用消息流卡片服务
pub struct AppFeedCardService {
    pub config: Config,
}

/// 创建应用消息流卡片请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppFeedCardRequest {
    /// 卡片内容
    pub card_content: Value,
    /// 目标用户ID列表
    pub target_users: Vec<String>,
    /// 卡片标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 卡片描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 创建应用消息流卡片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAppFeedCardResponse {
    /// 卡片ID
    pub card_id: String,
    /// 创建时间
    pub create_time: String,
}

impl ApiResponseTrait for CreateAppFeedCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新应用消息流卡片请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppFeedCardRequest {
    /// 卡片内容
    pub card_content: Value,
    /// 更新标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 更新描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 更新应用消息流卡片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppFeedCardResponse {
    /// 卡片ID
    pub card_id: String,
    /// 更新时间
    pub update_time: String,
}

impl ApiResponseTrait for UpdateAppFeedCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AppFeedCardService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建应用消息流卡片
    pub async fn create(
        &self,
        request: CreateAppFeedCardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateAppFeedCardResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: crate::core::endpoints::im::IM_V2_APP_FEED_CARD.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新应用消息流卡片
    pub async fn update(
        &self,
        card_id: &str,
        request: UpdateAppFeedCardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateAppFeedCardResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V2_GET_APP_FEED_CARD,
                "card_id",
                card_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除应用消息流卡片
    pub async fn delete(
        &self,
        card_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V2_DELETE_APP_FEED_CARD,
                "card_id",
                card_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

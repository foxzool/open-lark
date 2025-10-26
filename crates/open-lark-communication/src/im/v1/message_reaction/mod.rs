use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::im::v1::models::UserIdType;
use open_lark_core::core::standard_response::StandardResponse;
use open_lark_core::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 表情回复服务
pub struct MessageReactionService {
    pub config: Config,
}

/// 添加表情回复请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReactionRequest {
    /// 表情类型
    pub emoji_type: String,
}

/// 消息表情回复
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReaction {
    /// 表情回复ID
    pub reaction_id: String,
    /// 表情类型
    pub emoji_type: String,
    /// 用户信息
    pub user_id: String,
    /// 创建时间
    pub created_at: String,
}

// 接入统一 Service 抽象（IM v1 - MessageReactionService）
// impl_full_service!(MessageReactionService, "im.message_reaction", "v1");

/// 获取表情回复响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReactionResponse {
    /// 表情回复列表
    pub reactions: Vec<MessageReaction>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListReactionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MessageReactionService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 添加消息表情回复
    pub async fn create(
        &self,
        message_id: &str,
        emoji_type: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<EmptyResponse> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(
            open_lark_core::core::endpoints::im::IM_V1_MESSAGE_REACTIONS,
            "message_id",
            message_id,
        ));
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params = query_params;
        api_req.body = serde_json::to_vec(&CreateReactionRequest {
            emoji_type: emoji_type.to_string(),
        })?;

        let api_resp: BaseResponse<EmptyResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 获取消息表情回复
    pub async fn list(
        &self,
        message_id: &str,
        user_id_type: Option<UserIdType>,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<ListReactionResponse> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            open_lark_core::core::endpoints::im::IM_V1_MESSAGE_REACTIONS,
            "message_id",
            message_id,
        ));
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params = query_params;

        let api_resp: BaseResponse<ListReactionResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 删除消息表情回复
    pub async fn delete(
        &self,
        message_id: &str,
        reaction_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<EmptyResponse> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
        let mut params = std::collections::HashMap::new();
        params.insert("message_id".to_string(), message_id.to_string());
        params.insert("reaction_id".to_string(), reaction_id.to_string());
        api_req.set_api_path(EndpointBuilder::replace_params(
            open_lark_core::core::endpoints::im::IM_V1_MESSAGE_REACTIONS_DELETE,
            &params,
        ));
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params = query_params;

        let api_resp: BaseResponse<EmptyResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

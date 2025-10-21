use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::impl_full_service;
use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        standard_response::StandardResponse,
        SDKResult,
    },
    service::im::v1::models::{MessageReaction, UserIdType},
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

// 接入统一 Service 抽象（IM v1 - MessageReactionService）
impl_full_service!(MessageReactionService, "im.message_reaction", "v1");

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

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V1_MESSAGE_REACTIONS,
                "message_id",
                message_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&CreateReactionRequest {
                emoji_type: emoji_type.to_string(),
            })?,
            ..Default::default()
        };

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

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V1_MESSAGE_REACTIONS,
                "message_id",
                message_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

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

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_params_from_array(
                crate::core::endpoints::im::IM_V1_DELETE_MESSAGE_REACTION,
                &[("message_id", message_id), ("reaction_id", reaction_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let api_resp: BaseResponse<EmptyResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

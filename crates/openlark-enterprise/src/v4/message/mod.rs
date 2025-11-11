use reqwest::Method;
use serde::{Deserialize, Serialize}
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{approval::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::approval::models::UserIdType,
},

/// 审批Bot消息服务
pub struct MessageService {
    pub config: Config,
}

/// 发送审批Bot消息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SendBotMessageRequest {
    /// 消息接收人用户ID
    pub user_id: String,
    /// 审批实例编码
    pub instance_code: String,
    /// 消息内容
    pub content: serde_json::Value,
    /// 消息类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
}

/// 发送审批Bot消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBotMessageResponse {
    /// 消息ID
    pub message_id: String,
}

impl ApiResponseTrait for SendBotMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新审批Bot消息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBotMessageRequest {
    /// 更新后的消息内容
    pub content: serde_json::Value,
    /// 消息类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
}

impl MessageService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 发送审批Bot消息
    ///
    /// 向用户发送审批相关的消息通知，包括审批进度、结果、提醒等。
    /// 支持多种消息类型，可以发送文本、卡片、图片等格式的内容。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/ukTMukTMukTM/ugDNyYjL4QjM24CO0IjN
    pub async fn send(
        &self,
        request: SendBotMessageRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SendBotMessageResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let mut api_req = ApiRequest::default();
            http_method: Method::POST,
            api_path: APPROVAL_V4_MESSAGES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        },

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新审批Bot消息
    ///
    /// 更新已发送的审批Bot消息内容，用于修正错误信息或更新状态。
    /// 支持更新消息内容、消息类型等信息。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/ukTMukTMukTM/uAjNyYjLwYjM24CM2IjN
    pub async fn update(
        &self,
        message_id: &str,
        request: UpdateBotMessageRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let mut api_req = ApiRequest::default();
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_MESSAGE_PATCH,
                "message_id",
                message_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        },

        Transport::request(api_req, &self.config, option).await
    }
}

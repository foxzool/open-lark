use std::collections::HashMap;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use crate::service::im::v1::models::{BatchMessageStatus, UserIdType, ReceiveIdType};

/// 批量消息服务
pub struct BatchMessageService {
    pub config: Config,
}

/// 批量发送消息请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSendMessageRequest {
    /// 消息接收者ID列表
    pub receive_id_list: Vec<String>,
    /// 消息内容
    pub msg_type: String,
    /// 消息内容
    pub content: String,
    /// 用户ID类型，可选值为open_id、user_id、union_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// 批量发送消息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSendMessageResponse {
    /// 批量消息ID
    pub batch_message_id: String,
    /// 无效的接收者ID列表
    pub invalid_receive_ids: Vec<String>,
}

impl ApiResponseTrait for BatchSendMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询批量消息进度响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBatchProgressResponse {
    /// 批量消息进度信息
    pub batch_message_progress: BatchMessageProgress,
}

/// 批量消息进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMessageProgress {
    /// 批量消息ID
    pub batch_message_id: String,
    /// 状态
    pub status: BatchMessageStatus,
    /// 总数
    pub total_count: i32,
    /// 成功数
    pub success_count: i32,
    /// 失败数
    pub fail_count: i32,
    /// 已读数
    pub read_count: i32,
}

impl ApiResponseTrait for GetBatchProgressResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询批量消息推送和阅读人数响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBatchReadUserResponse {
    /// 已读用户列表
    pub read_users: Vec<BatchReadUser>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 批量消息已读用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchReadUser {
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub name: Option<String>,
    /// 阅读时间
    pub read_time: String,
}

impl ApiResponseTrait for GetBatchReadUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchMessageService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量发送消息
    pub async fn send(
        &self,
        receive_id_type: ReceiveIdType,
        request: BatchSendMessageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchSendMessageResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/im/v1/batch_messages".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params: HashMap::from([("receive_id_type".to_string(), receive_id_type.as_str().to_string())]),
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量撤回消息
    pub async fn delete(
        &self,
        batch_message_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: format!("/open-apis/im/v1/batch_messages/{}", batch_message_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询批量消息整体进度
    pub async fn get_progress(
        &self,
        batch_message_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetBatchProgressResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/im/v1/batch_messages/{}/get_progress", batch_message_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询批量消息推送和阅读人数
    pub async fn read_user(
        &self,
        batch_message_id: &str,
        user_id_type: Option<UserIdType>,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetBatchReadUserResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type".to_string(), user_id_type.as_str().to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token".to_string(), page_token);
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/im/v1/batch_messages/{}/read_user", batch_message_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
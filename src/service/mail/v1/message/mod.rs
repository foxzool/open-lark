use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::mail::models::{MailAddress, MailBody, Message, UserIdType},
};

/// 用户邮件服务
pub struct MessageService {
    pub config: Config,
}

/// 发送邮件请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    /// 收件人
    pub to: Vec<MailAddress>,
    /// 抄送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<MailAddress>>,
    /// 密送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<MailAddress>>,
    /// 主题
    pub subject: String,
    /// 邮件内容
    pub body: MailBody,
    /// 附件ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_ids: Option<Vec<String>>,
}

/// 发送邮件响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageResponse {
    /// 邮件ID
    pub message_id: String,
}

impl ApiResponseTrait for SendMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取邮件详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetMessageResponse {
    /// 邮件详情
    pub message: Message,
}

impl ApiResponseTrait for GetMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出邮件响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListMessagesResponse {
    /// 邮件列表
    pub messages: Vec<Message>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListMessagesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取邮件卡片的邮件列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetMessagesByCardResponse {
    /// 邮件列表
    pub messages: Vec<Message>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetMessagesByCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MessageService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 发送邮件
    ///
    /// 该接口用于通过指定用户邮箱发送邮件。
    ///
    /// # 参数
    ///
    /// - `user_mailbox_id`: 用户邮箱ID
    /// - `request`: 发送邮件请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 邮箱不存在
    pub async fn send(
        &self,
        user_mailbox_id: &str,
        request: SendMessageRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SendMessageResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_MESSAGES,
                "user_mailbox_id",
                user_mailbox_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取邮件详情
    ///
    /// 该接口用于获取指定邮件的详细信息。
    ///
    /// # 参数
    ///
    /// - `user_mailbox_id`: 用户邮箱ID
    /// - `message_id`: 邮件ID
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 邮件不存在
    pub async fn get(
        &self,
        user_mailbox_id: &str,
        message_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetMessageResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                &EndpointBuilder::replace_param(
                    Endpoints::MAIL_V1_USER_MAILBOX_MESSAGE,
                    "user_mailbox_id",
                    user_mailbox_id,
                ),
                "message_id",
                message_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 列出邮件
    ///
    /// 该接口用于获取用户邮箱中的邮件列表。
    ///
    /// # 参数
    ///
    /// - `user_mailbox_id`: 用户邮箱ID
    /// - `folder_id`: 文件夹ID，用于筛选特定文件夹的邮件
    /// - `page_size`: 分页大小，最大值100
    /// - `page_token`: 分页标识
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 邮箱不存在
    pub async fn list(
        &self,
        user_mailbox_id: &str,
        folder_id: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListMessagesResponse>> {
        let mut query_params = HashMap::new();
        if let Some(folder_id) = folder_id {
            query_params.insert("folder_id", folder_id);
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_MESSAGES,
                "user_mailbox_id",
                user_mailbox_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取邮件卡片的邮件列表
    ///
    /// 该接口用于根据邮件卡片获取相关的邮件列表。
    ///
    /// # 参数
    ///
    /// - `user_mailbox_id`: 用户邮箱ID
    /// - `card_id`: 邮件卡片ID
    /// - `page_size`: 分页大小，最大值100
    /// - `page_token`: 分页标识
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 邮箱不存在
    pub async fn get_by_card(
        &self,
        user_mailbox_id: &str,
        card_id: &str,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetMessagesByCardResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("card_id", card_id.to_string());
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MAIL_V1_USER_MAILBOX_MESSAGES_GET_BY_CARD,
                "user_mailbox_id",
                user_mailbox_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

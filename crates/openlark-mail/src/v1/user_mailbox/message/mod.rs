//! 用户邮箱消息模块

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ===== 数据模型 =====

/// 获取用户邮箱消息响应
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxMessageResponse {
    /// 消息 ID
    pub message_id: String,
    /// 邮件主题
    pub subject: String,
    /// 发件人
    pub from: Vec<UserMailboxMessageSender>,
    /// 收件人
    pub to: Vec<UserMailboxMessageRecipient>,
    /// 抄送人
    #[serde(default)]
    pub cc: Vec<UserMailboxMessageRecipient>,
    /// 发送时间
    pub send_time: String,
    /// 是否已读
    pub is_read: bool,
}

/// 用户邮箱消息发件人
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxMessageSender {
    /// 名称
    pub name: String,
    /// 邮箱地址
    pub email: String,
}

/// 用户邮箱消息收件人
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxMessageRecipient {
    /// 名称
    pub name: String,
    /// 邮箱地址
    pub email: String,
}

/// 获取用户邮箱消息列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxMessageListResponse {
    /// 消息列表
    pub items: Vec<UserMailboxMessageItem>,
    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 用户邮箱消息项目
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxMessageItem {
    /// 消息 ID
    pub message_id: String,
    /// 邮件主题
    pub subject: String,
    /// 发件人
    pub from: UserMailboxMessageSender,
    /// 发送时间
    pub send_time: String,
    /// 是否已读
    pub is_read: bool,
}

/// 发送用户邮箱消息请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct SendUserMailboxMessageBody {
    /// 收件人邮箱列表
    pub to_recipients: Vec<String>,
    /// 邮件主题
    pub subject: String,
    /// 邮件内容
    pub content: String,
}

/// 发送用户邮箱消息响应
#[derive(Debug, Clone, Deserialize)]
pub struct SendUserMailboxMessageResponse {
    /// 消息 ID
    pub message_id: String,
}

/// 获取用户邮箱消息附件下载链接响应
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxMessageAttachmentDownloadUrlResponse {
    /// 下载链接
    pub download_url: String,
    /// 文件名
    pub file_name: String,
}

/// Message：用户邮箱消息资源
#[derive(Clone)]
pub struct Message {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Message {
    pub fn new(config: Arc<Config>, mailbox_id: String) -> Self {
        Self { config, mailbox_id }
    }

    /// 获取消息详情
    pub fn get(&self, message_id: impl Into<String>) -> GetUserMailboxMessageRequest {
        GetUserMailboxMessageRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            message_id.into(),
        )
    }

    /// 获取消息列表
    pub fn list(&self) -> UserMailboxMessageListRequest {
        UserMailboxMessageListRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 发送消息
    pub fn send(&self) -> SendUserMailboxMessageRequest {
        SendUserMailboxMessageRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取附件下载链接
    pub fn attachment_download_url(&self, attachment_id: impl Into<String>) -> UserMailboxMessageAttachmentDownloadUrlRequest {
        UserMailboxMessageAttachmentDownloadUrlRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            attachment_id.into(),
        )
    }
}

/// 获取用户邮箱消息请求
#[derive(Debug, Clone)]
pub struct GetUserMailboxMessageRequest {
    config: Arc<Config>,
    mailbox_id: String,
    message_id: String,
}

impl GetUserMailboxMessageRequest {
    pub fn new(config: Arc<Config>, mailbox_id: String, message_id: String) -> Self {
        Self {
            config,
            mailbox_id,
            message_id,
        }
    }

    pub async fn execute(self) -> SDKResult<UserMailboxMessageResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserMailboxMessageResponse> {
        let api_endpoint = MailApiV1::UserMailboxMessageGet(self.mailbox_id.clone());
        let mut request = ApiRequest::<UserMailboxMessageResponse>::get(api_endpoint.to_url());
        request = request.query("message_id", &self.message_id);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取用户邮箱消息")
    }
}

impl ApiResponseTrait for UserMailboxMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户邮箱消息列表请求
#[derive(Debug, Clone)]
pub struct UserMailboxMessageListRequest {
    config: Arc<Config>,
    mailbox_id: String,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl UserMailboxMessageListRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            page_token: None,
            page_size: None,
        }
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub async fn execute(self) -> SDKResult<UserMailboxMessageListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserMailboxMessageListResponse> {
        let api_endpoint = MailApiV1::UserMailboxMessageList(self.mailbox_id.clone());
        let mut request = ApiRequest::<UserMailboxMessageListResponse>::get(api_endpoint.to_url());

        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = &self.page_size {
            request = request.query("page_size", page_size.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取用户邮箱消息列表")
    }
}

impl ApiResponseTrait for UserMailboxMessageListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 发送用户邮箱消息请求
#[derive(Debug, Clone)]
pub struct SendUserMailboxMessageRequest {
    config: Arc<Config>,
    mailbox_id: String,
    body: SendUserMailboxMessageBody,
}

impl SendUserMailboxMessageRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            body: SendUserMailboxMessageBody::default(),
        }
    }

    /// 设置收件人
    pub fn to_recipients(mut self, recipients: Vec<String>) -> Self {
        self.body.to_recipients = recipients;
        self
    }

    /// 设置邮件主题
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.body.subject = subject.into();
        self
    }

    /// 设置邮件内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = content.into();
        self
    }

    pub async fn execute(self) -> SDKResult<SendUserMailboxMessageResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SendUserMailboxMessageResponse> {
        validate_required!(self.body.to_recipients, "收件人不能为空");
        validate_required!(self.body.subject.trim(), "邮件主题不能为空");
        validate_required!(self.body.content.trim(), "邮件内容不能为空");

        let api_endpoint = MailApiV1::UserMailboxMessageSend(self.mailbox_id.clone());
        let mut request = ApiRequest::<SendUserMailboxMessageResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "发送用户邮箱消息")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "发送用户邮箱消息")
    }
}

impl ApiResponseTrait for SendUserMailboxMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户邮箱消息附件下载链接请求
#[derive(Debug, Clone)]
pub struct UserMailboxMessageAttachmentDownloadUrlRequest {
    config: Arc<Config>,
    mailbox_id: String,
    attachment_id: String,
}

impl UserMailboxMessageAttachmentDownloadUrlRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>, attachment_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            attachment_id: attachment_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UserMailboxMessageAttachmentDownloadUrlResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserMailboxMessageAttachmentDownloadUrlResponse> {
        let api_endpoint = MailApiV1::UserMailboxMessageAttachmentDownloadUrl(self.mailbox_id.clone());
        let mut request = ApiRequest::<UserMailboxMessageAttachmentDownloadUrlResponse>::get(api_endpoint.to_url());
        request = request.query("attachment_id", &self.attachment_id);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取用户邮箱消息附件下载链接")
    }
}

impl ApiResponseTrait for UserMailboxMessageAttachmentDownloadUrlResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_user_mailbox_message_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = SendUserMailboxMessageRequest::new(config, "mailbox_123".to_string())
            .to_recipients(vec!["test@example.com".to_string()])
            .subject("测试邮件")
            .content("这是一封测试邮件");

        assert_eq!(request.mailbox_id, "mailbox_123");
        assert_eq!(request.body.to_recipients, vec!["test@example.com"]);
        assert_eq!(request.body.subject, "测试邮件");
    }
}

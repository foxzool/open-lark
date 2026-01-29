//! 用户邮箱邮件联系人模块

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ===== 数据模型 =====

/// 创建用户邮箱邮件联系人请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateUserMailboxMailContactBody {
    /// 联系人名称
    pub name: String,
    /// 联系人邮箱
    pub email: String,
}

/// 创建用户邮箱邮件联系人响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserMailboxMailContactResponse {
    /// 联系人 ID
    pub contact_id: String,
    /// 联系人名称
    pub name: String,
    /// 联系人邮箱
    pub email: String,
    /// 创建时间
    pub create_time: String,
}

/// 获取用户邮箱邮件联系人列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxMailContactListResponse {
    /// 联系人列表
    pub items: Vec<UserMailboxMailContactItem>,
    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 用户邮箱邮件联系人项目
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxMailContactItem {
    /// 联系人 ID
    pub contact_id: String,
    /// 联系人名称
    pub name: String,
    /// 联系人邮箱
    pub email: String,
    /// 创建时间
    pub create_time: String,
}

/// 删除用户邮箱邮件联系人响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUserMailboxMailContactResponse {
    /// 是否删除成功
    pub success: bool,
}

/// 部分更新用户邮箱邮件联系人请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct PatchUserMailboxMailContactBody {
    /// 联系人名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 联系人邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// 部分更新用户邮箱邮件联系人响应
#[derive(Debug, Clone, Deserialize)]
pub struct PatchUserMailboxMailContactResponse {
    /// 联系人 ID
    pub contact_id: String,
    /// 更新时间
    pub update_time: String,
}

/// MailContact：用户邮箱邮件联系人资源
#[derive(Clone)]
pub struct MailContact {
    config: Arc<Config>,
    mailbox_id: String,
}

impl MailContact {
    pub fn new(config: Arc<Config>, mailbox_id: String) -> Self {
        Self { config, mailbox_id }
    }

    /// 创建联系人
    pub fn create(&self) -> CreateUserMailboxMailContactRequest {
        CreateUserMailboxMailContactRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取联系人列表
    pub fn list(&self) -> UserMailboxMailContactListRequest {
        UserMailboxMailContactListRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 删除联系人
    pub fn delete(&self, contact_id: impl Into<String>) -> DeleteUserMailboxMailContactRequest {
        DeleteUserMailboxMailContactRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            contact_id.into(),
        )
    }

    /// 部分更新联系人
    pub fn patch(&self, contact_id: impl Into<String>) -> PatchUserMailboxMailContactRequest {
        PatchUserMailboxMailContactRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            contact_id.into(),
        )
    }
}

/// 创建用户邮箱邮件联系人请求
#[derive(Debug, Clone)]
pub struct CreateUserMailboxMailContactRequest {
    config: Arc<Config>,
    mailbox_id: String,
    body: CreateUserMailboxMailContactBody,
}

impl CreateUserMailboxMailContactRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            body: CreateUserMailboxMailContactBody::default(),
        }
    }

    /// 设置联系人名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置联系人邮箱
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.body.email = email.into();
        self
    }

    pub async fn execute(self) -> SDKResult<CreateUserMailboxMailContactResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateUserMailboxMailContactResponse> {
        validate_required!(self.body.name.trim(), "联系人名称不能为空");
        validate_required!(self.body.email.trim(), "联系人邮箱不能为空");

        let api_endpoint = MailApiV1::UserMailboxMailContactCreate(self.mailbox_id.clone());
        let mut request =
            ApiRequest::<CreateUserMailboxMailContactResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建用户邮箱邮件联系人")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建用户邮箱邮件联系人")
    }
}

impl ApiResponseTrait for CreateUserMailboxMailContactResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户邮箱邮件联系人列表请求
#[derive(Debug, Clone)]
pub struct UserMailboxMailContactListRequest {
    config: Arc<Config>,
    mailbox_id: String,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl UserMailboxMailContactListRequest {
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

    pub async fn execute(self) -> SDKResult<UserMailboxMailContactListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserMailboxMailContactListResponse> {
        let api_endpoint = MailApiV1::UserMailboxMailContactList(self.mailbox_id.clone());
        let mut request =
            ApiRequest::<UserMailboxMailContactListResponse>::get(api_endpoint.to_url());

        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = &self.page_size {
            request = request.query("page_size", page_size.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取用户邮箱邮件联系人列表")
    }
}

impl ApiResponseTrait for UserMailboxMailContactListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除用户邮箱邮件联系人请求
#[derive(Debug, Clone)]
pub struct DeleteUserMailboxMailContactRequest {
    config: Arc<Config>,
    mailbox_id: String,
    contact_id: String,
}

impl DeleteUserMailboxMailContactRequest {
    pub fn new(config: Arc<Config>, mailbox_id: String, contact_id: String) -> Self {
        Self {
            config,
            mailbox_id,
            contact_id,
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteUserMailboxMailContactResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteUserMailboxMailContactResponse> {
        let api_endpoint = MailApiV1::UserMailboxMailContactDelete(self.mailbox_id.clone());
        let mut request =
            ApiRequest::<DeleteUserMailboxMailContactResponse>::delete(api_endpoint.to_url());
        request = request.query("contact_id", &self.contact_id);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除用户邮箱邮件联系人")
    }
}

impl ApiResponseTrait for DeleteUserMailboxMailContactResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 部分更新用户邮箱邮件联系人请求
#[derive(Debug, Clone)]
pub struct PatchUserMailboxMailContactRequest {
    config: Arc<Config>,
    mailbox_id: String,
    contact_id: String,
    body: PatchUserMailboxMailContactBody,
}

impl PatchUserMailboxMailContactRequest {
    pub fn new(
        config: Arc<Config>,
        mailbox_id: impl Into<String>,
        contact_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            contact_id: contact_id.into(),
            body: PatchUserMailboxMailContactBody::default(),
        }
    }

    /// 设置联系人名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    /// 设置联系人邮箱
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.body.email = Some(email.into());
        self
    }

    pub async fn execute(self) -> SDKResult<PatchUserMailboxMailContactResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchUserMailboxMailContactResponse> {
        let api_endpoint = MailApiV1::UserMailboxMailContactPatch(self.mailbox_id.clone());
        let mut request =
            ApiRequest::<PatchUserMailboxMailContactResponse>::patch(api_endpoint.to_url());
        request = request.query("contact_id", &self.contact_id);

        let request_body = &self.body;
        request = request.body(serialize_params(
            request_body,
            "部分更新用户邮箱邮件联系人",
        )?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "部分更新用户邮箱邮件联系人")
    }
}

impl ApiResponseTrait for PatchUserMailboxMailContactResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_mailbox_mail_contact_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateUserMailboxMailContactRequest::new(config, "mailbox_123".to_string())
            .name("张三")
            .email("zhangsan@example.com");

        assert_eq!(request.mailbox_id, "mailbox_123");
        assert_eq!(request.body.name, "张三");
        assert_eq!(request.body.email, "zhangsan@example.com");
    }
}

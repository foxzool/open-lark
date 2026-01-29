//! 用户邮箱别名模块

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ===== 数据模型 =====

/// 创建用户邮箱别名请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateUserMailboxAliasBody {
    /// 别名邮箱地址
    pub email: String,
}

/// 创建用户邮箱别名响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserMailboxAliasResponse {
    /// 别名 ID
    pub alias_id: String,
    /// 别名邮箱地址
    pub email: String,
    /// 创建时间
    pub create_time: String,
}

/// 用户邮箱别名列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxAliasListResponse {
    /// 别名列表
    pub items: Vec<UserMailboxAliasItem>,
    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 用户邮箱别名项目
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxAliasItem {
    /// 别名 ID
    pub alias_id: String,
    /// 别名邮箱地址
    pub email: String,
    /// 创建时间
    pub create_time: String,
}

/// 删除用户邮箱别名响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUserMailboxAliasResponse {
    /// 是否删除成功
    pub success: bool,
}

/// Alias：用户邮箱别名资源
#[derive(Clone)]
pub struct Alias {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Alias {
    pub fn new(config: Arc<Config>, mailbox_id: String) -> Self {
        Self { config, mailbox_id }
    }

    /// 创建别名
    pub fn create(&self) -> CreateUserMailboxAliasRequest {
        CreateUserMailboxAliasRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取别名列表
    pub fn list(&self) -> UserMailboxAliasListRequest {
        UserMailboxAliasListRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 删除别名
    pub fn delete(&self, alias_id: impl Into<String>) -> DeleteUserMailboxAliasRequest {
        DeleteUserMailboxAliasRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            alias_id.into(),
        )
    }
}

/// 创建用户邮箱别名请求
#[derive(Debug, Clone)]
pub struct CreateUserMailboxAliasRequest {
    config: Arc<Config>,
    mailbox_id: String,
    body: CreateUserMailboxAliasBody,
}

impl CreateUserMailboxAliasRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            body: CreateUserMailboxAliasBody::default(),
        }
    }

    /// 设置别名邮箱地址
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.body.email = email.into();
        self
    }

    pub async fn execute(self) -> SDKResult<CreateUserMailboxAliasResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateUserMailboxAliasResponse> {
        validate_required!(self.body.email.trim(), "别名邮箱地址不能为空");

        let api_endpoint = MailApiV1::UserMailboxAliasCreate(self.mailbox_id.clone());
        let mut request = ApiRequest::<CreateUserMailboxAliasResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建用户邮箱别名")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建用户邮箱别名")
    }
}

impl ApiResponseTrait for CreateUserMailboxAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户邮箱别名列表请求
#[derive(Debug, Clone)]
pub struct UserMailboxAliasListRequest {
    config: Arc<Config>,
    mailbox_id: String,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl UserMailboxAliasListRequest {
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

    pub async fn execute(self) -> SDKResult<UserMailboxAliasListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserMailboxAliasListResponse> {
        let api_endpoint = MailApiV1::UserMailboxAliasList(self.mailbox_id.clone());
        let mut request = ApiRequest::<UserMailboxAliasListResponse>::get(api_endpoint.to_url());

        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = &self.page_size {
            request = request.query("page_size", page_size.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取用户邮箱别名列表")
    }
}

impl ApiResponseTrait for UserMailboxAliasListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除用户邮箱别名请求
#[derive(Debug, Clone)]
pub struct DeleteUserMailboxAliasRequest {
    config: Arc<Config>,
    mailbox_id: String,
    alias_id: String,
}

impl DeleteUserMailboxAliasRequest {
    pub fn new(config: Arc<Config>, mailbox_id: String, alias_id: String) -> Self {
        Self {
            config,
            mailbox_id,
            alias_id,
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteUserMailboxAliasResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteUserMailboxAliasResponse> {
        validate_required!(self.mailbox_id.trim(), "邮箱ID不能为空");
        validate_required!(self.alias_id.trim(), "别名ID不能为空");

        let api_endpoint =
            MailApiV1::UserMailboxAliasDelete(self.mailbox_id.clone(), self.alias_id.clone());
        let request = ApiRequest::<DeleteUserMailboxAliasResponse>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除用户邮箱别名")
    }
}

impl ApiResponseTrait for DeleteUserMailboxAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_mailbox_alias_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateUserMailboxAliasRequest::new(config, "mailbox_123".to_string())
            .email("alias@example.com");

        assert_eq!(request.mailbox_id, "mailbox_123");
        assert_eq!(request.body.email, "alias@example.com");
    }
}

//! 用户邮箱文件夹模块

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ===== 数据模型 =====

/// 创建用户邮箱文件夹请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateUserMailboxFolderBody {
    /// 文件夹名称
    pub name: String,
    /// 父文件夹 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

/// 创建用户邮箱文件夹响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserMailboxFolderResponse {
    /// 文件夹 ID
    pub folder_id: String,
    /// 文件夹名称
    pub name: String,
    /// 父文件夹 ID
    #[serde(default)]
    pub parent_id: Option<String>,
    /// 创建时间
    pub create_time: String,
}

/// 获取用户邮箱文件夹列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxFolderListResponse {
    /// 文件夹列表
    pub items: Vec<UserMailboxFolderItem>,
    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 用户邮箱文件夹项目
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxFolderItem {
    /// 文件夹 ID
    pub folder_id: String,
    /// 文件夹名称
    pub name: String,
    /// 父文件夹 ID
    #[serde(default)]
    pub parent_id: Option<String>,
    /// 创建时间
    pub create_time: String,
}

/// 删除用户邮箱文件夹响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUserMailboxFolderResponse {
    /// 是否删除成功
    pub success: bool,
}

/// 部分更新用户邮箱文件夹请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct PatchUserMailboxFolderBody {
    /// 文件夹名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 部分更新用户邮箱文件夹响应
#[derive(Debug, Clone, Deserialize)]
pub struct PatchUserMailboxFolderResponse {
    /// 文件夹 ID
    pub folder_id: String,
    /// 更新时间
    pub update_time: String,
}

/// Folder：用户邮箱文件夹资源
#[derive(Clone)]
pub struct Folder {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Folder {
    pub fn new(config: Arc<Config>, mailbox_id: String) -> Self {
        Self { config, mailbox_id }
    }

    /// 创建文件夹
    pub fn create(&self) -> CreateUserMailboxFolderRequest {
        CreateUserMailboxFolderRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取文件夹列表
    pub fn list(&self) -> UserMailboxFolderListRequest {
        UserMailboxFolderListRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 删除文件夹
    pub fn delete(&self, folder_id: impl Into<String>) -> DeleteUserMailboxFolderRequest {
        DeleteUserMailboxFolderRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            folder_id.into(),
        )
    }

    /// 部分更新文件夹
    pub fn patch(&self, folder_id: impl Into<String>) -> PatchUserMailboxFolderRequest {
        PatchUserMailboxFolderRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            folder_id.into(),
        )
    }
}

/// 创建用户邮箱文件夹请求
#[derive(Debug, Clone)]
pub struct CreateUserMailboxFolderRequest {
    config: Arc<Config>,
    mailbox_id: String,
    body: CreateUserMailboxFolderBody,
}

impl CreateUserMailboxFolderRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            body: CreateUserMailboxFolderBody::default(),
        }
    }

    /// 设置文件夹名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置父文件夹 ID
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.body.parent_id = Some(parent_id.into());
        self
    }

    pub async fn execute(self) -> SDKResult<CreateUserMailboxFolderResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateUserMailboxFolderResponse> {
        validate_required!(self.body.name.trim(), "文件夹名称不能为空");

        let api_endpoint = MailApiV1::UserMailboxFolderCreate(self.mailbox_id.clone());
        let mut request = ApiRequest::<CreateUserMailboxFolderResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建用户邮箱文件夹")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建用户邮箱文件夹")
    }
}

impl ApiResponseTrait for CreateUserMailboxFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户邮箱文件夹列表请求
#[derive(Debug, Clone)]
pub struct UserMailboxFolderListRequest {
    config: Arc<Config>,
    mailbox_id: String,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl UserMailboxFolderListRequest {
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

    pub async fn execute(self) -> SDKResult<UserMailboxFolderListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserMailboxFolderListResponse> {
        let api_endpoint = MailApiV1::UserMailboxFolderList(self.mailbox_id.clone());
        let mut request = ApiRequest::<UserMailboxFolderListResponse>::get(api_endpoint.to_url());

        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = &self.page_size {
            request = request.query("page_size", page_size.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取用户邮箱文件夹列表")
    }
}

impl ApiResponseTrait for UserMailboxFolderListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除用户邮箱文件夹请求
#[derive(Debug, Clone)]
pub struct DeleteUserMailboxFolderRequest {
    config: Arc<Config>,
    mailbox_id: String,
    folder_id: String,
}

impl DeleteUserMailboxFolderRequest {
    pub fn new(config: Arc<Config>, mailbox_id: String, folder_id: String) -> Self {
        Self {
            config,
            mailbox_id,
            folder_id,
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteUserMailboxFolderResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteUserMailboxFolderResponse> {
        let api_endpoint = MailApiV1::UserMailboxFolderDelete(self.mailbox_id.clone());
        let mut request = ApiRequest::<DeleteUserMailboxFolderResponse>::delete(api_endpoint.to_url());
        request = request.query("folder_id", &self.folder_id);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除用户邮箱文件夹")
    }
}

impl ApiResponseTrait for DeleteUserMailboxFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 部分更新用户邮箱文件夹请求
#[derive(Debug, Clone)]
pub struct PatchUserMailboxFolderRequest {
    config: Arc<Config>,
    mailbox_id: String,
    folder_id: String,
    body: PatchUserMailboxFolderBody,
}

impl PatchUserMailboxFolderRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>, folder_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            folder_id: folder_id.into(),
            body: PatchUserMailboxFolderBody::default(),
        }
    }

    /// 设置文件夹名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    pub async fn execute(self) -> SDKResult<PatchUserMailboxFolderResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchUserMailboxFolderResponse> {
        let api_endpoint = MailApiV1::UserMailboxFolderPatch(self.mailbox_id.clone());
        let mut request = ApiRequest::<PatchUserMailboxFolderResponse>::patch(api_endpoint.to_url());
        request = request.query("folder_id", &self.folder_id);

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "部分更新用户邮箱文件夹")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "部分更新用户邮箱文件夹")
    }
}

impl ApiResponseTrait for PatchUserMailboxFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_mailbox_folder_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateUserMailboxFolderRequest::new(config, "mailbox_123".to_string())
            .name("收件箱");

        assert_eq!(request.mailbox_id, "mailbox_123");
        assert_eq!(request.body.name, "收件箱");
    }
}

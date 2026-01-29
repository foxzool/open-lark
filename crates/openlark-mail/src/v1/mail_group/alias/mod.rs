//! 邮件组别名模块

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ===== 数据模型 =====

/// 创建邮件组别名请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateMailGroupAliasBody {
    /// 别名邮箱地址
    pub email: String,
}

/// 创建邮件组别名响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateMailGroupAliasResponse {
    /// 别名 ID
    pub alias_id: String,
    /// 别名邮箱地址
    pub email: String,
    /// 创建时间
    pub create_time: String,
}

/// 邮件组别名列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct MailGroupAliasListResponse {
    /// 别名列表
    pub items: Vec<MailGroupAliasItem>,
    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 邮件组别名项目
#[derive(Debug, Clone, Deserialize)]
pub struct MailGroupAliasItem {
    /// 别名 ID
    pub alias_id: String,
    /// 别名邮箱地址
    pub email: String,
    /// 创建时间
    pub create_time: String,
}

/// 删除邮件组别名响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMailGroupAliasResponse {
    /// 是否删除成功
    pub success: bool,
}

/// Alias：邮件组别名资源
#[derive(Clone)]
pub struct Alias {
    config: Arc<Config>,
    group_id: String,
}

impl Alias {
    pub fn new(config: Arc<Config>, group_id: String) -> Self {
        Self { config, group_id }
    }

    /// 创建别名
    pub fn create(&self) -> CreateMailGroupAliasRequest {
        CreateMailGroupAliasRequest::new(self.config.clone(), self.group_id.clone())
    }

    /// 获取别名列表
    pub fn list(&self) -> MailGroupAliasListRequest {
        MailGroupAliasListRequest::new(self.config.clone(), self.group_id.clone())
    }

    /// 删除别名
    pub fn delete(&self, alias_id: impl Into<String>) -> DeleteMailGroupAliasRequest {
        DeleteMailGroupAliasRequest::new(
            self.config.clone(),
            self.group_id.clone(),
            alias_id.into(),
        )
    }
}

/// 创建邮件组别名请求
#[derive(Debug, Clone)]
pub struct CreateMailGroupAliasRequest {
    config: Arc<Config>,
    group_id: String,
    body: CreateMailGroupAliasBody,
}

impl CreateMailGroupAliasRequest {
    pub fn new(config: Arc<Config>, group_id: impl Into<String>) -> Self {
        Self {
            config,
            group_id: group_id.into(),
            body: CreateMailGroupAliasBody::default(),
        }
    }

    /// 设置别名邮箱地址
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.body.email = email.into();
        self
    }

    pub async fn execute(self) -> SDKResult<CreateMailGroupAliasResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateMailGroupAliasResponse> {
        validate_required!(self.body.email.trim(), "别名邮箱地址不能为空");

        let api_endpoint = MailApiV1::MailGroupAliasCreate(self.group_id.clone());
        let mut request = ApiRequest::<CreateMailGroupAliasResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建邮件组别名")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建邮件组别名")
    }
}

impl ApiResponseTrait for CreateMailGroupAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 邮件组别名列表请求
#[derive(Debug, Clone)]
pub struct MailGroupAliasListRequest {
    config: Arc<Config>,
    group_id: String,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl MailGroupAliasListRequest {
    pub fn new(config: Arc<Config>, group_id: impl Into<String>) -> Self {
        Self {
            config,
            group_id: group_id.into(),
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

    pub async fn execute(self) -> SDKResult<MailGroupAliasListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MailGroupAliasListResponse> {
        let api_endpoint = MailApiV1::MailGroupAliasList(self.group_id.clone());
        let mut request = ApiRequest::<MailGroupAliasListResponse>::get(api_endpoint.to_url());

        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = &self.page_size {
            request = request.query("page_size", page_size.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取邮件组别名列表")
    }
}

impl ApiResponseTrait for MailGroupAliasListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除邮件组别名请求
#[derive(Debug, Clone)]
pub struct DeleteMailGroupAliasRequest {
    config: Arc<Config>,
    group_id: String,
    alias_id: String,
}

impl DeleteMailGroupAliasRequest {
    pub fn new(config: Arc<Config>, group_id: String, alias_id: String) -> Self {
        Self {
            config,
            group_id,
            alias_id,
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteMailGroupAliasResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteMailGroupAliasResponse> {
        validate_required!(self.group_id.trim(), "邮件组ID不能为空");
        validate_required!(self.alias_id.trim(), "别名ID不能为空");

        let api_endpoint = MailApiV1::MailGroupAliasDelete(self.group_id.clone(), self.alias_id.clone());
        let request = ApiRequest::<DeleteMailGroupAliasResponse>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除邮件组别名")
    }
}

impl ApiResponseTrait for DeleteMailGroupAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_mail_group_alias_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateMailGroupAliasRequest::new(config, "group_123".to_string())
            .email("alias@example.com");

        assert_eq!(request.group_id, "group_123");
        assert_eq!(request.body.email, "alias@example.com");
    }
}

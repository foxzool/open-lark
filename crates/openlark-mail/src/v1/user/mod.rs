//! 用户邮箱查询模块

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

/// User：用户邮箱资源
#[derive(Clone)]
pub struct User {
    config: Arc<Config>,
}

impl User {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 查询用户邮箱
    pub fn query(&self) -> UserQueryRequest {
        UserQueryRequest::new(self.config.clone())
    }
}

/// 用户邮箱查询响应
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxResponse {
    /// 用户邮箱列表
    pub items: Vec<UserMailboxItem>,
}

/// 用户邮箱项目
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxItem {
    /// 邮箱 ID
    pub mailbox_id: String,

    /// 邮箱地址
    pub email: String,

    /// 是否为默认邮箱
    pub is_default: bool,

    /// 是否已激活
    pub is_activated: bool,
}

/// 用户邮箱查询请求
#[derive(Debug, Clone)]
pub struct UserQueryRequest {
    config: Arc<Config>,
    /// 用户邮箱状态
    status: Option<String>,
    /// 分页标记
    page_token: Option<String>,
    /// 每页数量
    page_size: Option<i32>,
}

impl UserQueryRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            status: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 设置用户邮箱状态
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub async fn execute(self) -> SDKResult<UserMailboxResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserMailboxResponse> {
        let api_endpoint = MailApiV1::UserQuery;
        let mut request = ApiRequest::<UserMailboxResponse>::get(api_endpoint.to_url());

        if let Some(status) = &self.status {
            request = request.query("status", status);
        }
        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = &self.page_size {
            request = request.query("page_size", page_size.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "查询用户邮箱")
    }
}

impl ApiResponseTrait for UserMailboxResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_query_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = UserQueryRequest::new(config.clone())
            .status("activated".to_string())
            .page_token("token_123".to_string())
            .page_size(20);

        assert_eq!(request.status, Some("activated".to_string()));
        assert_eq!(request.page_token, Some("token_123".to_string()));
        assert_eq!(request.page_size, Some(20));
    }
}

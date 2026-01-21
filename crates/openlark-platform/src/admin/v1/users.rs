//! 用户管理 API
//!
//! 提供后台用户管理功能

use crate::{PlatformConfig, LarkClient};
use openlark_core::Result;

/// 用户管理 API
#[derive(Debug, Clone)]
pub struct UsersApi {
    config: PlatformConfig,
    client: LarkClient,
}

impl UsersApi {
    pub fn new(config: PlatformConfig, client: LarkClient) -> Self {
        Self { config, client }
    }

    /// 获取用户列表
    pub fn list(&self) -> ListAdminUsersRequest {
        ListAdminUsersRequest::new(self.config.clone(), self.client.clone())
    }

    /// 禁用用户
    pub fn disable(&self) -> DisableUserRequest {
        DisableUserRequest::new(self.config.clone(), self.client.clone())
    }

    /// 启用用户
    pub fn enable(&self) -> EnableUserRequest {
        EnableUserRequest::new(self.config.clone(), self.client.clone())
    }
}

/// 获取用户列表请求
pub struct ListAdminUsersRequest {
    config: PlatformConfig,
    client: LarkClient,
    page_size: Option<u32>,
}

impl ListAdminUsersRequest {
    fn new(config: PlatformConfig, client: LarkClient) -> Self {
        Self {
            config,
            client,
            page_size: None,
        }
    }

    /// 设置页面大小
    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> Result<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"items": []}))
    }
}

/// 禁用用户请求
pub struct DisableUserRequest {
    config: PlatformConfig,
    client: LarkClient,
    user_id: Option<String>,
}

impl DisableUserRequest {
    fn new(config: PlatformConfig, client: LarkClient) -> Self {
        Self {
            config,
            client,
            user_id: None,
        }
    }

    /// 设置用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> Result<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"success": true}))
    }
}

/// 启用用户请求
pub struct EnableUserRequest {
    config: PlatformConfig,
    client: LarkClient,
    user_id: Option<String>,
}

impl EnableUserRequest {
    fn new(config: PlatformConfig, client: LarkClient) -> Self {
        Self {
            config,
            client,
            user_id: None,
        }
    }

    /// 设置用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> Result<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"success": true}))
    }
}

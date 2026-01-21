//! 用户偏好 V1 API
//!
//! 提供用户偏好 V1 版本的 API 访问

use crate::{UserConfig, LarkClient};

/// 用户偏好 V1 API
#[derive(Debug, Clone)]
pub struct PreferencesV1 {
    /// 客户端配置
    config: UserConfig,
    /// HTTP 客户端
    client: LarkClient,
}

impl PreferencesV1 {
    /// 创建新的用户偏好 V1 实例
    pub fn new(config: UserConfig, client: LarkClient) -> Self {
        Self { config, client }
    }

    /// 获取偏好
    pub fn get(&self) -> GetPreferenceRequest {
        GetPreferenceRequest::new(self.config.clone(), self.client.clone())
    }

    /// 更新偏好
    pub fn update(&self) -> UpdatePreferenceRequest {
        UpdatePreferenceRequest::new(self.config.clone(), self.client.clone())
    }

    /// 删除偏好
    pub fn delete(&self) -> DeletePreferenceRequest {
        DeletePreferenceRequest::new(self.config.clone(), self.client.clone())
    }

    /// 获取所有偏好
    pub fn list(&self) -> ListPreferencesRequest {
        ListPreferencesRequest::new(self.config.clone(), self.client.clone())
    }
}

/// 获取偏好请求
pub struct GetPreferenceRequest {
    config: UserConfig,
    client: LarkClient,
    key: Option<String>,
}

impl GetPreferenceRequest {
    fn new(config: UserConfig, client: LarkClient) -> Self {
        Self {
            config,
            client,
            key: None,
        }
    }

    /// 设置键
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> Result<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"key": "test"}))
    }
}

/// 更新偏好请求
pub struct UpdatePreferenceRequest {
    config: UserConfig,
    client: LarkClient,
    key: Option<String>,
    value: Option<String>,
    category: Option<String>,
}

impl UpdatePreferenceRequest {
    fn new(config: UserConfig, client: LarkClient) -> Self {
        Self {
            config,
            client,
            key: None,
            value: None,
            category: None,
        }
    }

    /// 设置键
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 设置值
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// 设置类别
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> Result<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"success": true}))
    }
}

/// 删除偏好请求
pub struct DeletePreferenceRequest {
    config: UserConfig,
    client: LarkClient,
    key: Option<String>,
}

impl DeletePreferenceRequest {
    fn new(config: UserConfig, client: LarkClient) -> Self {
        Self {
            config,
            client,
            key: None,
        }
    }

    /// 设置键
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> Result<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"success": true}))
    }
}

/// 获取所有偏好请求
pub struct ListPreferencesRequest {
    config: UserConfig,
    client: LarkClient,
    category: Option<String>,
}

impl ListPreferencesRequest {
    fn new(config: UserConfig, client: LarkClient) -> Self {
        Self {
            config,
            client,
            category: None,
        }
    }

    /// 设置类别
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> Result<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"items": []}))
    }
}

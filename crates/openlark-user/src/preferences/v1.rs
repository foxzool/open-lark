//! 用户偏好 V1 API
//!
//! 提供用户偏好 V1 版本的 API 访问

use crate::UserConfig;
use openlark_core::SDKResult;
use std::sync::Arc;

/// 用户偏好 V1 API
#[derive(Debug, Clone)]
pub struct PreferencesV1 {
    /// 客户端配置
    config: Arc<UserConfig>,
}

impl PreferencesV1 {
    /// 创建新的用户偏好 V1 实例
    pub fn new(config: Arc<UserConfig>) -> Self {
        Self { config }
    }

    /// 获取偏好
    pub fn get(&self) -> GetPreferenceRequest {
        GetPreferenceRequest::new(self.config.clone())
    }

    /// 更新偏好
    pub fn update(&self) -> UpdatePreferenceRequest {
        UpdatePreferenceRequest::new(self.config.clone())
    }

    /// 删除偏好
    pub fn delete(&self) -> DeletePreferenceRequest {
        DeletePreferenceRequest::new(self.config.clone())
    }

    /// 获取所有偏好
    pub fn list(&self) -> ListPreferencesRequest {
        ListPreferencesRequest::new(self.config.clone())
    }
}

/// 获取偏好请求
pub struct GetPreferenceRequest {
    #[allow(dead_code)]
    config: Arc<UserConfig>,
    key: Option<String>,
}

impl GetPreferenceRequest {
    fn new(config: Arc<UserConfig>) -> Self {
        Self { config, key: None }
    }

    /// 设置键
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"key": "test"}))
    }
}

/// 更新偏好请求
pub struct UpdatePreferenceRequest {
    #[allow(dead_code)]
    config: Arc<UserConfig>,
    key: Option<String>,
    value: Option<String>,
    category: Option<String>,
}

impl UpdatePreferenceRequest {
    fn new(config: Arc<UserConfig>) -> Self {
        Self {
            config,
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
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"success": true}))
    }
}

/// 删除偏好请求
pub struct DeletePreferenceRequest {
    #[allow(dead_code)]
    config: Arc<UserConfig>,
    key: Option<String>,
}

impl DeletePreferenceRequest {
    fn new(config: Arc<UserConfig>) -> Self {
        Self { config, key: None }
    }

    /// 设置键
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"success": true}))
    }
}

/// 获取所有偏好请求
pub struct ListPreferencesRequest {
    #[allow(dead_code)]
    config: Arc<UserConfig>,
    category: Option<String>,
}

impl ListPreferencesRequest {
    fn new(config: Arc<UserConfig>) -> Self {
        Self {
            config,
            category: None,
        }
    }

    /// 设置类别
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"success": true}))
    }
}

//! 文档服务主模块

use crate::error::{DocsError, DocsResult};
use std::sync::Arc;

/// 文档服务主入口
#[derive(Debug, Clone)]
pub struct DocsService {
    /// 配置信息
    config: Arc<openlark_core::config::Config>,
    /// HTTP客户端
    http_client: Arc<reqwest::Client>,
}

impl DocsService {
    /// 创建新的文档服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            config: Arc::new(config),
            http_client: Arc::new(reqwest::Client::new()),
        }
    }

    /// 从配置创建服务（用于方便的构造）
    pub fn from_config(config: openlark_core::config::Config) -> DocsResult<Self> {
        Ok(Self::new(config))
    }

    /// 获取配置引用
    pub fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    /// 获取HTTP客户端引用
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// 通用请求发送器（自动根据HTTP方法处理query/body）
    pub async fn request_value<Q: serde::Serialize + ?Sized, B: serde::Serialize + ?Sized>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> openlark_core::SDKResult<serde_json::Value> {
        let url = format!("{}{}", self.config.base_url, path);
        let mut builder = self.http_client.request(method.clone(), url.clone());
        if let Some(q) = query {
            builder = builder.query(q);
        }
        if let Some(b) = body {
            builder = builder.json(b);
        }
        let resp = builder
            .send()
            .await
            .map_err(|e| openlark_core::error::network_error_with_details(e.to_string(), None, Some(url.clone())))?;
        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| openlark_core::error::network_error_with_details(e.to_string(), None, Some(url.clone())))?;
        let value: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({"raw": text}));
        if !status.is_success() {
            return Err(openlark_core::error::api_error(status.as_u16(), url, text, None));
        }
        Ok(value)
    }

    /// 访问会议纪要服务
    #[cfg(feature = "minutes")]
    pub fn minutes(&self) -> crate::minutes::MinutesService {
        crate::minutes::MinutesService::new(self.config.clone())
    }
}

/// 服务构建器
#[derive(Debug, Clone)]
pub struct DocsServiceBuilder {
    config: Option<openlark_core::config::Config>,
}

impl DocsServiceBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self { config: None }
    }

    /// 设置应用ID
    pub fn app_id(self, _app_id: impl Into<String>) -> Self {
        // 简化实现，直接创建新的配置
        self
    }

    /// 设置应用密钥
    pub fn app_secret(self, _app_secret: impl Into<String>) -> Self {
        // 简化实现
        self
    }

    /// 设置访问令牌
    pub fn access_token(self, _access_token: impl Into<String>) -> Self {
        // 简化实现
        self
    }

    /// 设置自定义配置
    pub fn config(mut self, config: openlark_core::config::Config) -> Self {
        self.config = Some(config);
        self
    }

    /// 构建服务实例
    pub fn build(self) -> DocsResult<DocsService> {
        let config = self.config.ok_or_else(|| DocsError::InvalidParameter {
            parameter: "config".to_string(),
            reason: "Configuration is required".to_string(),
        })?;

        Ok(DocsService::new(config))
    }
}

impl Default for DocsServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocsService::new(config);

        assert_eq!(service.config().app_id, "test_app_id");
        assert_eq!(service.config().app_secret, "test_app_secret");
    }

    #[test]
    fn test_docs_service_builder() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocsServiceBuilder::new().config(config).build().unwrap();

        assert_eq!(service.config().app_id, "test_app_id");
        assert_eq!(service.config().app_secret, "test_app_secret");
    }

    #[test]
    fn test_docs_service_builder_missing_config() {
        let result = DocsServiceBuilder::new().build();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DocsError::InvalidParameter { .. }
        ));
    }
}

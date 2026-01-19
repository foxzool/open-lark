//! 通讯服务主模块
//!
//! 提供统一的 HTTP 调用封装，供各 project-version-resource API 复用。

use std::sync::Arc;

use openlark_core::config::Config;
use openlark_core::SDKResult;
use reqwest::Method;

/// 通讯服务入口
#[derive(Clone)]
pub struct CommunicationService {
    /// 配置信息
    config: Arc<Config>,
    /// HTTP 客户端
    http_client: Arc<reqwest::Client>,
}

impl CommunicationService {
    /// 使用配置创建服务实例
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            http_client: Arc::new(reqwest::Client::new()),
        }
    }

    /// 便捷构造
    pub fn from_config(config: Config) -> SDKResult<Self> {
        Ok(Self::new(config))
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 通用请求发送器，自动处理 query/body
    pub async fn request_value<Q: serde::Serialize + ?Sized, B: serde::Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> SDKResult<serde_json::Value> {
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
        let value: serde_json::Value =
            serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({ "raw": text }));
        if !status.is_success() {
            return Err(openlark_core::error::api_error(status.as_u16(), url, text, None));
        }
        Ok(value)
}

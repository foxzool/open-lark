//! 通信服务适配器
//!
//! 将openlark-communication服务适配到统一客户端接口。

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::unified::{
    traits::{UnifiedService, ServiceDescriptor, ServiceStatus, ServiceLifecycle, APICall},
    config::{UnifiedConfig, CommunicationConfig},
    error::{UnifiedError, UnifiedResult},
};

/// 通信服务适配器
///
/// 将openlark-communication的功能适配到统一客户端接口。
#[derive(Debug, Clone)]
pub struct CommunicationService {
    /// 服务配置
    config: Option<CommunicationConfig>,
    /// 服务状态
    status: ServiceStatus,
    /// 核心客户端（用于实际API调用）
    core_client: Option<Arc<openlark_core::client::LarkClient>>,
    /// 服务元数据
    metadata: HashMap<String, String>,
}

impl CommunicationService {
    /// 创建新的通信服务适配器
    pub fn new() -> Self {
        Self {
            config: None,
            status: ServiceStatus::Uninitialized,
            core_client: None,
            metadata: HashMap::new(),
        }
    }

    /// 从配置创建服务
    pub fn with_config(mut self, config: CommunicationConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 从核心客户端创建服务
    pub fn with_core_client(mut self, core_client: Arc<openlark_core::client::LarkClient>) -> Self {
        self.core_client = Some(core_client);
        self
    }

    /// 检查服务是否可用
    pub fn is_enabled(&self) -> bool {
        self.config
            .as_ref()
            .map(|config| config.enabled)
            .unwrap_or(false)
    }

    /// 发送文本消息
    pub async fn send_text_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        content: &str,
    ) -> UnifiedResult<MessageSendResult> {
        self.ensure_available()?;

        let core_client = self.core_client.as_ref()
            .ok_or_else(|| UnifiedError::ServiceNotAvailable("core_client".to_string()))?;

        // TODO: 实现具体的消息发送逻辑
        // 这里需要调用openlark-communication的API
        let message_request = serde_json::json!({
            "receive_id": receive_id,
            "receive_id_type": receive_id_type,
            "content": serde_json::json!({
                "text": content
            }),
            "msg_type": "text"
        });

        // 模拟API调用
        tracing::info!("发送文本消息: {:?}", message_request);

        Ok(MessageSendResult {
            message_id: "mock_message_id".to_string(),
            send_time: chrono::Utc::now(),
        })
    }

    /// 发送图片消息
    pub async fn send_image_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        image_key: &str,
    ) -> UnifiedResult<MessageSendResult> {
        self.ensure_available()?;

        let message_request = serde_json::json!({
            "receive_id": receive_id,
            "receive_id_type": receive_id_type,
            "content": serde_json::json!({
                "image_key": image_key
            }),
            "msg_type": "image"
        });

        tracing::info!("发送图片消息: {:?}", message_request);

        Ok(MessageSendResult {
            message_id: "mock_message_id".to_string(),
            send_time: chrono::Utc::now(),
        })
    }

    /// 获取消息列表
    pub async fn list_messages(
        &self,
        container_id: &str,
        container_id_type: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> UnifiedResult<MessageListResult> {
        self.ensure_available()?;

        let list_request = serde_json::json!({
            "container_id": container_id,
            "container_id_type": container_id_type,
            "page_size": page_size,
            "page_token": page_token
        });

        tracing::info!("获取消息列表: {:?}", list_request);

        Ok(MessageListResult {
            messages: vec![],
            page_token: None,
            has_more: false,
        })
    }

    /// 确保服务可用
    fn ensure_available(&self) -> UnifiedResult<()> {
        if !self.is_enabled() {
            return Err(UnifiedError::ServiceNotAvailable("communication".to_string()));
        }

        if self.status != ServiceStatus::Running {
            return Err(UnifiedError::ServiceNotAvailable(
                "communication service not running".to_string(),
            ));
        }

        Ok(())
    }
}

/// 消息发送结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageSendResult {
    /// 消息ID
    pub message_id: String,
    /// 发送时间
    pub send_time: chrono::DateTime<chrono::Utc>,
}

/// 消息列表结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageListResult {
    /// 消息列表
    pub messages: Vec<serde_json::Value>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否还有更多消息
    pub has_more: bool,
}

impl Default for CommunicationService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UnifiedService for CommunicationService {
    type Config = CommunicationConfig;
    type Error = UnifiedError;

    fn name(&self) -> &'static str {
        "communication"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    async fn configure(&mut self, config: Self::Config) -> UnifiedResult<()> {
        if !config.enabled {
            self.status = ServiceStatus::Stopped;
            return Ok(());
        }

        self.config = Some(config);

        // 创建核心客户端
        let core_config = self.config.as_ref().map(|config| {
            openlark_core::config::ConfigBuilder::new()
                .base_url(&config.api_url)
                .timeout(config.timeout)
                .build()
                .unwrap_or_else(|_| openlark_core::config::Config::default())
        });

        if let Some(core_config) = core_config {
            match openlark_core::client::LarkClient::new(
                core_config.app_id.clone(),
                core_config.app_secret.clone(),
            ) {
                Ok(client) => {
                    self.core_client = Some(Arc::new(client));
                    self.status = ServiceStatus::Running;
                    tracing::info!("通信服务配置成功");
                    Ok(())
                }
                Err(e) => {
                    self.status = ServiceStatus::Error;
                    Err(UnifiedError::ConfigurationError(
                        format!("创建核心客户端失败: {}", e),
                    ))
                }
            }
        } else {
            self.status = ServiceStatus::Error;
            Err(UnifiedError::ConfigurationError("通信配置无效".to_string()))
        }
    }

    fn is_available(&self) -> bool {
        self.is_enabled() && self.status == ServiceStatus::Running && self.core_client.is_some()
    }

    fn status(&self) -> ServiceStatus {
        self.status
    }

    fn descriptor(&self) -> ServiceDescriptor {
        let mut descriptor = ServiceDescriptor::new(
            "communication",
            "1.0.0",
            "飞书通信服务，提供消息、联系人、群组等功能",
        )
        .with_tag("messaging")
        .with_tag("real-time")
        .with_dependency("openlark-core");

        if let Some(config) = &self.config {
            descriptor = descriptor
                .with_metadata("api_url", config.api_url.clone())
                .with_metadata("timeout_ms", config.timeout.as_millis().to_string())
                .with_metadata("enabled", config.enabled.to_string());
        }

        descriptor
    }
}

#[async_trait]
impl ServiceLifecycle for CommunicationService {
    async fn start(&mut self) -> SDKResult<()> {
        if let Some(config) = self.config.clone() {
            self.configure(config).await?;
        } else {
            tracing::warn!("通信服务配置未设置，服务将处于未初始化状态");
        }
        Ok(())
    }

    async fn stop(&mut self) -> SDKResult<()> {
        self.status = ServiceStatus::Stopped;
        self.core_client = None;
        tracing::info!("通信服务已停止");
        Ok(())
    }

    async fn health_check(&self) -> SDKResult<bool> {
        Ok(self.is_available())
    }
}

/// 通信服务API调用特征
pub trait CommunicationAPI: APICall<CommunicationRequest, CommunicationResponse> + Send + Sync {}

/// 通信请求类型
#[derive(Debug, Clone)]
pub enum CommunicationRequest {
    /// 发送消息
    SendMessage {
        receive_id: String,
        receive_id_type: String,
        content: serde_json::Value,
        msg_type: String,
    },
    /// 获取消息列表
    ListMessages {
        container_id: String,
        container_id_type: String,
        page_size: Option<u32>,
        page_token: Option<String>,
    },
}

/// 通信响应类型
#[derive(Debug, Clone)]
pub enum CommunicationResponse {
    /// 消息发送结果
    SendResult(MessageSendResult),
    /// 消息列表结果
    ListResult(MessageListResult),
}

/// 通信服务构建器
pub struct CommunicationServiceBuilder {
    config: Option<CommunicationConfig>,
    core_client: Option<Arc<openlark_core::client::LarkClient>>,
}

impl CommunicationServiceBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: None,
            core_client: None,
        }
    }

    /// 设置配置
    pub fn config(mut self, config: CommunicationConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置核心客户端
    pub fn core_client(mut self, core_client: Arc<openlark_core::client::LarkClient>) -> Self {
        self.core_client = Some(core_client);
        self
    }

    /// 构建服务
    pub fn build(self) -> UnifiedResult<CommunicationService> {
        let mut service = CommunicationService::new();

        if let Some(config) = self.config {
            service = service.with_config(config);
        }

        if let Some(core_client) = self.core_client {
            service = service.with_core_client(core_client);
        }

        Ok(service)
    }
}

impl Default for CommunicationServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_communication_service_creation() {
        let service = CommunicationService::new();
        assert_eq!(service.name(), "communication");
        assert_eq!(service.version(), "1.0.0");
    }

    #[test]
    fn test_communication_service_builder() {
        let config = CommunicationConfig::default();
        let service = CommunicationServiceBuilder::new()
            .config(config)
            .build()
            .unwrap();

        assert!(service.is_enabled());
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let mut service = CommunicationService::new();

        // 测试启动
        service.start().await.unwrap();
        // 由于没有配置，服务应该是未初始化状态
        assert_eq!(service.status(), ServiceStatus::Stopped);

        // 测试停止
        service.stop().await.unwrap();
        assert_eq!(service.status(), ServiceStatus::Stopped);
    }

    #[tokio::test]
    async fn test_service_descriptors() {
        let service = CommunicationService::new();
        let descriptor = service.descriptor();

        assert_eq!(descriptor.name, "communication");
        assert_eq!(descriptor.version, "1.0.0");
        assert!(descriptor.tags.contains(&"messaging".to_string()));
    }
}
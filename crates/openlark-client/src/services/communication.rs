//! 📡 通讯服务访问层
//!
//! 提供统一的通讯服务接口，封装底层openlark-communication crate

use crate::{Config, Result, ServiceRegistry};
use std::sync::Arc;

/// 📡 通讯服务 - 统一访问接口
///
/// 包装openlark-communication crate的功能，提供简洁的API
#[derive(Debug)]
pub struct CommunicationService<'a> {
    /// 🔧 客户端配置
    config: &'a Config,
    /// 📋 服务注册表
    registry: &'a ServiceRegistry,
}

impl<'a> CommunicationService<'a> {
    /// 🆕 创建新的通讯服务实例
    pub(crate) fn new(config: &'a Config, registry: &'a ServiceRegistry) -> Self {
        Self { config, registry }
    }

    /// 💬 发送文本消息
    pub async fn send_text_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        content: &str,
    ) -> Result<SendMessageResponse> {
        // TODO: 实现实际的消息发送
        tracing::info!("发送文本消息到 {}: {}", receive_id, content);

        Ok(SendMessageResponse {
            message_id: "mock_message_id".to_string(),
            create_time: chrono::Utc::now().timestamp(),
            msg_type: "text".to_string(),
        })
    }
}

/// 📤 发送消息响应
#[derive(Debug, Clone)]
pub struct SendMessageResponse {
    /// 🏷️ 消息ID
    pub message_id: String,
    /// ⏰ 创建时间
    pub create_time: i64,
    /// 📝 消息类型
    pub msg_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_communication_service_creation() {
        let config = Config::default();
        let config_arc = Arc::new(config);
        let registry = ServiceRegistry::new(&config_arc);
        let service = CommunicationService::new(&config_arc, &registry);

        // 基本创建测试
        assert_eq!(service.config.app_id, "");
    }

    #[tokio::test]
    async fn test_send_text_message() {
        let config = Config::default();
        let config_arc = Arc::new(config);
        let registry = ServiceRegistry::new(&config_arc);
        let service = CommunicationService::new(&config_arc, &registry);

        let result = service
            .send_text_message("test_user", "open_id", "Hello, World!")
            .await;

        assert!(result.is_ok());
        if let Ok(response) = result {
            assert_eq!(response.msg_type, "text");
            assert!(!response.message_id.is_empty());
        }
    }
}

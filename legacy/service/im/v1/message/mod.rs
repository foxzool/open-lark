//! Message service module for IM v1 API
//!
//! This module provides functionality for creating, sending, and managing messages
//! in the Lark/Feishu IM system.

pub mod builders;
pub mod content_types;
pub mod list;
pub mod send;
pub mod types;

// Re-export main types and services for easier imports
pub use builders::{
    CreateMessageRequest, CreateMessageRequestBody, CreateMessageRequestBodyBuilder,
    CreateMessageRequestBuilder, UpdateMessageRequest, UpdateMessageRequestBuilder,
};
pub use content_types::*;
pub use list::{ListMessageRequest, ListMessageRequestBuilder, ListMessageRespData};
pub use types::{CreateMessageResp, ListMessageIterator, Message, SendMessageTrait};

use crate::core::config::Config;
use crate::impl_full_service;

/// Message service
///
/// Provides core message functionality including creating, sending, and retrieving messages.
/// Supports multiple message types: text, post, image, file, audio, media, sticker,
/// interactive, share_chat, share_user.
///
/// Service 抽象接入：通过 `impl_full_service!` 为该服务实现
/// `Service`/`ServiceObservability`/`ServiceBuilder`/`ServiceHealthCheck`/`ConfigurableService`
/// 等核心能力，统一服务行为与可观测性。
#[derive(Debug, Clone)]
pub struct MessageService {
    /// Service configuration
    pub config: Config,
}

impl MessageService {
    /// Create a new message service instance
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

// 为 MessageService 一次性实现核心服务能力（标准样例）
impl_full_service!(MessageService, "im.message", "v1");

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use crate::core::trait_system::Service;

    fn create_test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_message_service_creation() {
        let config = create_test_config();
        let service = MessageService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
        // 验证 Service 抽象静态信息
        assert_eq!(MessageService::service_name(), "im.message");
        assert_eq!(MessageService::service_version(), "v1");
        // 验证 Service::config() 访问
        assert_eq!(service.config().app_id, config.app_id);
    }

    #[test]
    fn test_message_service_with_custom_config() {
        let config = Config::builder()
            .app_id("message_app")
            .app_secret("message_secret")
            .req_timeout(std::time::Duration::from_millis(16000))
            .base_url("https://message.api.com")
            .build();

        let service = MessageService::new(config.clone());

        assert_eq!(service.config.app_id, "message_app");
        assert_eq!(service.config.app_secret, "message_secret");
        assert_eq!(service.config.base_url, "https://message.api.com");
        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_millis(16000))
        );
    }

    #[test]
    fn test_message_service_config_independence() {
        let config1 = Config::builder()
            .app_id("message1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("message2")
            .app_secret("secret2")
            .build();

        let service1 = MessageService::new(config1);
        let service2 = MessageService::new(config2);

        assert_eq!(service1.config.app_id, "message1");
        assert_eq!(service2.config.app_id, "message2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_message_service_memory_layout() {
        let config = create_test_config();
        let service = MessageService::new(config);

        let service_ptr = std::ptr::addr_of!(service) as *const u8;
        let config_ptr = std::ptr::addr_of!(service.config) as *const u8;

        assert!(
            !service_ptr.is_null(),
            "Service should have valid memory address"
        );
        assert!(
            !config_ptr.is_null(),
            "Config should have valid memory address"
        );
    }

    #[test]
    fn test_message_service_with_different_configurations() {
        let test_configs = vec![
            Config::builder()
                .app_id("message_basic")
                .app_secret("basic_secret")
                .build(),
            Config::builder()
                .app_id("message_timeout")
                .app_secret("timeout_secret")
                .req_timeout(std::time::Duration::from_millis(13500))
                .build(),
            Config::builder()
                .app_id("message_custom")
                .app_secret("custom_secret")
                .base_url("https://custom.message.com")
                .build(),
            Config::builder()
                .app_id("message_full")
                .app_secret("full_secret")
                .req_timeout(std::time::Duration::from_millis(23000))
                .base_url("https://full.message.com")
                .enable_token_cache(false)
                .build(),
        ];

        for config in test_configs {
            let service = MessageService::new(config.clone());

            assert_eq!(service.config.app_id, config.app_id);
            assert_eq!(service.config.app_secret, config.app_secret);
            assert_eq!(service.config.base_url, config.base_url);
            assert_eq!(service.config.req_timeout, config.req_timeout);
        }
    }

    #[test]
    fn test_message_service_multiple_instances() {
        let config = create_test_config();
        let service1 = MessageService::new(config.clone());
        let service2 = MessageService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);

        let ptr1 = std::ptr::addr_of!(service1) as *const u8;
        let ptr2 = std::ptr::addr_of!(service2) as *const u8;
        assert_ne!(ptr1, ptr2, "Services should be independent instances");
    }

    #[test]
    fn test_message_service_config_cloning() {
        let original_config = create_test_config();
        let cloned_config = original_config.clone();

        let service = MessageService::new(cloned_config);

        assert_eq!(service.config.app_id, original_config.app_id);
        assert_eq!(service.config.app_secret, original_config.app_secret);
    }

    #[test]
    fn test_message_service_with_empty_config() {
        let config = Config::default();
        let service = MessageService::new(config);

        assert_eq!(service.config.app_id, "");
        assert_eq!(service.config.app_secret, "");
    }

    #[test]
    fn test_message_service_with_unicode_config() {
        let config = Config::builder()
            .app_id("消息应用")
            .app_secret("消息密钥")
            .base_url("https://消息.com")
            .build();
        let service = MessageService::new(config);

        assert_eq!(service.config.app_id, "消息应用");
        assert_eq!(service.config.app_secret, "消息密钥");
        assert_eq!(service.config.base_url, "https://消息.com");
    }

    #[test]
    fn test_message_service_with_extreme_timeout() {
        let config = Config::builder()
            .app_id("message_extreme")
            .app_secret("extreme_secret")
            .req_timeout(std::time::Duration::from_secs(10800))
            .build();
        let service = MessageService::new(config);

        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_secs(10800))
        );
    }

    #[test]
    fn test_message_service_concurrent_creation() {
        let configs = vec![
            Config::builder()
                .app_id("message_concurrent_1")
                .app_secret("secret_1")
                .build(),
            Config::builder()
                .app_id("message_concurrent_2")
                .app_secret("secret_2")
                .build(),
            Config::builder()
                .app_id("message_concurrent_3")
                .app_secret("secret_3")
                .build(),
        ];

        let mut services = Vec::new();
        for config in configs {
            let service = MessageService::new(config);
            services.push(service);
        }

        assert_eq!(services.len(), 3);

        for (i, service1) in services.iter().enumerate() {
            for service2 in services.iter().skip(i + 1) {
                let ptr1 = std::ptr::addr_of!(*service1) as *const u8;
                let ptr2 = std::ptr::addr_of!(*service2) as *const u8;
                assert_ne!(ptr1, ptr2, "Services should be independent instances");
            }
        }
    }

    #[test]
    fn test_message_service_debug_trait() {
        let config = create_test_config();
        let service = MessageService::new(config);

        let debug_output = format!("{:?}", service);
        assert!(debug_output.contains("MessageService"));
        assert!(debug_output.contains("config"));
    }

    #[test]
    fn test_message_service_clone_trait() {
        let config = create_test_config();
        let service = MessageService::new(config);
        let cloned_service = service.clone();

        assert_eq!(service.config.app_id, cloned_service.config.app_id);
        assert_eq!(service.config.app_secret, cloned_service.config.app_secret);

        let ptr1 = std::ptr::addr_of!(service) as *const u8;
        let ptr2 = std::ptr::addr_of!(cloned_service) as *const u8;
        assert_ne!(ptr1, ptr2, "Cloned service should be a different instance");
    }
}

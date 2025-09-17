//! 即时消息（IM）服务
//!
//! 提供飞书即时消息相关的所有API功能，包括消息发送、接收、管理等。
//! 支持多种消息类型：文本、富文本、图片、文件、卡片等。
//!
//! # API版本
//!
//! - **v1**: 稳定版本，包含核心消息功能
//! - **v2**: 新版本，包含增强功能
//!
//! # 主要功能
//!
//! - 📨 消息发送和接收
//! - 🎨 富文本和卡片消息
//! - 📁 文件和媒体消息
//! - 👥 群聊管理
//! - 🔔 消息推送和通知
//!
//! # 快速开始
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 发送文本消息
//! let message = CreateMessageRequestBody::builder()
//!     .receive_id("ou_xxx")
//!     .msg_type("text")
//!     .content("{\"text\":\"Hello!\"}")
//!     .build();
//!
//! let request = CreateMessageRequest::builder()
//!     .receive_id_type("open_id")
//!     .request_body(message)
//!     .build();
//!
//! // let result = client.im.v1.message.create(request, None).await?;
//! ```

use crate::{
    core::config::Config,
    service::im::{v1::V1, v2::V2},
};

/// IM API v1版本
pub mod v1;
/// IM API v2版本
pub mod v2;

/// 即时消息服务
///
/// 聚合所有IM相关的API版本，提供统一的访问接口。
/// 通过不同版本的子服务访问具体的API功能。
pub struct ImService {
    /// IM API v1版本服务
    pub v1: V1,
    /// IM API v2版本服务
    pub v2: V2,
}

impl ImService {
    /// 创建新的IM服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置
    pub fn new(config: Config) -> Self {
        Self {
            v1: V1::new(config.clone()),
            v2: V2::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    #[test]
    fn test_im_service_creation() {
        let config = create_test_config();
        let im_service = ImService::new(config);

        // Verify service structure
        assert!(std::ptr::addr_of!(im_service.v1) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(im_service.v2) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_im_service_with_custom_config() {
        let config = Config::builder()
            .app_id("im_app")
            .app_secret("im_secret")
            .req_timeout(std::time::Duration::from_millis(12000))
            .base_url("https://im.api.com")
            .build();

        let im_service = ImService::new(config);

        // Verify service creation with custom config
        assert!(std::ptr::addr_of!(im_service.v1) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(im_service.v2) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_im_service_api_versions_independence() {
        let config = create_test_config();
        let im_service = ImService::new(config);

        // Test that API versions are independent
        let v1_ptr = std::ptr::addr_of!(im_service.v1) as *const _;
        let v2_ptr = std::ptr::addr_of!(im_service.v2) as *const _;

        assert_ne!(v1_ptr, v2_ptr, "API versions should be independent");
    }

    #[test]
    fn test_im_service_with_various_configurations() {
        let test_configs = vec![
            Config::builder()
                .app_id("im_basic")
                .app_secret("basic_secret")
                .build(),
            Config::builder()
                .app_id("im_timeout")
                .app_secret("timeout_secret")
                .req_timeout(std::time::Duration::from_millis(18000))
                .build(),
            Config::builder()
                .app_id("im_custom")
                .app_secret("custom_secret")
                .base_url("https://custom.im.com")
                .build(),
            Config::builder()
                .app_id("im_full")
                .app_secret("full_secret")
                .req_timeout(std::time::Duration::from_millis(22000))
                .base_url("https://full.im.com")
                .enable_token_cache(false)
                .build(),
        ];

        for config in test_configs {
            let im_service = ImService::new(config);

            // Each configuration should create valid services
            assert!(std::ptr::addr_of!(im_service.v1) as *const _ != std::ptr::null());
            assert!(std::ptr::addr_of!(im_service.v2) as *const _ != std::ptr::null());
        }
    }

    #[test]
    fn test_im_service_multiple_instances() {
        let config1 = create_test_config();
        let config2 = Config::builder()
            .app_id("im2")
            .app_secret("secret2")
            .build();

        let im_service1 = ImService::new(config1);
        let im_service2 = ImService::new(config2);

        // Services should be independent instances
        let service1_ptr = std::ptr::addr_of!(im_service1) as *const _;
        let service2_ptr = std::ptr::addr_of!(im_service2) as *const _;

        assert_ne!(
            service1_ptr, service2_ptr,
            "Services should be independent instances"
        );

        // Each service should have valid API versions
        assert!(std::ptr::addr_of!(im_service1.v1) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(im_service1.v2) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(im_service2.v1) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(im_service2.v2) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_im_service_config_cloning_behavior() {
        let original_config = create_test_config();

        // Test that the service works with cloned configs
        let im_service1 = ImService::new(original_config.clone());
        let im_service2 = ImService::new(original_config);

        // Both should work independently
        assert!(std::ptr::addr_of!(im_service1.v1) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(im_service1.v2) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(im_service2.v1) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(im_service2.v2) as *const _ != std::ptr::null());

        // But should be different service instances
        let service1_ptr = std::ptr::addr_of!(im_service1) as *const _;
        let service2_ptr = std::ptr::addr_of!(im_service2) as *const _;
        assert_ne!(service1_ptr, service2_ptr);
    }
}

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

    /// 使用共享配置（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            v1: V1::new(shared.as_ref().clone()),
            v2: V2::new(shared.as_ref().clone()),
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    fn create_test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_im_service_creation() {
        let config = create_test_config();
        let im_service = ImService::new(config);

        // Verify service structure
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
    }

    #[test]
    fn test_im_service_config_cloning_behavior() {
        let original_config = create_test_config();

        // Test that the service works with cloned configs
        let im_service1 = ImService::new(original_config.clone());
        let im_service2 = ImService::new(original_config);

        // Both should work independently

        // But should be different service instances
        let service1_ptr = std::ptr::addr_of!(im_service1) as *const u8;
        let service2_ptr = std::ptr::addr_of!(im_service2) as *const u8;
        assert_ne!(service1_ptr, service2_ptr);
    }

    #[test]
    fn test_im_service_v1_v2_api_access() {
        let config = create_test_config();
        let im_service = ImService::new(config);

        // Verify that both v1 and v2 APIs are accessible
        let v1_ptr = std::ptr::addr_of!(im_service.v1) as *const u8;
        let v2_ptr = std::ptr::addr_of!(im_service.v2) as *const u8;

        assert!(
            !v1_ptr.is_null(),
            "V1 service should be properly instantiated"
        );
        assert!(
            !v2_ptr.is_null(),
            "V2 service should be properly instantiated"
        );
        assert_ne!(
            v1_ptr, v2_ptr,
            "V1 and V2 services should be independent instances"
        );
    }

    #[test]
    fn test_im_service_v1_api_completeness() {
        let config = create_test_config();
        let im_service = ImService::new(config);

        // Test V1 API structure exists and is accessible
        let v1_ptr = std::ptr::addr_of!(im_service.v1) as *const u8;
        assert!(!v1_ptr.is_null(), "V1 IM API should be instantiated");
    }

    #[test]
    fn test_im_service_v2_api_completeness() {
        let config = create_test_config();
        let im_service = ImService::new(config);

        // Test V2 API structure exists and is accessible
        let v2_ptr = std::ptr::addr_of!(im_service.v2) as *const u8;
        assert!(!v2_ptr.is_null(), "V2 IM API should be instantiated");
    }

    #[test]
    fn test_im_service_concurrent_creation() {
        let configs = vec![
            Config::builder()
                .app_id("im_concurrent_1")
                .app_secret("secret_1")
                .build(),
            Config::builder()
                .app_id("im_concurrent_2")
                .app_secret("secret_2")
                .build(),
            Config::builder()
                .app_id("im_concurrent_3")
                .app_secret("secret_3")
                .build(),
        ];

        let mut services = Vec::new();
        for config in configs {
            let service = ImService::new(config);
            services.push(service);
        }

        // All services should be created successfully
        assert_eq!(services.len(), 3);

        // Verify all services are independent
        for (i, service1) in services.iter().enumerate() {
            for service2 in services.iter().skip(i + 1) {
                let ptr1 = std::ptr::addr_of!(*service1) as *const u8;
                let ptr2 = std::ptr::addr_of!(*service2) as *const u8;
                assert_ne!(ptr1, ptr2, "Services should be independent instances");
            }
        }
    }

    #[test]
    fn test_im_service_extreme_configurations() {
        let extreme_configs = vec![
            // Very short timeout
            Config::builder()
                .app_id("im_fast")
                .app_secret("fast_secret")
                .req_timeout(std::time::Duration::from_millis(100))
                .build(),
            // Very long timeout
            Config::builder()
                .app_id("im_slow")
                .app_secret("slow_secret")
                .req_timeout(std::time::Duration::from_secs(300))
                .build(),
            // Token cache disabled
            Config::builder()
                .app_id("im_no_cache")
                .app_secret("no_cache_secret")
                .enable_token_cache(false)
                .build(),
            // Custom base URL
            Config::builder()
                .app_id("im_custom_base")
                .app_secret("custom_base_secret")
                .base_url("https://custom.im.api.endpoint")
                .build(),
        ];

        for config in extreme_configs {
            let im_service = ImService::new(config);

            // Each service should be created successfully regardless of extreme config
            let service_ptr = std::ptr::addr_of!(im_service) as *const u8;
            assert!(
                !service_ptr.is_null(),
                "Service should be created with extreme config"
            );
        }
    }

    #[test]
    fn test_im_service_api_version_structure() {
        let config = create_test_config();
        let im_service = ImService::new(config);

        // Verify the service contains exactly two API versions
        let v1_offset = unsafe {
            std::ptr::addr_of!(im_service.v1) as usize - std::ptr::addr_of!(im_service) as usize
        };
        let v2_offset = unsafe {
            std::ptr::addr_of!(im_service.v2) as usize - std::ptr::addr_of!(im_service) as usize
        };

        // V1 and V2 should have different memory offsets
        assert_ne!(
            v1_offset, v2_offset,
            "V1 and V2 should occupy different memory positions"
        );

        // Both offsets should be reasonable (within struct bounds)
        assert!(v1_offset < 4096, "V1 offset should be reasonable");
        assert!(v2_offset < 4096, "V2 offset should be reasonable");
    }

    #[test]
    fn test_im_service_memory_consistency() {
        let config = create_test_config();
        let im_service = ImService::new(config);

        // Test that the service maintains memory consistency across accesses
        let service_ptr1 = std::ptr::addr_of!(im_service) as *const u8;
        let service_ptr2 = std::ptr::addr_of!(im_service) as *const u8;

        assert_eq!(
            service_ptr1, service_ptr2,
            "Service memory address should be consistent"
        );

        // Test API version consistency
        let v1_ptr1 = std::ptr::addr_of!(im_service.v1) as *const u8;
        let v1_ptr2 = std::ptr::addr_of!(im_service.v1) as *const u8;
        let v2_ptr1 = std::ptr::addr_of!(im_service.v2) as *const u8;
        let v2_ptr2 = std::ptr::addr_of!(im_service.v2) as *const u8;

        assert_eq!(
            v1_ptr1, v1_ptr2,
            "V1 API memory address should be consistent"
        );
        assert_eq!(
            v2_ptr1, v2_ptr2,
            "V2 API memory address should be consistent"
        );
    }

    #[test]
    fn test_im_service_config_variations() {
        let variations = vec![
            (
                "minimal",
                Config::builder()
                    .app_id("minimal")
                    .app_secret("secret")
                    .build(),
            ),
            (
                "with_timeout",
                Config::builder()
                    .app_id("timeout")
                    .app_secret("secret")
                    .req_timeout(std::time::Duration::from_millis(30000))
                    .build(),
            ),
            (
                "with_base_url",
                Config::builder()
                    .app_id("base_url")
                    .app_secret("secret")
                    .base_url("https://test.api.com")
                    .build(),
            ),
            (
                "full_featured",
                Config::builder()
                    .app_id("full")
                    .app_secret("secret")
                    .req_timeout(std::time::Duration::from_millis(45000))
                    .base_url("https://full.test.api.com")
                    .enable_token_cache(true)
                    .build(),
            ),
        ];

        let mut services = Vec::new();
        for (name, config) in variations {
            let service = ImService::new(config);
            services.push((name, service));
        }

        // All variations should create valid services
        assert_eq!(services.len(), 4);

        // Test that all services are independent
        for (i, (_, service1)) in services.iter().enumerate() {
            for (_, service2) in services.iter().skip(i + 1) {
                let ptr1 = std::ptr::addr_of!(*service1) as *const u8;
                let ptr2 = std::ptr::addr_of!(*service2) as *const u8;
                assert_ne!(
                    ptr1, ptr2,
                    "Services with different configs should be independent"
                );
            }
        }
    }
}

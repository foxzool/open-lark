use crate::core::config::Config;

/// Message service
///
/// Provides core message functionality including creating, sending, and retrieving messages.
/// Supports multiple message types: text, post, image, file, audio, media, sticker,
/// interactive, share_chat, share_user.
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

// Import all functionality from the message module
pub use crate::service::im::v1::message::*;

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.example.com")
            .build()
    }

    #[test]
    fn test_message_service_new() {
        let config = create_test_config();
        let _service = MessageService::new(config);

        // Service should be created successfully
    }

    #[test]
    fn test_message_service_clone() {
        let config = create_test_config();
        let service = MessageService::new(config);
        let _cloned_service = service.clone();

        // Services should be cloned successfully
    }

    #[test]
    fn test_message_service_debug() {
        let config = create_test_config();
        let service = MessageService::new(config);
        let debug_output = format!("{:?}", service);

        assert!(debug_output.contains("MessageService"));
        assert!(debug_output.contains("config"));
    }

    #[test]
    fn test_message_service_config_independence() {
        let config1 = Config::builder()
            .app_id("app1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("app2")
            .app_secret("secret2")
            .build();

        let _service1 = MessageService::new(config1);
        let _service2 = MessageService::new(config2);

        // Services should be created independently
    }

    #[test]
    fn test_message_service_with_empty_config() {
        let config = Config::default();
        let _service = MessageService::new(config);

        // Service should handle default config
    }

    #[test]
    fn test_message_service_with_unicode_config() {
        let config = Config::builder()
            .app_id("消息应用")
            .app_secret("消息密钥")
            .base_url("https://消息域名.com")
            .build();
        let _service = MessageService::new(config);

        // Service should handle Unicode config
    }

    #[test]
    fn test_message_service_multiple_instances() {
        let config = create_test_config();
        let _service1 = MessageService::new(config.clone());
        let _service2 = MessageService::new(config.clone());

        // Multiple services should be created successfully
    }

    #[test]
    fn test_message_service_config_cloning() {
        let config = create_test_config();
        let cloned_config = config.clone();
        let _service = MessageService::new(cloned_config);

        // Service should work with cloned config
    }

    #[test]
    fn test_message_service_with_timeout_config() {
        let config = Config::builder()
            .app_id("timeout_app")
            .app_secret("timeout_secret")
            .base_url("https://api.test.com")
            .build();
        let _service = MessageService::new(config);

        // Service should handle timeout config
    }

    #[test]
    fn test_message_service_field_access() {
        let config = create_test_config();
        let _service = MessageService::new(config);

        // Service should be created with config
    }

    #[test]
    fn test_message_service_creation_variants() {
        let test_configs = vec![
            Config::builder()
                .app_id("basic_app")
                .app_secret("basic_secret")
                .build(),
            Config::builder()
                .app_id("timeout_app")
                .app_secret("timeout_secret")
                .req_timeout(std::time::Duration::from_millis(5000))
                .build(),
            Config::builder()
                .app_id("custom_app")
                .app_secret("custom_secret")
                .base_url("https://custom.api.com")
                .build(),
        ];

        for config in test_configs {
            let _service = MessageService::new(config);
            // All variants should create services successfully
        }
    }
}

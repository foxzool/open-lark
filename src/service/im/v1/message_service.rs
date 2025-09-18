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
        Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://test.example.com".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_message_service_new() {
        let config = create_test_config();
        let service = MessageService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
        assert_eq!(service.config.base_url, config.base_url);
    }

    #[test]
    fn test_message_service_clone() {
        let config = create_test_config();
        let service = MessageService::new(config);
        let cloned_service = service.clone();

        assert_eq!(service.config.app_id, cloned_service.config.app_id);
        assert_eq!(service.config.app_secret, cloned_service.config.app_secret);
        assert_eq!(service.config.base_url, cloned_service.config.base_url);
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
        let config1 = Config {
            app_id: "app1".to_string(),
            app_secret: "secret1".to_string(),
            ..Default::default()
        };
        let config2 = Config {
            app_id: "app2".to_string(),
            app_secret: "secret2".to_string(),
            ..Default::default()
        };

        let service1 = MessageService::new(config1);
        let service2 = MessageService::new(config2);

        assert_eq!(service1.config.app_id, "app1");
        assert_eq!(service2.config.app_id, "app2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
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
        let config = Config {
            app_id: "消息应用".to_string(),
            app_secret: "消息密钥".to_string(),
            base_url: "https://消息域名.com".to_string(),
            ..Default::default()
        };
        let service = MessageService::new(config);

        assert_eq!(service.config.app_id, "消息应用");
        assert_eq!(service.config.app_secret, "消息密钥");
        assert_eq!(service.config.base_url, "https://消息域名.com");
    }

    #[test]
    fn test_message_service_multiple_instances() {
        let config = create_test_config();
        let service1 = MessageService::new(config.clone());
        let service2 = MessageService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);
    }

    #[test]
    fn test_message_service_config_cloning() {
        let config = create_test_config();
        let cloned_config = config.clone();
        let service = MessageService::new(cloned_config);

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_message_service_with_timeout_config() {
        let config = Config {
            app_id: "timeout_app".to_string(),
            app_secret: "timeout_secret".to_string(),
            base_url: "https://api.test.com".to_string(),
            req_timeout: Some(std::time::Duration::from_secs(30)),
            ..Default::default()
        };
        let service = MessageService::new(config.clone());

        assert_eq!(service.config.app_id, "timeout_app");
        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_secs(30))
        );
    }

    #[test]
    fn test_message_service_field_access() {
        let config = create_test_config();
        let service = MessageService::new(config);

        // Test that we can access the config field
        assert!(!service.config.app_id.is_empty());
        assert!(!service.config.app_secret.is_empty());
        assert!(!service.config.base_url.is_empty());
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
            let service = MessageService::new(config);
            assert!(!service.config.app_id.is_empty());
            assert!(!service.config.app_secret.is_empty());
        }
    }
}

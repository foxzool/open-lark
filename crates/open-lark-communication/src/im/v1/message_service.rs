use open_lark_core::core::config::Config;

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
// Import all functionality from the message module
pub use crate::im::v1::message::*;
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use open_lark_core::core::config::Config;
    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.example.com")
            .build()
    #[test]
    fn test_message_service_new() {
        let config = create_test_config();
        let _service = MessageService::new(config);
        // Service should be created successfully
    fn test_message_service_clone() {
        let service = MessageService::new(config);
        let _cloned_service = service.clone();
        // Services should be cloned successfully
    fn test_message_service_debug() {
        let debug_output = format!("{:?}", service);
        assert!(debug_output.contains("MessageService"));
        assert!(debug_output.contains("config"));
    fn test_message_service_config_independence() {
        let config1 = Config::builder()
            .app_id("app1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("app2")
            .app_secret("secret2")
        let _service1 = MessageService::new(config1);
        let _service2 = MessageService::new(config2);
        // Services should be created independently
    fn test_message_service_with_empty_config() {
        let config = Config::default();
        // Service should handle default config
    fn test_message_service_with_unicode_config() {
        let config = Config::builder()
            .app_id("消息应用")
            .app_secret("消息密钥")
            .base_url("https://消息域名.com")
        // Service should handle Unicode config
    fn test_message_service_multiple_instances() {
        let _service1 = MessageService::new(config.clone());
        let _service2 = MessageService::new(config.clone());
        // Multiple services should be created successfully
    fn test_message_service_config_cloning() {
        let cloned_config = config.clone();
        let _service = MessageService::new(cloned_config);
        // Service should work with cloned config
    fn test_message_service_with_timeout_config() {
            .app_id("timeout_app")
            .app_secret("timeout_secret")
            .base_url("https://api.test.com")
        // Service should handle timeout config
    fn test_message_service_field_access() {
        // Service should be created with config
    fn test_message_service_creation_variants() {
        let test_configs = vec![
            Config::builder()
                .app_id("basic_app")
                .app_secret("basic_secret")
                .build(),
                .app_id("timeout_app")
                .app_secret("timeout_secret")
                .req_timeout(std::time::Duration::from_millis(5000))
                .app_id("custom_app")
                .app_secret("custom_secret")
                .base_url("https://custom.api.com")
        ];
        for config in test_configs {
            let _service = MessageService::new(config);
            // All variants should create services successfully
        }

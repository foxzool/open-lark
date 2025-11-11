use serde::{Deserialize, Serialize};

use crate::{
    cache::QuickCache,
    config::Config,
    constants::{APPLY_APP_TICKET_PATH, APP_TICKET_KEY_PREFIX},
    SDKResult,
};

#[derive(Debug)]
pub struct AppTicketManager {
    pub cache: QuickCache<String>,
}

impl Default for AppTicketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AppTicketManager {
    pub fn new() -> Self {
        Self {
            cache: QuickCache::new(),
        }
    }

    pub fn set(&mut self, app_id: &str, value: &str, expire_time: i32) {
        let key = app_ticket_key(app_id);
        self.cache.set(&key, value.to_string(), expire_time);
    }

    pub async fn get(&self, config: &Config) -> Option<String> {
        let key = app_ticket_key(&config.app_id);
        match self.cache.get(&key) {
            None => None,
            Some(ticket) => {
                if ticket.is_empty() {
                    apply_app_ticket(config).await.ok();
                }

                Some(ticket)
            }
        }
    }
}

fn app_ticket_key(app_id: &str) -> String {
    format!("{APP_TICKET_KEY_PREFIX}-{app_id}")
}

pub async fn apply_app_ticket(config: &Config) -> SDKResult<()> {
    let url = format!("{}{}", config.base_url, APPLY_APP_TICKET_PATH);

    let body = ResendAppTicketReq {
        app_id: config.app_id.clone(),
        app_secret: config.app_secret.clone(),
    };

    let _response = config.http_client.post(&url).json(&body).send().await?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct ResendAppTicketReq {
    app_id: String,
    app_secret: String,
}

// #[derive(Serialize, Deserialize)]
// struct ResendAppTicketResp {
//     #[serde(skip)]
//     api_resp:
//     #[serde(flatten)]
//     code_error: ErrorResponse,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::time::Duration;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.api.com")
            .build()
    }

    #[test]
    fn test_app_ticket_manager_creation() {
        let manager = AppTicketManager::new();

        // Check that manager is created properly
        assert!(format!("{:?}", manager).contains("AppTicketManager"));
    }

    #[test]
    fn test_app_ticket_manager_default() {
        let manager = AppTicketManager::default();

        // Test default implementation
        assert!(format!("{:?}", manager).contains("AppTicketManager"));
    }

    #[test]
    fn test_app_ticket_key_generation() {
        let app_id = "test_app_123";
        let key = app_ticket_key(app_id);

        assert!(key.contains("test_app_123"));
        assert!(key.starts_with("app_ticket"));
    }

    #[test]
    fn test_set_and_get_ticket() {
        let mut manager = AppTicketManager::new();
        let app_id = "test_app";
        let ticket_value = "test_ticket_value";

        // Set ticket
        manager.set(app_id, ticket_value, 60);

        // Check cache directly
        let key = app_ticket_key(app_id);
        let cached_ticket = manager.cache.get(&key);
        assert_eq!(cached_ticket, Some(ticket_value.to_string()));
    }

    #[test]
    fn test_set_ticket_with_different_app_ids() {
        let mut manager = AppTicketManager::new();

        // Set tickets for different apps
        manager.set("app1", "ticket1", 60);
        manager.set("app2", "ticket2", 60);

        // Verify both are stored separately
        let key1 = app_ticket_key("app1");
        let key2 = app_ticket_key("app2");

        assert_eq!(manager.cache.get(&key1), Some("ticket1".to_string()));
        assert_eq!(manager.cache.get(&key2), Some("ticket2".to_string()));
    }

    #[test]
    fn test_ticket_expiration() {
        let mut manager = AppTicketManager::new();
        let app_id = "test_app";

        // Set ticket with very short expiration
        manager.set(app_id, "short_lived_ticket", 1);

        // Verify it exists initially
        let key = app_ticket_key(app_id);
        assert!(manager.cache.get(&key).is_some());

        // Wait for expiration
        std::thread::sleep(Duration::from_secs(2));

        // Should be expired and removed
        assert!(manager.cache.get(&key).is_none());
    }

    #[test]
    fn test_overwrite_existing_ticket() {
        let mut manager = AppTicketManager::new();
        let app_id = "test_app";

        // Set initial ticket
        manager.set(app_id, "initial_ticket", 60);

        // Overwrite with new ticket
        manager.set(app_id, "updated_ticket", 60);

        // Verify only new ticket exists
        let key = app_ticket_key(app_id);
        assert_eq!(manager.cache.get(&key), Some("updated_ticket".to_string()));
    }

    #[test]
    fn test_empty_app_id() {
        let mut manager = AppTicketManager::new();

        // Test with empty app_id
        manager.set("", "ticket", 60);

        let key = app_ticket_key("");
        assert!(manager.cache.get(&key).is_some());
    }

    #[test]
    fn test_special_characters_in_app_id() {
        let mut manager = AppTicketManager::new();
        let special_app_id = "app-id_with.special@chars";

        manager.set(special_app_id, "special_ticket", 60);

        let key = app_ticket_key(special_app_id);
        assert_eq!(manager.cache.get(&key), Some("special_ticket".to_string()));
    }

    #[test]
    fn test_zero_expiration_time() {
        let mut manager = AppTicketManager::new();
        let app_id = "test_app";

        // Set ticket with zero expiration (should expire immediately or very soon)
        manager.set(app_id, "zero_exp_ticket", 0);

        // Sleep briefly to allow expiration
        std::thread::sleep(Duration::from_millis(10));

        let key = app_ticket_key(app_id);
        // May or may not be present depending on timing, but shouldn't crash
        let _ = manager.cache.get(&key);
    }

    #[test]
    fn test_very_long_ticket_value() {
        let mut manager = AppTicketManager::new();
        let app_id = "test_app";
        let long_ticket = "a".repeat(10000); // 10KB ticket

        manager.set(app_id, &long_ticket, 60);

        let key = app_ticket_key(app_id);
        assert_eq!(manager.cache.get(&key), Some(long_ticket));
    }

    #[tokio::test]
    async fn test_get_with_cached_ticket() {
        let mut manager = AppTicketManager::new();
        let config = create_test_config();

        // Pre-populate cache with ticket
        manager.set(&config.app_id, "cached_ticket", 60);

        let result = manager.get(&config).await;
        assert_eq!(result, Some("cached_ticket".to_string()));
    }

    #[tokio::test]
    async fn test_get_with_empty_ticket() {
        let mut manager = AppTicketManager::new();
        let config = create_test_config();

        // Set empty ticket in cache
        manager.set(&config.app_id, "", 60);

        // This will try to apply for new ticket, but return the empty one
        let result = manager.get(&config).await;
        assert_eq!(result, Some("".to_string()));
    }

    #[tokio::test]
    async fn test_get_without_cached_ticket() {
        let manager = AppTicketManager::new();
        let config = create_test_config();

        // No ticket in cache
        let result = manager.get(&config).await;
        assert_eq!(result, None);
    }

    #[test]
    fn test_resend_app_ticket_req_serialization() {
        let req = ResendAppTicketReq {
            app_id: "test_app".to_string(),
            app_secret: "test_secret".to_string(),
        };

        // Test JSON serialization
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("test_secret"));

        // Test JSON deserialization
        let deserialized: ResendAppTicketReq = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.app_id, "test_app");
        assert_eq!(deserialized.app_secret, "test_secret");
    }

    #[test]
    fn test_multiple_managers_independence() {
        let mut manager1 = AppTicketManager::new();
        let mut manager2 = AppTicketManager::new();

        manager1.set("app1", "ticket1", 60);
        manager2.set("app2", "ticket2", 60);

        // Each manager should only have its own ticket
        let key1 = app_ticket_key("app1");
        let key2 = app_ticket_key("app2");

        assert_eq!(manager1.cache.get(&key1), Some("ticket1".to_string()));
        assert_eq!(manager1.cache.get(&key2), None);

        assert_eq!(manager2.cache.get(&key2), Some("ticket2".to_string()));
        assert_eq!(manager2.cache.get(&key1), None);
    }

    #[test]
    fn test_app_ticket_key_format() {
        let test_cases = vec![
            ("simple_app", "app_ticket-simple_app"),
            ("app-with-dashes", "app_ticket-app-with-dashes"),
            ("app_with_underscores", "app_ticket-app_with_underscores"),
            ("123numeric", "app_ticket-123numeric"),
        ];

        for (app_id, _expected_suffix) in test_cases {
            let key = app_ticket_key(app_id);
            assert!(key.ends_with(&format!("-{}", app_id)));
            // Verify it starts with the expected prefix
            assert!(key.starts_with("app_ticket"));
        }
    }
}

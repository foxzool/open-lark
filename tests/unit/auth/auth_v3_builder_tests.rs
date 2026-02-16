//! Auth v3 Builder 单元测试
//!
//! 测试 Auth V3 API 的 Builder 模式

use open_lark::{
    core::{config::Config, constants::AppType},
    openlark_auth::AuthService,
};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;

/// 创建测试配置
fn create_test_config() -> Config {
    Config {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
        app_type: AppType::SelfBuild,
        base_url: "https://open.feishu.cn".to_string(),
        http_client: reqwest::Client::new(),
        enable_token_cache: true,
        req_timeout: Some(Duration::from_secs(30)),
        header: HashMap::new(),
        token_manager: Arc::new(Mutex::new(
            open_lark::core::token_manager::TokenManager::new(),
        )),
        app_ticket_manager: Arc::new(Mutex::new(
            open_lark::core::app_ticket_manager::AppTicketManager::new(),
        )),
    }
}

#[cfg(test)]
mod auth_service_tests {
    use super::*;

    #[test]
    fn test_auth_service_creation() {
        let config = create_test_config();
        let auth_service = AuthService::new(config);
        let _v3 = auth_service.v3();
    }

    #[test]
    fn test_app_access_token_internal_builder() {
        let config = create_test_config();
        let auth_service = AuthService::new(config);
        let _builder = auth_service.v3().app_access_token_internal();
    }

    #[test]
    fn test_tenant_access_token_internal_builder() {
        let config = create_test_config();
        let auth_service = AuthService::new(config);
        let _builder = auth_service.v3().tenant_access_token_internal();
    }

    #[test]
    fn test_app_access_token_store_builder() {
        let config = create_test_config();
        let auth_service = AuthService::new(config);
        let _builder = auth_service.v3().app_access_token();
    }

    #[test]
    fn test_tenant_access_token_store_builder() {
        let config = create_test_config();
        let auth_service = AuthService::new(config);
        let _builder = auth_service.v3().tenant_access_token();
    }

    #[test]
    fn test_app_ticket_resend_builder() {
        let config = create_test_config();
        let auth_service = AuthService::new(config);
        let _builder = auth_service.v3().app_ticket_resend();
    }
}

#[cfg(test)]
mod builder_chain_tests {
    use super::*;

    #[test]
    fn test_app_access_token_internal_chain() {
        let config = create_test_config();
        let auth_service = AuthService::new(config);
        let _builder = auth_service
            .v3()
            .app_access_token_internal()
            .app_id("test_app")
            .app_secret("test_secret");
    }

    #[test]
    fn test_tenant_access_token_internal_chain() {
        let config = create_test_config();
        let auth_service = AuthService::new(config);
        let _builder = auth_service
            .v3()
            .tenant_access_token_internal()
            .app_id("test_app")
            .app_secret("test_secret");
    }

    #[test]
    fn test_app_access_token_store_chain() {
        let config = create_test_config();
        let auth_service = AuthService::new(config);
        let _builder = auth_service
            .v3()
            .app_access_token()
            .app_access_token("test_token")
            .tenant_key("test_tenant");
    }

    #[test]
    fn test_tenant_access_token_store_chain() {
        let config = create_test_config();
        let auth_service = AuthService::new(config);
        let _builder = auth_service
            .v3()
            .tenant_access_token()
            .app_access_token("test_token")
            .tenant_key("test_tenant");
    }

    #[test]
    fn test_app_ticket_resend_chain() {
        let config = create_test_config();
        let auth_service = AuthService::new(config);
        let _builder = auth_service
            .v3()
            .app_ticket_resend()
            .app_id("test_app")
            .app_ticket("test_ticket");
    }
}

#[cfg(test)]
mod config_propagation_tests {
    use super::*;

    #[test]
    fn test_custom_base_url() {
        let mut config = create_test_config();
        config.base_url = "https://custom.feishu.cn".to_string();
        let auth_service = AuthService::new(config);
        let _v3 = auth_service.v3();
    }

    #[test]
    fn test_token_cache_disabled() {
        let mut config = create_test_config();
        config.enable_token_cache = false;
        let auth_service = AuthService::new(config);
        let _v3 = auth_service.v3();
    }

    #[test]
    fn test_custom_timeout() {
        let mut config = create_test_config();
        config.req_timeout = Some(Duration::from_secs(60));
        let auth_service = AuthService::new(config);
        let _v3 = auth_service.v3();
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_app_id() {
        let mut config = create_test_config();
        config.app_id = String::new();
        let auth_service = AuthService::new(config);
        let _builder = auth_service.v3().app_access_token_internal();
    }

    #[test]
    fn test_long_app_id() {
        let mut config = create_test_config();
        config.app_id = "a".repeat(1000);
        let auth_service = AuthService::new(config);
        let _builder = auth_service.v3().app_access_token_internal();
    }

    #[test]
    fn test_special_characters_in_app_id() {
        let mut config = create_test_config();
        config.app_id = "app🚀测试-123".to_string();
        let auth_service = AuthService::new(config);
        let _builder = auth_service.v3().app_access_token_internal();
    }

    #[test]
    fn test_multiple_auth_service_instances() {
        let config1 = create_test_config();
        let mut config2 = create_test_config();
        config2.app_id = "different_app".to_string();

        let auth1 = AuthService::new(config1);
        let auth2 = AuthService::new(config2);

        let _builder1 = auth1.v3().app_access_token_internal();
        let _builder2 = auth2.v3().app_access_token_internal();
    }
}

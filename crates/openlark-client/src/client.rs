//! Default LarkClient implementation
//!
//! This module provides the default implementation of the LarkClient trait,
//! offering service aggregation and basic client functionality.

use std::sync::Arc;

use openlark_core::{config::Config, constants::AppType};

use crate::traits::{LarkClient, ClientBuilder};

/// Default implementation of LarkClient
///
/// This client aggregates various services and provides a unified interface
/// for accessing all Lark functionality. It includes service registration,
/// configuration management, and basic client operations.
#[derive(Debug)]
pub struct DefaultLarkClient {
    /// Client configuration
    config: Config,
    /// Shared configuration for performance optimization
    shared_config: Arc<Config>,
    /// Service registry for managing services
    services: crate::registry::DefaultServiceRegistry,
}

impl DefaultLarkClient {
    /// Create a new DefaultLarkClient with the given configuration
    pub fn new(config: Config) -> Self {
        let shared_config = Arc::new(config.clone());
        Self::new_from_shared(shared_config)
    }

    /// Create a new DefaultLarkClient with shared configuration
    pub fn new_from_shared(shared_config: Arc<Config>) -> Self {
        let config = shared_config.as_ref().clone();
        let mut services = crate::registry::DefaultServiceRegistry::new();

        // 初始化所有启用的服务
        #[cfg(feature = "all-services")]
        crate::services::ServiceManager::initialize_services(&config, &shared_config, &mut services);

        #[cfg(not(feature = "all-services"))]
        {
            // 根据具体功能标志初始化服务
            crate::services::ServiceManager::initialize_services(&config, &shared_config, &mut services);
        }

        Self {
            config,
            shared_config,
            services,
        }
    }

    /// Get the shared configuration reference
    pub fn shared_config(&self) -> &Arc<Config> {
        &self.shared_config
    }

    /// Get access to the service registry
    pub fn services(&self) -> &crate::registry::DefaultServiceRegistry {
        &self.services
    }

    /// Get mutable access to the service registry
    pub fn services_mut(&mut self) -> &mut crate::registry::DefaultServiceRegistry {
        &mut self.services
    }

    /// Get service accessors for backward compatibility
    pub fn service_accessors(&self) -> &crate::registry::DefaultServiceRegistry {
        &self.services
    }

    /// Create client using compatible builder
    pub fn builder(app_id: impl Into<String>, app_secret: impl Into<String>) -> crate::accessors::CompatibleClientBuilder {
        crate::accessors::CompatibleClientBuilder::new(app_id, app_secret)
    }
}

impl Clone for DefaultLarkClient {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            shared_config: Arc::clone(&self.shared_config),
            services: crate::registry::DefaultServiceRegistry::new(),
        }
    }
}

impl LarkClient for DefaultLarkClient {
    fn config(&self) -> &Config {
        &self.config
    }

    fn new(config: Config) -> Self {
        Self::new(config)
    }

    fn new_from_shared(shared_config: Arc<Config>) -> Self {
        Self::new_from_shared(shared_config)
    }
}

/// Builder for creating DefaultLarkClient instances
#[derive(Debug)]
pub struct DefaultLarkClientBuilder {
    config: Option<Config>,
    shared_config: Option<Arc<Config>>,
    services: Option<crate::registry::DefaultServiceRegistry>,
}

impl DefaultLarkClientBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            config: None,
            shared_config: None,
            services: None,
        }
    }

    /// Set the configuration
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    /// Set shared configuration
    pub fn with_shared_config(mut self, shared_config: Arc<Config>) -> Self {
        self.shared_config = Some(shared_config);
        self
    }

    /// Set custom service registry
    pub fn with_services(mut self, services: crate::registry::DefaultServiceRegistry) -> Self {
        self.services = Some(services);
        self
    }

    /// Build the client instance
    pub fn build(self) -> DefaultLarkClient {
        let config = self.config.unwrap_or_else(|| {
            Config::builder()
                .app_id("app_id")
                .app_secret("app_secret")
                .app_type(AppType::SelfBuild)
                .build()
        });

        let shared_config = self.shared_config.unwrap_or_else(|| Arc::new(config.clone()));

        let services = self.services.unwrap_or_default();

        DefaultLarkClient {
            config,
            shared_config,
            services,
        }
    }

    /// Build the client instance with shared config
    pub fn build_with_shared_config(self) -> DefaultLarkClient {
        let shared_config = self.shared_config.unwrap_or_else(|| {
            Arc::new(Config::builder()
                .app_id("app_id")
                .app_secret("app_secret")
                .app_type(AppType::SelfBuild)
                .build())
        });

        let config = shared_config.as_ref().clone();
        let services = self.services.unwrap_or_default();

        DefaultLarkClient {
            config,
            shared_config,
            services,
        }
    }
}

impl ClientBuilder for DefaultLarkClientBuilder {
    type Output = DefaultLarkClient;

    fn new() -> Self {
        Self::new()
    }

    fn with_config(self, config: Config) -> Self {
        self.with_config(config)
    }

    fn build(self) -> Self::Output {
        self.build()
    }

    fn build_with_shared_config(self, _config: Arc<Config>) -> Self::Output {
        self.build_with_shared_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_client_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .build();

        let client = DefaultLarkClient::new(config);
        assert_eq!(client.config().app_id, "test_app_id");
    }

    #[test]
    fn test_client_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .build();

        let client = DefaultLarkClientBuilder::new()
            .with_config(config)
            .build();

        assert_eq!(client.config().app_id, "test_app_id");
    }

    #[test]
    fn test_shared_config() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .build();
        let shared_config = Arc::new(config);

        let client = DefaultLarkClient::new_from_shared(shared_config);
        assert_eq!(client.config().app_id, "test_app_id");
    }
}
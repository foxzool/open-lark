//! Client traits and interfaces
//!
//! This module defines the core abstractions for OpenLark clients,
//! providing a clean separation between client interfaces and implementations.

use async_trait::async_trait;
use openlark_core::{config::Config, SDKResult};

/// Core client trait that all Lark clients should implement
///
/// This trait defines the fundamental interface for any Lark client,
/// providing access to configuration and basic client functionality.
/// Implementations can be synchronous or asynchronous depending on use case.
pub trait LarkClient {
    /// Get the configuration associated with this client
    fn config(&self) -> &Config;

    /// Create a new client with the given configuration
    fn new(config: Config) -> Self where Self: Sized;

    /// Create a client from a shared configuration (for performance optimization)
    fn new_from_shared(config: std::sync::Arc<Config>) -> Self where Self: Sized;
}

/// Async client trait for operations that need to be performed asynchronously
///
/// This extends the basic client trait with async capabilities,
/// allowing clients to perform operations like API calls without blocking.
#[async_trait]
pub trait AsyncLarkClient: LarkClient {
    /// Perform an async health check
    async fn health_check(&self) -> SDKResult<bool>;

    /// Get client information asynchronously
    async fn get_client_info(&self) -> SDKResult<serde_json::Value>;
}

/// Service registry trait for managing client services
///
/// This trait defines how services are registered, discovered,
/// and managed within a client instance.
pub trait ServiceRegistry {
    /// Register a service with the registry
    fn register_service<T: Send + Sync + 'static>(&mut self, name: &str, service: T);

    /// Get a service by name
    fn get_service<T: Send + Sync + 'static>(&self, name: &str) -> Option<&T>;

    /// Get a mutable service by name
    fn get_service_mut<T: Send + Sync + 'static>(&mut self, name: &str) -> Option<&mut T>;

    /// List all registered service names
    fn list_services(&self) -> Vec<String>;

    /// Check if a service is registered
    fn has_service(&self, name: &str) -> bool;
}

/// Builder trait for constructing clients with fluent interface
///
/// This provides a builder pattern for client creation,
/// allowing for step-by-step configuration.
pub trait ClientBuilder {
    /// The type of client being built
    type Output;

    /// Create a new builder instance
    fn new() -> Self;

    /// Set the configuration
    fn with_config(self, config: Config) -> Self;

    /// Build the client instance
    fn build(self) -> Self::Output;

    /// Build the client instance with shared config
    fn build_with_shared_config(self, config: std::sync::Arc<Config>) -> Self::Output;
}
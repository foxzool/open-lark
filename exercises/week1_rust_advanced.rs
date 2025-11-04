// Week 1: Advanced Rust Features - Practice Exercises
//
// This file contains practice exercises for the advanced Rust features training.
// Complete the TODO sections to practice what you've learned.

use std::collections::HashMap;
use std::time::{Duration, Instant};

// ==================== Exercise 1: Generic Container ====================
// Implement a generic service storage system based on the training content

#[derive(Debug, Clone)]
struct ServiceInfo {
    name: String,
    version: String,
    uptime: Duration,
}

impl ServiceInfo {
    fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            uptime: Duration::from_secs(0),
        }
    }

    fn increment_uptime(&mut self) {
        self.uptime += Duration::from_secs(1);
    }
}

// TODO: Implement ServiceStore with generic type S
// Requirements:
// 1. Use HashMap<String, S> to store services
// 2. Implement register(), get(), remove(), list_services() methods
// 3. Implement Clone trait if S: Clone
// 4. Add error handling for duplicate registrations

#[derive(Debug)]
enum StoreError {
    AlreadyRegistered(String),
    NotFound(String),
}

struct ServiceStore<S> {
    // TODO: Add HashMap to store services by name
    services: HashMap<String, S>,
}

impl<S> ServiceStore<S> {
    fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    // TODO: Implement register method
    // Should return Err if service already exists
    fn register(&mut self, name: String, service: S) -> Result<(), StoreError> {
        if self.services.contains_key(&name) {
            Err(StoreError::AlreadyRegistered(name))
        } else {
            self.services.insert(name, service);
            Ok(())
        }
    }

    // TODO: Implement get method
    // Return reference to stored service or None if not found
    fn get(&self, name: &str) -> Option<&S> {
        self.services.get(name)
    }

    // TODO: Implement remove method
    // Return removed service or None if not found
    fn remove(&mut self, name: &str) -> Option<S> {
        self.services.remove(name)
    }

    // TODO: Implement list_services method
    // Return vector of all service names
    fn list_services(&self) -> Vec<&String> {
        self.services.keys().collect()
    }
}

// TODO: Implement Clone for ServiceStore when S: Clone
impl<S: Clone> Clone for ServiceStore<S> {
    fn clone(&self) -> Self {
        Self {
            services: self.services.clone(),
        }
    }
}

// ==================== Exercise 2: Service Traits ====================
// Design and implement service traits based on the training content

// TODO: Define the basic Service trait
// Requirements:
// 1. Methods: name(), version(), is_available()
// 2. Include appropriate documentation
// 3. Use appropriate lifetime annotations

trait Service {
    /// Returns the name of the service
    fn name(&self) -> &str;

    /// Returns the version of the service
    fn version(&self) -> &str;

    /// Returns true if the service is available
    fn is_available(&self) -> bool { true }
}

// TODO: Define NamedService trait to avoid string hardcoding
// Requirements:
// 1. Associate constant NAME for service identification
// 2. Implement name_static() method
// 3. Include clone_owned() method for copying
trait NamedService: Service + Sized {
    /// The constant name of this service type
    const NAME: &'static str;

    /// Returns the static name of this service type
    fn name_static() -> Option<&'static str> { Some(Self::NAME) }

    /// Creates a new owned instance of this service
    fn clone_owned(&self) -> Self;
}

// TODO: Implement a concrete service type
// Create a service that represents message functionality
#[derive(Debug, Clone)]
struct MessageService {
    name: String,
    version: String,
    active: bool,
}

impl MessageService {
    fn new() -> Self {
        Self {
            name: "message".to_string(),
            version: "1.0.0".to_string(),
            active: false,
        }
    }

    fn start(&mut self) {
        self.active = true;
    }

    fn stop(&mut self) {
        self.active = false;
    }

    fn send_message(&self, msg: &str) -> String {
        if self.is_available() {
            format!("Message sent: {}", msg)
        } else {
            "Service not available".to_string()
        }
    }
}

// TODO: Implement Service trait for MessageService
impl Service for MessageService {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn is_available(&self) -> bool {
        self.active
    }
}

// TODO: Implement NamedService trait for MessageService
impl NamedService for MessageService {
    const NAME: &'static str = "message";

    fn name_static() -> Option<&'static str> {
        Some(Self::NAME)
    }

    fn clone_owned(&self) -> Self {
        Self {
            name: self.name.clone(),
            version: self.version.clone(),
            active: self.active,
        }
    }
}

// ==================== Exercise 3: Lifetime Management ====================
// Design services with proper lifetime management

// TODO: Define ServiceConfig with lifetime parameters
// Requirements:
// 1. Use lifetime annotations for borrowed data
// 2. Include methods for configuration management
// 3. Handle configuration validation

#[derive(Debug)]
struct ServiceConfig<'a> {
    name: &'a str,
    endpoint: &'a str,
    timeout: Duration,
    max_connections: usize,
}

impl<'a> ServiceConfig<'a> {
    fn new(name: &'a str, endpoint: &'a str) -> Self {
        Self {
            name,
            endpoint,
            timeout: Duration::from_secs(30),
            max_connections: 100,
        }
    }

    // TODO: Implement validation method
    // Validate that endpoint is a valid URL and other constraints
    fn validate(&self) -> Result<(), ConfigError> {
        if self.name.is_empty() {
            return Err(ConfigError::InvalidName);
        }

        if self.endpoint.is_empty() {
            return Err(ConfigError::InvalidEndpoint);
        }

        if !self.endpoint.starts_with("http://") && !self.endpoint.starts_with("https://") {
            return Err(ConfigError::InvalidEndpoint);
        }

        Ok(())
    }

    // TODO: Implement with_timeout method
    // Return new config with different timeout
    fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    // TODO: Implement with_max_connections method
    // Return new config with different max_connections
    fn with_max_connections(mut self, max_connections: usize) -> Self {
        self.max_connections = max_connections;
        self
    }
}

// Define configuration errors
#[derive(Debug)]
enum ConfigError {
    InvalidName,
    InvalidEndpoint,
    InvalidTimeout,
    InvalidMaxConnections,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::InvalidName => write!(f, "Service name cannot be empty"),
            ConfigError::InvalidEndpoint => write!(f, "Endpoint must be a valid URL"),
            ConfigError::InvalidTimeout => write!(f, "Timeout must be positive"),
            ConfigError::InvalidMaxConnections => write!(f, "Max connections must be positive"),
        }
    }
}

impl std::error::Error for ConfigError {}

// TODO: Implement ServiceClient with proper lifetime management
// Requirements:
// 1. Use ServiceConfig with lifetime parameter
// 2. Track last request time
// 3. Implement request method with freshness checking

#[derive(Debug)]
struct ServiceClient<'a> {
    config: ServiceConfig<'a>,
    last_request: Instant,
    request_count: u64,
}

impl<'a> ServiceClient<'a> {
    fn new(config: ServiceConfig<'a>) -> Result<Self, ConfigError> {
        config.validate()?;
        Ok(Self {
            config,
            last_request: Instant::now(),
            request_count: 0,
        })
    }

    // TODO: Implement make_request method
    // Should:
    // 1. Update last_request time
    // 2. Increment request count
    // 3. Format and return response string
    fn make_request(&mut self, path: &str) -> String {
        self.last_request = Instant::now();
        self.request_count += 1;

        format!("Request #{} to {}{}: {}",
                self.request_count,
                self.config.endpoint,
                if path.starts_with('/') { "" } else { "/" },
                path)
    }

    // TODO: Implement is_fresh method
    // Return true if last request was within timeout period
    fn is_fresh(&self) -> bool {
        self.last_request.elapsed() < self.config.timeout
    }

    // TODO: Implement reset method
    // Reset request count and update timestamp
    fn reset(&mut self) {
        self.request_count = 0;
        self.last_request = Instant::now();
    }

    // TODO: Implement get_stats method
    // Return service statistics
    fn get_stats(&self) -> ServiceStats {
        ServiceStats {
            name: self.config.name,
            endpoint: self.config.endpoint,
            request_count: self.request_count,
            is_fresh: self.is_fresh(),
            uptime: self.last_request.elapsed(),
        }
    }
}

// TODO: Define ServiceStats structure
#[derive(Debug)]
struct ServiceStats {
    name: String,
    endpoint: String,
    request_count: u64,
    is_fresh: bool,
    uptime: Duration,
}

// ==================== Exercise 4: Advanced Generic Patterns ====================
// Implement advanced patterns combining all learned concepts

// TODO: Implement ServiceManager with complex generic constraints
// Requirements:
// 1. Store different types of services
// 2. Use trait bounds for type safety
// 3. Provide methods for bulk operations
// 4. Include proper error handling

#[derive(Debug)]
enum ManagerError {
    ServiceNotFound(String),
    ConfigError(String),
    RegistrationFailed(String),
}

struct ServiceManager<'a> {
    clients: Vec<ServiceClient<'a>>,
    services: HashMap<&'static str, ServiceInfo>,
}

impl<'a> ServiceManager<'a> {
    fn new() -> Self {
        Self {
            clients: Vec::new(),
            services: HashMap::new(),
        }
    }

    // TODO: Implement add_client method with proper configuration
    fn add_client(&mut self, config: ServiceConfig<'a>) -> Result<(), ManagerError> {
        let client = ServiceClient::new(config)?;
        self.clients.push(client);

        // Add service info to tracking
        self.services.insert(config.name, ServiceInfo::new(config.name, "1.0"));

        Ok(())
    }

    // TODO: Implement get_client method with NamedService constraint
    // This should allow type-safe client retrieval
    fn get_client<S>(&self) -> Option<&ServiceClient<'a>>
    where
        S: NamedService,
    {
        // Find client by service name
        for client in &self.clients {
            if client.config.name == S::NAME {
                return Some(client);
            }
        }
        None
    }

    // TODO: Implement bulk_request method
    // Make requests to all fresh services
    fn bulk_request(&mut self, path: &str) -> Vec<String> {
        self.clients.iter_mut()
            .filter(|client| client.is_fresh())
            .map(|client| client.make_request(path))
            .collect()
    }

    // TODO: Implement get_all_services method
    // Return all service information
    fn get_all_services(&self) -> Vec<&ServiceInfo> {
        self.services.values().collect()
    }

    // TODO: Implement get_service_by_name method
    fn get_service_by_name(&self, name: &str) -> Option<&ServiceInfo> {
        self.services.get(name)
    }

    // TODO: Implement update_service_info method
    fn update_service_info(&mut self, name: &str, info: ServiceInfo) {
        self.services.insert(name, info);
    }
}

// ==================== Test Cases ====================
// Test your implementations with these test cases

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_store() {
        let mut store = ServiceStore::new();

        // Test registration
        let service1 = ServiceInfo::new("test1", "1.0");
        assert!(store.register("test1".to_string(), service1).is_ok());

        // Test duplicate registration
        let service1_duplicate = ServiceInfo::new("test1", "1.0");
        assert!(store.register("test1".to_string(), service1_duplicate).is_err());

        // Test retrieval
        let retrieved = store.get("test1");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().version, "1.0");

        // Test listing
        let services = store.list_services();
        assert_eq!(services.len(), 1);
        assert_eq!(services[0], "test1");
    }

    #[test]
    fn test_message_service() {
        let mut service = MessageService::new();

        // Test initial state
        assert_eq!(service.name(), "message");
        assert_eq!(service.version(), "1.0.0");
        assert!(!service.is_available());

        // Test starting service
        service.start();
        assert!(service.is_available());

        // Test message sending
        let response = service.send_message("Hello World");
        assert_eq!(response, "Message sent: Hello World");

        // Test stopping service
        service.stop();
        assert!(!service.is_available());

        let response = service.send_message("Hello World");
        assert_eq!(response, "Service not available");
    }

    #[test]
    fn test_service_config() {
        let config = ServiceConfig::new("test", "https://api.example.com");

        // Test default values
        assert_eq!(config.name, "test");
        assert_eq!(config.endpoint, "https://api.example.com");
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_connections, 100);

        // Test validation
        assert!(config.validate().is_ok());

        // Test invalid endpoint
        let invalid_config = ServiceConfig::new("test", "invalid-url");
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_service_client() {
        let config = ServiceConfig::new("test", "https://api.example.com");
        let mut client = ServiceClient::new(config).unwrap();

        // Test initial state
        assert_eq!(client.get_stats().request_count, 0);
        assert!(client.is_fresh());

        // Test request making
        let response = client.make_request("/api/data");
        assert!(response.contains("https://api.example.com"));
        assert!(response.contains("/api/data"));
        assert_eq!(client.get_stats().request_count, 1);

        // Test freshness
        assert!(client.is_fresh());

        // Test reset
        client.reset();
        assert_eq!(client.get_stats().request_count, 0);
    }

    #[test]
    fn test_service_manager() {
        let mut manager = ServiceManager::new();

        // Test adding clients
        let config1 = ServiceConfig::new("service1", "https://api1.example.com");
        let config2 = ServiceConfig::new("service2", "https://api2.example.com");

        assert!(manager.add_client(config1).is_ok());
        assert!(manager.add_client(config2).is_ok());

        // Test getting clients by type (would need concrete service types)
        // This will be tested with MessageService

        // Test bulk requests
        let responses = manager.bulk_request("/test");
        assert_eq!(responses.len(), 2);

        // Test service information
        let services = manager.get_all_services();
        assert_eq!(services.len(), 2);

        let service1 = manager.get_service_by_name("service1");
        assert!(service1.is_some());
        assert_eq!(service1.unwrap().name, "service1");
    }

    #[test]
    fn test_advanced_generics() {
        // Test generic store with ServiceInfo
        let mut store: ServiceStore<ServiceInfo> = ServiceStore::new();

        let service = ServiceInfo::new("test", "1.0");
        assert!(store.register("test".to_string(), service).is_ok());

        // Test clone with cloneable type
        let cloned_store = store.clone();
        assert_eq!(cloned_store.get("test").unwrap().name, "test");

        // Test with MessageService
        let mut message_store: ServiceStore<MessageService> = ServiceStore::new();
        let message_service = MessageService::new();
        assert!(message_store.register("message".to_string(), message_service).is_ok());
    }

    #[test]
    fn test_trait_objects() {
        // Test NamedService constant
        assert_eq!(MessageService::NAME, "message");
        assert_eq!(MessageService::name_static(), Some("message"));

        // Test clone_owned
        let service1 = MessageService::new();
        let service2 = service1.clone_owned();
        assert_eq!(service1.name(), service2.name());
        assert_eq!(service1.is_available(), service2.is_available());
    }

    #[test]
    fn test_lifetimes() {
        let config_string = String::from("test");
        let config = ServiceConfig::new(&config_string, "https://api.example.com");

        let client = ServiceClient::new(config).unwrap();

        // Test that config lifetime is properly managed
        assert_eq!(client.config.name, "test");
        assert_eq!(client.config.endpoint, "https://api.example.com");

        // Test that ServiceConfig can't outlive the borrowed data
        drop(config_string);
        // This line would cause a compilation error if uncommented:
        // assert_eq!(client.config.name, "test");
    }
}

// ==================== Bonus Challenges ====================
// Complete these if you finish the main exercises early

// Challenge 1: Implement a ServiceBuilder with fluent API
// Requirements:
// - Use generic types for different service configurations
// - Implement fluent methods (with_* patterns)
// - Include validation and error handling

// Challenge 2: Implement a ServicePool for connection management
// Requirements:
// - Use Arc<RwLock> for thread-safe access
// - Implement connection pooling logic
// - Include proper lifetime management

// Challenge 3: Implement async service traits
// Requirements:
// - Add async methods to Service trait
// - Implement async ServiceManager
// - Use proper async/await patterns

// Challenge 4: Implement service health checking
// Requirements:
// - Add health checking capability to Service trait
// - Implement health check scheduling
// - Include status reporting

// ==================== Learning Reflection ====================
// After completing the exercises, reflect on:
// 1. What concepts were most challenging?
// 2. Which patterns are most useful for our ServiceRegistry project?
// 3. How can we apply these patterns to improve the Open-Lark SDK?
// 4. What additional learning might be helpful for Phase 2?

// Add your reflection below:
/*
Learning Reflection:
- Most challenging concept:
- Most useful pattern for our project:
- How to apply to Open-Lark SDK:
- Additional learning needed:
*/
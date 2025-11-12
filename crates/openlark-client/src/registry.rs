//! Service registry implementation
//!
//! This module provides a default implementation of the ServiceRegistry trait,
//! allowing services to be registered, discovered, and managed dynamically.

use std::collections::HashMap;
use std::any::{Any, TypeId};

use crate::traits::ServiceRegistry;

/// Default implementation of ServiceRegistry
///
/// This registry manages services using a HashMap with type-safe
/// access to registered services. Services are stored as Any
/// and can be retrieved by their concrete type.
#[derive(Debug, Default)]
pub struct DefaultServiceRegistry {
    /// HashMap storing services with string keys and Any values
    services: HashMap<String, Box<dyn Any + Send + Sync>>,
    /// Type registry for ensuring type safety
    type_registry: HashMap<String, TypeId>,
}

impl DefaultServiceRegistry {
    /// Create a new empty service registry
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            type_registry: HashMap::new(),
        }
    }

    /// Clear all registered services
    pub fn clear(&mut self) {
        self.services.clear();
        self.type_registry.clear();
    }

    /// Get the number of registered services
    pub fn len(&self) -> usize {
        self.services.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.services.is_empty()
    }
}

impl ServiceRegistry for DefaultServiceRegistry {
    /// Register a service with the registry
    fn register_service<T: Send + Sync + 'static>(&mut self, name: &str, service: T) {
        self.services.insert(name.to_string(), Box::new(service));
        self.type_registry.insert(name.to_string(), TypeId::of::<T>());
    }

    /// Get a service by name
    fn get_service<T: Send + Sync + 'static>(&self, name: &str) -> Option<&T> {
        // First check if the service exists and has the correct type
        if !self.has_service(name) {
            return None;
        }

        let expected_type = self.type_registry.get(name)?;
        if *expected_type != TypeId::of::<T>() {
            return None;
        }

        // Safe to unwrap since we checked existence and type
        self.services
            .get(name)
            .and_then(|service| service.downcast_ref())
    }

    /// Get a mutable service by name
    fn get_service_mut<T: Send + Sync + 'static>(&mut self, name: &str) -> Option<&mut T> {
        // First check if the service exists and has the correct type
        if !self.has_service(name) {
            return None;
        }

        let expected_type = self.type_registry.get(name)?;
        if *expected_type != TypeId::of::<T>() {
            return None;
        }

        // Safe to unwrap since we checked existence and type
        self.services
            .get_mut(name)
            .and_then(|service| service.downcast_mut())
    }

    /// List all registered service names
    fn list_services(&self) -> Vec<String> {
        self.services.keys().cloned().collect()
    }

    /// Check if a service is registered
    fn has_service(&self, name: &str) -> bool {
        self.services.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestService {
        name: String,
    }

    #[derive(Debug)]
    struct AnotherTestService {
        value: i32,
    }

    #[test]
    fn test_service_registration() {
        let mut registry = DefaultServiceRegistry::new();

        // Register services
        let test_service = TestService {
            name: "test".to_string(),
        };
        let another_service = AnotherTestService { value: 42 };

        registry.register_service("test_service", test_service);
        registry.register_service("another_service", another_service);

        // Test retrieval
        let retrieved: &TestService = registry.get_service("test_service").unwrap();
        assert_eq!(retrieved.name, "test");

        let another_retrieved: &AnotherTestService = registry.get_service("another_service").unwrap();
        assert_eq!(another_retrieved.value, 42);

        // Test service listing
        let services = registry.list_services();
        assert_eq!(services.len(), 2);
        assert!(services.contains(&"test_service".to_string()));
        assert!(services.contains(&"another_service".to_string()));
    }

    #[test]
    fn test_mutable_service_access() {
        let mut registry = DefaultServiceRegistry::new();

        let initial_service = TestService {
            name: "initial".to_string(),
        };
        registry.register_service("test_service", initial_service);

        // Get mutable reference
        let service_mut: &mut TestService = registry.get_service_mut("test_service").unwrap();
        service_mut.name = "modified".to_string();

        // Verify the change
        let retrieved: &TestService = registry.get_service("test_service").unwrap();
        assert_eq!(retrieved.name, "modified");
    }

    #[test]
    fn test_nonexistent_service() {
        let mut registry = DefaultServiceRegistry::new();

        let service: Option<&TestService> = registry.get_service("nonexistent");
        assert!(service.is_none());

        let mut_service: Option<&mut TestService> = registry.get_service_mut("nonexistent");
        assert!(mut_service.is_none());

        assert!(!registry.has_service("nonexistent"));
    }

    #[test]
    fn test_type_safety() {
        let mut registry = DefaultServiceRegistry::new();

        registry.register_service("test_service", TestService {
            name: "test".to_string(),
        });

        // Try to retrieve with wrong type - should return None
        let wrong_type: Option<&AnotherTestService> = registry.get_service("test_service");
        assert!(wrong_type.is_none());
    }

    #[test]
    fn test_registry_operations() {
        let mut registry = DefaultServiceRegistry::new();

        // Initially empty
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);

        // Add a service
        registry.register_service("test", TestService {
            name: "test".to_string(),
        });
        assert!(!registry.is_empty());
        assert_eq!(registry.len(), 1);

        // Clear registry
        registry.clear();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }
}
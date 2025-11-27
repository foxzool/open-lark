//! 类型安全的服务注册表

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::Result;

use super::service::{Service, ServiceHealth, ServiceKind};

/// 注册表存储的条目
#[derive(Debug, Clone)]
pub struct ServiceHandle {
    pub kind: ServiceKind,
    pub service: Arc<dyn Service>,
    pub health: ServiceHealth,
}

/// 类型安全注册表
#[derive(Debug, Default)]
pub struct TypedServiceRegistry {
    services: RwLock<HashMap<String, ServiceHandle>>,
}

impl TypedServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, handle: ServiceHandle) -> Result<()> {
        let mut guard = self.services.write().expect("lock poisoned");
        guard.insert(handle.kind.name.to_string(), handle);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<ServiceHandle> {
        self.services
            .read()
            .ok()
            .and_then(|map| map.get(name).cloned())
    }

    pub fn list(&self) -> Vec<ServiceHandle> {
        self.services
            .read()
            .map(|map| map.values().cloned().collect())
            .unwrap_or_default()
    }
}

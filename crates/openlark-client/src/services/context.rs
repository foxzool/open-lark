//! 服务上下文
//!
//! 为服务生命周期提供共享的配置、扩展依赖与可选的观测组件。

use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::Config;

/// 服务上下文，线程安全可共享
#[derive(Clone, Debug)]
pub struct ServiceContext {
    /// 全局客户端配置（拷贝一份，避免生命周期依赖）
    pub config: Config,
    /// 可扩展组件（如 HTTP 客户端、指标收集器等）
    extensions: Arc<RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
}

impl ServiceContext {
    /// 创建新的上下文
    pub fn new(config: Config) -> Self {
        Self {
            config,
            extensions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 写入扩展组件
    pub fn insert_extension<T>(&self, name: impl Into<String>, ext: T)
    where
        T: Any + Send + Sync,
    {
        let mut guard = self.extensions.write().expect("lock poisoned");
        guard.insert(name.into(), Arc::new(ext));
    }

    /// 读取扩展组件（类型安全）
    pub fn get_extension<T>(&self, name: &str) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        let guard = self.extensions.read().expect("lock poisoned");
        guard
            .get(name)
            .and_then(|arc_any| arc_any.clone().downcast::<T>().ok().map(|arc| arc))
    }
}

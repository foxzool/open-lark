//! 延迟初始化工具模块
//!
//! 提供 `LazyService` 包装器，用于延迟初始化服务实例。
//! 这在客户端构造时不想立即初始化所有服务时很有用。

use std::sync::OnceLock;

/// 延迟初始化的服务包装器
///
/// 使用 `std::sync::OnceLock` 实现线程安全的延迟初始化。
/// 服务只在第一次访问时创建，之后复用已创建的实例。
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_client::lazy::LazyService;
/// use openlark_core::config::Config;
///
/// pub struct MyClient {
///     config: Arc<Config>,
///     docs_service: LazyService<DocsClient>,
/// }
///
/// impl MyClient {
///     pub fn new(config: Arc<Config>) -> Self {
///         Self {
///             config: config.clone(),
///             docs_service: LazyService::new(move || {
///                 DocsClient::new(config)
///             }),
///         }
///     }
///
///     pub fn docs(&self) -> &DocsClient {
///         self.docs_service.get()
///     }
/// }
/// ```
pub struct LazyService<T> {
    /// 延迟初始化存储
    inner: OnceLock<T>,
    /// 初始化工厂函数（仅用于第一次初始化）
    factory: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> LazyService<T> {
    /// 创建新的延迟初始化服务
    ///
    /// # 参数
    ///
    /// * `factory` - 创建服务的工厂函数
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            inner: OnceLock::new(),
            factory: Box::new(factory),
        }
    }

    /// 获取服务实例
    ///
    /// 如果是第一次访问，会调用工厂函数创建服务。
    /// 后续访问直接返回已创建的服务。
    pub fn get(&self) -> &T {
        self.inner.get_or_init(|| (self.factory)())
    }

    /// 检查服务是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.inner.get().is_some()
    }

    /// 尝试获取已初始化的服务（如果未初始化则返回 None）
    pub fn try_get(&self) -> Option<&T> {
        self.inner.get()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for LazyService<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.try_get() {
            Some(service) => f.debug_tuple("LazyService").field(service).finish(),
            None => f
                .debug_struct("LazyService")
                .field("status", &"uninitialized")
                .finish(),
        }
    }
}

impl<T: Clone> Clone for LazyService<T> {
    fn clone(&self) -> Self {
        // 如果已初始化，克隆内部值；否则创建新的未初始化状态
        match self.try_get() {
            Some(value) => Self {
                inner: OnceLock::from(value.clone()),
                factory: Box::new(|| unreachable!("Already initialized")),
            },
            None => Self {
                inner: OnceLock::new(),
                factory: Box::new(|| unreachable!("Cannot clone uninitialized LazyService")),
            },
        }
    }
}

/// 延迟初始化服务 trait
///
/// 为 Client 提供延迟初始化服务的统一接口
pub trait LazyClientTrait: Send + Sync {
    /// 服务类型标识
    fn service_type() -> &'static str;

    /// 检查服务是否可用（已初始化且配置正确）
    fn is_available(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestService {
        value: String,
    }

    #[test]
    fn test_lazy_service_initialization() {
        let lazy = LazyService::new(|| TestService {
            value: "test".to_string(),
        });

        // 未初始化状态
        assert!(!lazy.is_initialized());
        assert!(lazy.try_get().is_none());

        // 第一次访问触发初始化
        let service = lazy.get();
        assert_eq!(service.value, "test");
        assert!(lazy.is_initialized());

        // 后续访问返回同一实例
        let service2 = lazy.get();
        assert!(std::ptr::eq(service, service2));
    }

    #[test]
    fn test_lazy_service_clone_initialized() {
        let lazy = LazyService::new(|| TestService {
            value: "original".to_string(),
        });

        // 先初始化
        let _ = lazy.get();

        // 克隆已初始化的服务
        let cloned = lazy.clone();
        assert!(cloned.is_initialized());
        assert_eq!(cloned.get().value, "original");
    }

    #[test]
    fn test_lazy_service_debug() {
        let lazy: LazyService<TestService> = LazyService::new(|| TestService {
            value: "debug_test".to_string(),
        });

        // 未初始化时的 debug 输出
        let debug_uninit = format!("{lazy:?}");
        assert!(debug_uninit.contains("uninitialized"));

        // 初始化后的 debug 输出
        let _ = lazy.get();
        let debug_init = format!("{lazy:?}");
        assert!(debug_init.contains("debug_test"));
    }
}

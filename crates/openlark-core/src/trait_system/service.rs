use crate::{config::Config, observability::OperationTracker, SDKResult};
use std::future::Future;

/// 服务基础 trait
///
/// 定义所有 Lark 服务的通用行为和接口
pub trait Service {
    /// 获取服务配置
    fn config(&self) -> &Config;

    /// 获取服务名称，用于日志和监控
    fn service_name() -> &'static str
    where
        Self: Sized;

    /// 服务版本，用于 API 兼容性
    fn service_version() -> &'static str
    where
        Self: Sized,
    {
        "v1"
    }
}

/// 可观测性支持 trait
///
/// 为服务提供统一的日志、监控和跟踪接口
pub trait ServiceObservability: Service {
    /// 记录服务操作开始
    fn log_operation_start(&self, operation: &str, params: Option<&str>)
    where
        Self: Sized,
    {
        tracing::debug!(
            service_name = <Self as Service>::service_name(),
            operation = operation,
            params = params,
            "Starting service operation"
        );
    }

    /// 记录服务操作完成
    fn log_operation_success(&self, operation: &str, duration_ms: u64)
    where
        Self: Sized,
    {
        tracing::info!(
            service_name = <Self as Service>::service_name(),
            operation = operation,
            duration_ms = duration_ms,
            "Service operation completed successfully"
        );
    }

    /// 记录服务操作失败
    fn log_operation_error(&self, operation: &str, error: &str, duration_ms: u64)
    where
        Self: Sized,
    {
        tracing::error!(
            service_name = <Self as Service>::service_name(),
            operation = operation,
            duration_ms = duration_ms,
            error = error,
            "Service operation failed"
        );
    }
}

/// 异步服务操作 trait
///
/// 为异步操作提供统一的错误处理和重试机制
pub trait AsyncServiceOperation<T, R>: Service
where
    T: Send + Sync,
    R: Send + Sync,
{
    /// 执行带有观测性支持的异步操作
    fn execute_with_observability<F, Fut>(
        &self,
        operation_name: &str,
        operation: F,
    ) -> impl Future<Output = SDKResult<R>> + Send
    where
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = SDKResult<R>> + Send,
        Self: ServiceObservability + Sync + Sized,
    {
        let service_name = <Self as Service>::service_name();
        async move {
            let tracker = OperationTracker::start(service_name, operation_name);

            match operation().await {
                Ok(result) => {
                    tracker.success();
                    Ok(result)
                }
                Err(err) => {
                    tracker.error(&err.to_string());
                    Err(err)
                }
            }
        }
    }
}

/// 服务构建器 trait
///
/// 提供统一的服务实例化接口
pub trait ServiceBuilder<S: Service> {
    /// 使用配置创建服务实例
    fn build(config: Config) -> S;

    /// 使用默认配置创建服务实例
    fn build_default() -> S
    where
        Config: Default,
    {
        Self::build(Config::default())
    }
}

/// 支持健康检查的服务 trait
///
/// 为服务提供健康状态监控能力
pub trait ServiceHealthCheck: Service {
    /// 检查服务健康状态
    fn health_check(&self) -> impl Future<Output = SDKResult<ServiceHealthStatus>> + Send;

    /// 获取服务状态摘要
    fn status_summary(&self) -> ServiceStatusSummary
    where
        Self: Sized,
    {
        ServiceStatusSummary {
            service_name: <Self as Service>::service_name().to_string(),
            version: <Self as Service>::service_version().to_string(),
            config_valid: self.is_config_valid(),
        }
    }

    /// 验证配置是否有效
    fn is_config_valid(&self) -> bool {
        let config = self.config();
        !config.app_id.is_empty() && !config.app_secret.is_empty()
    }
}

/// 服务健康状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceHealthStatus {
    /// 健康状态
    Healthy,
    /// 降级状态（部分功能可用）
    Degraded(String),
    /// 不健康状态
    Unhealthy(String),
}

/// 服务状态摘要
#[derive(Debug, Clone)]
pub struct ServiceStatusSummary {
    /// 服务名称
    pub service_name: String,
    /// 服务版本
    pub version: String,
    /// 配置是否有效
    pub config_valid: bool,
}

/// 可配置的服务 trait
///
/// 为服务提供动态配置更新能力
pub trait ConfigurableService: Service {
    /// 更新服务配置
    fn update_config(&mut self, new_config: Config) -> SDKResult<()>;

    /// 验证新配置的有效性
    fn validate_config(&self, config: &Config) -> SDKResult<()> {
        if config.app_id.is_empty() {
            return Err(crate::error::validation_error(
                "app_id",
                "app_id cannot be empty",
            ));
        }

        if config.app_secret.is_empty() {
            return Err(crate::error::validation_error(
                "app_secret",
                "app_secret cannot be empty",
            ));
        }

        Ok(())
    }
}

/// 带缓存的服务 trait
///
/// 为服务提供缓存机制支持
pub trait CacheableService: Service {
    /// 缓存键类型
    type CacheKey: Send + Sync + std::hash::Hash + Eq + Clone;
    /// 缓存值类型
    type CacheValue: Send + Sync + Clone;

    /// 从缓存获取数据
    fn get_from_cache(&self, key: &Self::CacheKey) -> Option<Self::CacheValue>;

    /// 将数据存入缓存
    fn put_to_cache(&self, key: Self::CacheKey, value: Self::CacheValue);

    /// 清除缓存
    fn clear_cache(&self);

    /// 缓存过期时间（秒）
    fn cache_ttl(&self) -> u64 {
        300 // 默认 5 分钟
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    // 测试用的简单服务实现
    struct TestService {
        config: Config,
    }

    impl Service for TestService {
        fn config(&self) -> &Config {
            &self.config
        }

        fn service_name() -> &'static str {
            "test_service"
        }

        fn service_version() -> &'static str {
            "v1.0"
        }
    }

    impl ServiceObservability for TestService {}

    impl ServiceBuilder<TestService> for TestService {
        fn build(config: Config) -> TestService {
            TestService { config }
        }
    }

    impl AsyncServiceOperation<(), String> for TestService {}

    #[test]
    fn test_service_creation() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let service = TestService::build(config.clone());

        assert_eq!(service.config().app_id, "test_app");
        assert_eq!(service.config().app_secret, "test_secret");
        assert_eq!(TestService::service_name(), "test_service");
        assert_eq!(TestService::service_version(), "v1.0");
    }

    #[test]
    fn test_service_status_summary() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let service = TestService::build(config);

        // 需要手动实现 ServiceHealthCheck 来测试
        struct HealthCheckableTestService {
            inner: TestService,
        }

        impl Service for HealthCheckableTestService {
            fn config(&self) -> &Config {
                self.inner.config()
            }

            fn service_name() -> &'static str {
                TestService::service_name()
            }

            fn service_version() -> &'static str {
                TestService::service_version()
            }
        }

        impl ServiceHealthCheck for HealthCheckableTestService {
            async fn health_check(&self) -> SDKResult<ServiceHealthStatus> {
                Ok(ServiceHealthStatus::Healthy)
            }
        }

        let health_service = HealthCheckableTestService { inner: service };
        let summary = health_service.status_summary();

        assert_eq!(summary.service_name, "test_service");
        assert_eq!(summary.version, "v1.0");
        assert!(summary.config_valid);
    }

    #[test]
    fn test_config_validation() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let service = TestService::build(config);

        struct ConfigurableTestService {
            inner: TestService,
        }

        impl Service for ConfigurableTestService {
            fn config(&self) -> &Config {
                self.inner.config()
            }

            fn service_name() -> &'static str {
                TestService::service_name()
            }
        }

        impl ConfigurableService for ConfigurableTestService {
            fn update_config(&mut self, new_config: Config) -> SDKResult<()> {
                self.validate_config(&new_config)?;
                self.inner.config = new_config;
                Ok(())
            }
        }

        let configurable_service = ConfigurableTestService { inner: service };

        // 测试有效配置
        let valid_config = Config::builder()
            .app_id("new_app")
            .app_secret("new_secret")
            .build();
        assert!(configurable_service.validate_config(&valid_config).is_ok());

        // 测试无效配置 - 空 app_id
        let invalid_config = Config::builder().app_id("").app_secret("secret").build();
        assert!(configurable_service
            .validate_config(&invalid_config)
            .is_err());

        // 测试无效配置 - 空 app_secret
        let invalid_config = Config::builder().app_id("app").app_secret("").build();
        assert!(configurable_service
            .validate_config(&invalid_config)
            .is_err());
    }

    #[test]
    fn test_service_builder_default() {
        // 由于我们的 Config 有 Default 实现，测试默认构建
        let service = TestService::build_default();
        // 默认配置应为空凭证，保持安全缺省
        assert!(service.config().app_id.is_empty());
    }

    #[tokio::test]
    async fn test_async_service_operation() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let service = TestService::build(config);

        // 测试成功操作
        let result = service
            .execute_with_observability("test_operation", || async { Ok("success".to_string()) })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");

        // 测试失败操作
        let result = service
            .execute_with_observability("test_operation_fail", || async {
                Err(crate::error::validation_error("service", "test error"))
            })
            .await;

        assert!(result.is_err());
    }
}

//! Service trait
//!
//! 业务服务的最小能力抽象：持有 Config，并提供服务标识信息。

use crate::config::Config;

/// 业务服务抽象
pub trait Service {
    /// 获取 SDK 配置
    fn config(&self) -> &Config;

    /// 服务名（用于日志/指标/路由等）
    fn service_name() -> &'static str
    where
        Self: Sized;

    /// 服务版本（默认 v1）
    fn service_version() -> &'static str
    where
        Self: Sized,
    {
        "v1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            "v2"
        }
    }

    #[test]
    fn test_service_name() {
        assert_eq!(TestService::service_name(), "test_service");
    }

    #[test]
    fn test_service_version() {
        assert_eq!(TestService::service_version(), "v2");
    }

    #[test]
    fn test_service_config() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let service = TestService { config };
        assert_eq!(service.config().app_id(), "test_app");
    }
}

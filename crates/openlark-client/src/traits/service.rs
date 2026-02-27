//! OpenLark Service 核心特征
//!
//! 定义服务的统一接口和行为

use crate::Result;

/// 🌐 服务基础特征
///
/// 所有服务实现都应该实现此特征
///
/// # 特性要求
/// - 异步支持：所有操作都是异步的
/// - 线程安全：服务可以跨线程安全使用
/// - 生命周期：支持服务的启动、停止和重启
pub trait ServiceTrait: Send + Sync {
    /// 📋 服务名称
    fn name(&self) -> &'static str;

    /// 🔢 服务版本
    fn version(&self) -> &'static str {
        "1.0.0"
    }

    /// 📝 服务描述
    fn description(&self) -> &'static str {
        "OpenLark Service"
    }

    /// ✅ 检查服务健康状态
    async fn health_check(&self) -> Result<bool>;

    /// 🔄 启动服务
    async fn start(&self) -> Result<()> {
        tracing::info!("服务 '{}' 启动", self.name());
        Ok(())
    }

    /// 🛑 停止服务
    async fn stop(&self) -> Result<()> {
        tracing::info!("服务 '{}' 停止", self.name());
        Ok(())
    }
}

/// 🔄 服务生命周期特征
///
/// 定义服务的启动、停止和健康检查生命周期管理
pub trait ServiceLifecycle: Send + Sync {
    /// 🚀 启动服务
    async fn start(&self) -> Result<()> {
        tracing::info!("服务启动");
        Ok(())
    }

    /// 🛑 停止服务
    async fn stop(&self) -> Result<()> {
        tracing::info!("服务停止");
        Ok(())
    }

    /// 🔄 重启服务
    async fn restart(&self) -> Result<()> {
        tracing::info!("服务重启");
        self.stop().await?;
        self.start().await
    }

    /// ✅ 健康检查
    async fn health_check(&self) -> Result<bool> {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestService {
        name: &'static str,
    }

    impl ServiceTrait for TestService {
        fn name(&self) -> &'static str {
            self.name
        }

        async fn health_check(&self) -> Result<bool> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn test_service_trait() {
        let service = TestService {
            name: "test_service",
        };

        assert_eq!(service.name(), "test_service");
        assert_eq!(service.version(), "1.0.0");
        assert_eq!(service.description(), "OpenLark Service");

        let health = service.health_check().await;
        assert!(health.is_ok());
        assert!(health.unwrap());
    }
}

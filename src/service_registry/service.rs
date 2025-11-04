//! 服务特征和基础类型定义

use std::any::Any;
use std::fmt;

/// 服务基础特征
///
/// 所有注册到ServiceRegistry的服务都必须实现此特征
pub trait Service: Send + Sync + 'static {
    /// 获取服务名称
    fn name(&self) -> &'static str;

    /// 获取服务版本
    fn version(&self) -> &'static str;

    /// 获取服务状态
    fn status(&self) -> ServiceStatus {
        ServiceStatus::Healthy
    }

    /// 检查服务是否可用
    fn is_available(&self) -> bool {
        matches!(self.status(), ServiceStatus::Healthy)
    }

    /// 获取服务描述
    fn description(&self) -> &'static str {
        "No description provided"
    }

    /// 获取服务的Any引用，用于类型转换
    fn as_any(&self) -> &dyn Any;

    /// 获取服务的Any可变引用，用于类型转换
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 命名服务特征
///
/// 提供编译时类型安全的服务名称
pub trait NamedService: Service + Sized {
    /// 服务名称常量
    const NAME: &'static str;

    /// 获取静态名称（可选，用于动态查找）
    fn name_static() -> Option<&'static str> {
        Some(Self::NAME)
    }

    /// 克隆服务实例
    fn clone_owned(&self) -> Self;

    /// 检查服务是否匹配指定名称
    fn matches_name(name: &str) -> bool {
        Self::NAME == name
    }
}

/// 服务状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ServiceStatus {
    /// 健康，服务正常
    Healthy,
    /// 降级，服务可用但性能受限
    Degraded,
    /// 不健康，服务不可用
    Unhealthy,
    /// 启动中
    Starting,
    /// 停止中
    Stopping,
    /// 已停止
    Stopped,
    /// 错误状态
    Error,
}

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceStatus::Healthy => write!(f, "Healthy"),
            ServiceStatus::Degraded => write!(f, "Degraded"),
            ServiceStatus::Unhealthy => write!(f, "Unhealthy"),
            ServiceStatus::Starting => write!(f, "Starting"),
            ServiceStatus::Stopping => write!(f, "Stopping"),
            ServiceStatus::Stopped => write!(f, "Stopped"),
            ServiceStatus::Error => write!(f, "Error"),
        }
    }
}

/// 服务信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceInfo {
    /// 服务名称
    pub name: String,
    /// 服务版本
    pub version: String,
    /// 服务状态
    pub status: ServiceStatus,
    /// 服务描述
    pub description: String,
    /// 注册时间
    pub registered_at: std::time::SystemTime,
    /// 最后更新时间
    pub updated_at: std::time::SystemTime,
}

impl ServiceInfo {
    /// 创建新的服务信息
    pub fn new<S: Service + ?Sized>(service: &S) -> Self {
        let now = std::time::SystemTime::now();
        Self {
            name: service.name().to_string(),
            version: service.version().to_string(),
            status: service.status(),
            description: service.description().to_string(),
            registered_at: now,
            updated_at: now,
        }
    }

    /// 更新服务信息
    pub fn update<S: Service + ?Sized>(&mut self, service: &S) {
        self.version = service.version().to_string();
        self.status = service.status();
        self.description = service.description().to_string();
        self.updated_at = std::time::SystemTime::now();
    }
}

// 为实现NamedService的类型自动实现Service特征
// 暂时注释以避免测试中的冲突
// impl<T> Service for T
// where
//     T: NamedService,
// {
//     fn name(&self) -> &'static str {
//         Self::NAME
//     }
//
//     fn version(&self) -> &'static str {
//         "1.0.0"
//     }
//
//     fn description(&self) -> &'static str {
//         "Service implementation"
//     }
//
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//
//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }
// }

/// 服务标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceId {
    /// 服务名称
    pub name: String,
    /// 服务版本
    pub version: String,
}

impl ServiceId {
    /// 创建新的服务ID
    pub fn new<S: Service + ?Sized>(service: &S) -> Self {
        Self {
            name: service.name().to_string(),
            version: service.version().to_string(),
        }
    }

    /// 从名称和版本创建服务ID
    pub fn from_parts(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }

    /// 获取服务名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取服务版本
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl fmt::Display for ServiceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.version)
    }
}

/// 服务生命周期特征
pub trait ServiceLifecycle: Service {
    /// 启动服务
    fn start(&mut self) -> ServiceResult<()> {
        Ok(())
    }

    /// 停止服务
    fn stop(&mut self) -> ServiceResult<()> {
        Ok(())
    }

    /// 重启服务
    fn restart(&mut self) -> ServiceResult<()> {
        self.stop()?;
        self.start()
    }

    /// 健康检查
    fn health_check(&self) -> ServiceResult<bool> {
        Ok(self.is_available())
    }
}

/// 服务结果类型
pub type ServiceResult<T> = Result<T, crate::service_registry::ServiceError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[derive(Debug, Clone)]
    struct TestService {
        name: &'static str,
    }

    impl TestService {
        fn new() -> Self {
            Self {
                name: "test-service",
            }
        }
    }

    impl Service for TestService {
        fn name(&self) -> &'static str {
            self.name
        }

        fn version(&self) -> &'static str {
            "1.0.0"
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl NamedService for TestService {
        const NAME: &'static str = "test-service";

        fn clone_owned(&self) -> Self {
            Self::new()
        }
    }

    #[test]
    fn test_service_id() {
        let service = TestService::new();
        let id = ServiceId::new(&service);

        assert_eq!(id.name(), "test-service");
        assert_eq!(id.version(), "1.0.0");
        assert_eq!(id.to_string(), "test-service:1.0.0");
    }

    #[test]
    fn test_service_info() {
        let service = TestService::new();
        let info = ServiceInfo::new(&service);

        assert_eq!(info.name, "test-service");
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.status, ServiceStatus::Healthy);
    }

    #[test]
    fn test_service_status_display() {
        assert_eq!(ServiceStatus::Healthy.to_string(), "Healthy");
        assert_eq!(ServiceStatus::Degraded.to_string(), "Degraded");
        assert_eq!(ServiceStatus::Unhealthy.to_string(), "Unhealthy");
    }
}

//! OpenLark Service 核心特征
//!
//! 定义服务的统一接口和行为

use crate::Result;
use async_trait::async_trait;
use std::time::SystemTime;

/// 🌐 服务基础特征
///
/// 所有服务实现都应该实现此特征
///
/// # 特性要求
/// - 异步支持：所有操作都是异步的
/// - 线程安全：服务可以跨线程安全使用
/// - 生命周期：支持服务的启动、停止和重启
/// - 监控支持：提供健康检查和统计信息
#[async_trait]
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

    /// 📊 获取服务元数据
    fn metadata(&self) -> ServiceMetadata {
        ServiceMetadata {
            name: self.name().to_string(),
            version: self.version().to_string(),
            description: self.description().to_string(),
            api_version: "v1".to_string(),
            endpoints: vec![],
            last_health_check: None,
            health_status: None,
        }
    }

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

/// 🔄 可重启服务特征
///
/// 扩展服务，添加重启功能
#[async_trait]
pub trait RestartableService: ServiceTrait {
    /// 🔄 重启服务
    async fn restart(&self) -> Result<()> {
        tracing::info!("正在重启服务 '{}'", self.name());
        self.stop().await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        self.start().await?;
        tracing::info!("服务 '{}' 重启完成", self.name());
        Ok(())
    }
}

/// 📈 可监控服务特征
///
/// 扩展服务，添加监控和统计功能
#[async_trait]
pub trait MonitorableService: ServiceTrait {
    /// 📊 获取服务统计信息
    async fn stats(&self) -> Result<ServiceStats>;

    /// 🔄 重置统计信息
    async fn reset_stats(&self) -> Result<()>;

    /// 📋 获取服务状态
    async fn status(&self) -> Result<ServiceStatus>;
}

/// 🔧 可配置服务特征
///
/// 扩展服务，添加动态配置能力
#[async_trait]
pub trait ConfigurableService: ServiceTrait {
    /// ⚙️ 配置类型
    type Config: Send + Sync;

    /// 🔧 更新配置
    async fn update_config(&self, config: Self::Config) -> Result<()>;

    /// 🔍 获取当前配置
    async fn get_config(&self) -> Result<Self::Config>;

    /// 🔄 重新加载配置
    async fn reload_config(&self) -> Result<()> {
        // 默认实现：从环境变量重新加载
        tracing::info!("服务 '{}' 重新加载配置", self.name());
        Ok(())
    }
}

/// 📝 可记录服务特征
///
/// 扩展服务，添加日志记录功能
pub trait LoggableService: ServiceTrait {
    /// 📝 记录服务事件
    fn log_event(&self, level: LogLevel, message: &str) {
        let service_name = self.name();
        match level {
            LogLevel::Debug => tracing::debug!("[{}] {}", service_name, message),
            LogLevel::Info => tracing::info!("[{}] {}", service_name, message),
            LogLevel::Warn => tracing::warn!("[{}] {}", service_name, message),
            LogLevel::Error => tracing::error!("[{}] {}", service_name, message),
        }
    }

    /// 🔍 记录服务指标
    fn log_metric(&self, metric_name: &str, value: f64) {
        let service_name = self.name();
        tracing::info!("[{}] {}: {}", service_name, metric_name, value);
    }
}

/// 🌐 可发现服务特征
///
/// 扩展服务，添加服务发现功能
pub trait DiscoverableService: ServiceTrait {
    /// 🏷️ 获取服务标签
    fn tags(&self) -> Vec<&'static str> {
        vec![]
    }

    /// 🔗 获取服务依赖
    fn dependencies(&self) -> Vec<&'static str> {
        vec![]
    }

    /// 📋 获取服务端点
    fn endpoints(&self) -> Vec<ServiceEndpoint> {
        vec![]
    }

    /// 🔍 检查依赖是否满足
    fn dependencies_satisfied(&self) -> bool {
        // 默认实现：所有依赖都需要被满足
        !self.dependencies().is_empty()
    }
}

/// 📊 服务元数据
#[derive(Debug, Clone)]
pub struct ServiceMetadata {
    /// 🏷️ 服务名称
    pub name: String,
    /// 🔢 服务版本
    pub version: String,
    /// 📝 服务描述
    pub description: String,
    /// 🔢 API版本
    pub api_version: String,
    /// 🔗 服务端点列表
    pub endpoints: Vec<String>,
    /// 🏥 最后健康检查时间
    pub last_health_check: Option<SystemTime>,
    /// 🏥 当前健康状态
    pub health_status: Option<bool>,
}

impl ServiceMetadata {
    /// ✅ 标记健康状态为良好
    pub fn mark_healthy(&mut self) {
        self.last_health_check = Some(SystemTime::now());
        self.health_status = Some(true);
    }

    /// ❌ 标记健康状态为不良
    pub fn mark_unhealthy(&mut self) {
        self.last_health_check = Some(SystemTime::now());
        self.health_status = Some(false);
    }

    /// 🔍 检查是否健康
    pub fn is_healthy(&self) -> bool {
        self.health_status.unwrap_or(false)
    }

    /// ⏰ 获取距离上次健康检查的时间
    pub fn time_since_last_health_check(&self) -> Option<std::time::Duration> {
        self.last_health_check?
            .duration_since(SystemTime::now())
            .ok()
    }
}

/// 📊 服务统计信息
#[derive(Debug, Clone, Copy, Default)]
pub struct ServiceStats {
    /// 📡 请求总数
    pub requests_count: u64,
    /// ✅ 成功请求数
    pub success_count: u64,
    /// ❌ 错误请求数
    pub error_count: u64,
    /// ⏱️ 平均响应时间（毫秒）
    pub average_response_time: f64,
    /// 🕐 最后请求时间
    pub last_request_time: Option<SystemTime>,
    /// 🏃‍♂️ 当前并发数
    pub current_concurrency: u64,
    /// 📈 峰值并发数
    pub peak_concurrency: u64,
    /// ⏰ 启动时间
    pub start_time: Option<SystemTime>,
    /// ⏱️ 运行时间
    pub uptime: Option<std::time::Duration>,
}

impl ServiceStats {
    /// 📊 计算成功率
    pub fn success_rate(&self) -> f64 {
        if self.requests_count == 0 {
            0.0
        } else {
            self.success_count as f64 / self.requests_count as f64 * 100.0
        }
    }

    /// 📊 计算错误率
    pub fn error_rate(&self) -> f64 {
        if self.requests_count == 0 {
            0.0
        } else {
            self.error_count as f64 / self.requests_count as f64 * 100.0
        }
    }

    /// 🔄 更新运行时间
    pub fn update_uptime(&mut self) {
        if let Some(start_time) = self.start_time {
            self.uptime = start_time.elapsed().ok();
        }
    }

    /// 📊 获取QPS（每秒请求数）
    pub fn qps(&self) -> f64 {
        if let Some(start_time) = self.start_time {
            if let Ok(elapsed) = start_time.elapsed() {
                let seconds = elapsed.as_secs_f64();
                if seconds > 0.0 {
                    self.requests_count as f64 / seconds
                } else {
                    0.0
                }
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

/// 📋 服务状态
#[derive(Debug, Clone)]
pub enum ServiceStatus {
    /// 🚀 运行中
    Running,
    /// 🛑 已停止
    Stopped,
    /// 🔄 启动中
    Starting,
    /// 🛑 停止中
    Stopping,
    /// ⚠️ 错误状态
    Error(String),
    /// ❓ 未知状态
    Unknown,
}

impl ServiceStatus {
    /// 🔍 检查是否为活跃状态
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            ServiceStatus::Running | ServiceStatus::Starting | ServiceStatus::Stopping
        )
    }

    /// 🔍 检查是否为健康状态
    pub fn is_healthy(&self) -> bool {
        matches!(self, ServiceStatus::Running)
    }

    /// 📝 获取状态描述
    pub fn description(&self) -> &'static str {
        match self {
            ServiceStatus::Running => "运行中",
            ServiceStatus::Stopped => "已停止",
            ServiceStatus::Starting => "启动中",
            ServiceStatus::Stopping => "停止中",
            ServiceStatus::Error(_) => "错误",
            ServiceStatus::Unknown => "未知",
        }
    }
}

/// 🔗 服务端点
#[derive(Debug, Clone)]
pub struct ServiceEndpoint {
    /// 🏷️ 端点名称
    pub name: String,
    /// 🔗 端点URL
    pub url: String,
    /// 📝 端点描述
    pub description: String,
    /// 🔢 API版本
    pub version: String,
    /// 🏷️ 端点标签
    pub tags: Vec<String>,
}

impl ServiceEndpoint {
    /// 🆕 创建新的服务端点
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            description: String::new(),
            version: "v1".to_string(),
            tags: Vec::new(),
        }
    }

    /// 📝 设置端点描述
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// 🔢 设置API版本
    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    /// 🏷️ 添加标签
    pub fn add_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
}

/// 📝 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// 🔍 调试信息
    Debug,
    /// ℹ️ 一般信息
    Info,
    /// ⚠️ 警告信息
    Warn,
    /// ❌ 错误信息
    Error,
}

impl LogLevel {
    /// 📝 获取级别名称
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 创建一个测试用的服务实现
    struct TestService {
        name: &'static str,
    }

    #[async_trait]
    impl ServiceTrait for TestService {
        fn name(&self) -> &'static str {
            self.name
        }

        async fn health_check(&self) -> Result<bool> {
            Ok(true)
        }
    }

    #[test]
    fn test_service_metadata() {
        let mut metadata = ServiceMetadata {
            name: "test_service".to_string(),
            version: "1.0.0".to_string(),
            description: "Test service".to_string(),
            api_version: "v1".to_string(),
            endpoints: vec!["/test".to_string()],
            last_health_check: None,
            health_status: None,
        };

        assert!(!metadata.is_healthy());

        metadata.mark_healthy();
        assert!(metadata.is_healthy());

        metadata.mark_unhealthy();
        assert!(!metadata.is_healthy());
    }

    #[test]
    fn test_service_stats() {
        let mut stats = ServiceStats {
            requests_count: 100,
            success_count: 95,
            error_count: 5,
            average_response_time: 150.0,
            ..Default::default()
        };

        assert_eq!(stats.success_rate(), 95.0);
        assert_eq!(stats.error_rate(), 5.0);

        stats.update_uptime();
        // 运行时间应该仍然为None，因为没有设置start_time
        assert!(stats.uptime.is_none());
    }

    #[test]
    fn test_service_status() {
        let running = ServiceStatus::Running;
        assert!(running.is_active());
        assert!(running.is_healthy());
        assert_eq!(running.description(), "运行中");

        let stopped = ServiceStatus::Stopped;
        assert!(!stopped.is_active());
        assert!(!stopped.is_healthy());
        assert_eq!(stopped.description(), "已停止");

        let error = ServiceStatus::Error("Connection failed".to_string());
        assert!(!error.is_active());
        assert!(!error.is_healthy());
        assert_eq!(error.description(), "错误");
    }

    #[test]
    fn test_service_endpoint() {
        let endpoint = ServiceEndpoint::new("test", "https://api.example.com/test")
            .description("Test endpoint")
            .version("v2")
            .add_tag("api");

        assert_eq!(endpoint.name, "test");
        assert_eq!(endpoint.url, "https://api.example.com/test");
        assert_eq!(endpoint.description, "Test endpoint");
        assert_eq!(endpoint.version, "v2");
        assert!(endpoint.tags.contains(&"api".to_string()));
    }

    #[test]
    fn test_log_level() {
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);

        assert_eq!(LogLevel::Debug.as_str(), "DEBUG");
        assert_eq!(LogLevel::Info.as_str(), "INFO");
        assert_eq!(LogLevel::Warn.as_str(), "WARN");
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
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

        let metadata = service.metadata();
        assert_eq!(metadata.name, "test_service");
        assert_eq!(metadata.version, "1.0.0");
    }
}

/// 🔄 服务生命周期特征
///
/// 定义服务的启动、停止和健康检查生命周期管理
#[async_trait]
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

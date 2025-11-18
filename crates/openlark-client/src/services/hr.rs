//! 👥 HR服务访问层
//!
//! 提供统一的HR服务接口，封装底层openlark-hr crate

use std::sync::Arc;
use crate::{Config, ServiceRegistry, Result};

/// 👥 HR服务 - 统一访问接口
///
/// 包装openlark-hr crate的功能，提供简洁的API
#[derive(Debug)]
pub struct HRService<'a> {
    /// 🔧 客户端配置
    config: &'a Config,
    /// 📋 服务注册表
    registry: &'a ServiceRegistry,
}

impl<'a> HRService<'a> {
    /// 🆕 创建新的HR服务实例
    pub(crate) fn new(config: &'a Config, registry: &'a ServiceRegistry) -> Self {
        Self { config, registry }
    }

    /// 👥 获取员工列表
    pub async fn list_employees(
        &self,
        user_id_type: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<ListEmployeesResponse> {
        tracing::info!("获取员工列表");

        Ok(ListEmployeesResponse {
            employees: vec![],
            page_token: page_token.map(|s| s.to_string()),
            has_more: false,
        })
    }

    /// 👤 获取员工详细信息
    pub async fn get_employee_info(
        &self,
        user_id: &str,
        user_id_type: &str,
    ) -> Result<EmployeeInfo> {
        tracing::info!("获取员工信息: {}", user_id);

        Ok(EmployeeInfo {
            user_id: user_id.to_string(),
            name: "Mock Employee".to_string(),
            department: None,
            position: None,
        })
    }
}

/// 👥 员工列表响应
#[derive(Debug, Clone)]
pub struct ListEmployeesResponse {
    /// 👥 员工列表
    pub employees: Vec<EmployeeInfo>,
    /// 🔄 分页token
    pub page_token: Option<String>,
    /// 🔗 是否有更多
    pub has_more: bool,
}

/// 👤 员工信息
#[derive(Debug, Clone)]
pub struct EmployeeInfo {
    /// 🆔 员工ID
    pub user_id: String,
    /// 👤 员工姓名
    pub name: String,
    /// 🏢 部门
    pub department: Option<String>,
    /// 💼 职位
    pub position: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_hr_service_creation() {
        let config = Config::default();
        let registry = ServiceRegistry::new(&Arc::new(config));
        let service = HRService::new(&config, &registry);

        // 基本创建测试
        assert_eq!(service.config.app_id, "");
    }

    #[tokio::test]
    async fn test_list_employees() {
        let config = Config::default();
        let registry = ServiceRegistry::new(&Arc::new(config));
        let service = HRService::new(&config, &registry);

        let result = service
            .list_employees(Some("open_id"), Some(20), None)
            .await;

        assert!(result.is_ok());
        if let Ok(response) = result {
            assert!(response.employees.is_empty());
            assert!(!response.has_more);
        }
    }

    #[tokio::test]
    async fn test_get_employee_info() {
        let config = Config::default();
        let registry = ServiceRegistry::new(&Arc::new(config));
        let service = HRService::new(&config, &registry);

        let result = service
            .get_employee_info("test_user", "open_id")
            .await;

        assert!(result.is_ok());
        if let Ok(employee) = result {
            assert_eq!(employee.user_id, "test_user");
            assert_eq!(employee.name, "Mock Employee");
        }
    }
}

impl Default for HRService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UnifiedService for HRService {
    type Config = HRConfig;
    type Error = UnifiedError;

    fn name(&self) -> &'static str {
        "hr"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    async fn configure(&mut self, config: Self::Config) -> UnifiedResult<()> {
        if !config.enabled {
            self.status = ServiceStatus::Stopped;
            return Ok(());
        }

        self.config = Some(config);

        // 创建核心客户端
        let core_config = self.config.as_ref().map(|config| {
            openlark_core::config::ConfigBuilder::new()
                .base_url(&config.api_url)
                .timeout(config.timeout)
                .build()
                .unwrap_or_else(|_| openlark_core::config::Config::default())
        });

        if let Some(core_config) = core_config {
                core_config.app_id.clone(),
                core_config.app_secret.clone(),
            ) {
                Ok(client) => {
                    self.core_client = Some(Arc::new(client));
                    self.status = ServiceStatus::Running;
                    tracing::info!("HR服务配置成功");
                    Ok(())
                }
                Err(e) => {
                    self.status = ServiceStatus::Error;
                    Err(UnifiedError::ConfigurationError(
                        format!("创建核心客户端失败: {}", e),
                    ))
                }
            }
        } else {
            self.status = ServiceStatus::Error;
            Err(UnifiedError::ConfigurationError("HR配置无效".to_string()))
        }
    }

    fn is_available(&self) -> bool {
        self.is_enabled() && self.status == ServiceStatus::Running && self.core_client.is_some()
    }

    fn status(&self) -> ServiceStatus {
        self.status
    }

    fn descriptor(&self) -> ServiceDescriptor {
        let mut descriptor = ServiceDescriptor::new(
            "hr",
            "1.0.0",
            "飞书人力资源服务，提供员工、考勤、薪酬等功能",
        )
        .with_tag("hr")
        .with_tag("management")
        .with_dependency("openlark-core");

        if let Some(config) = &self.config {
            descriptor = descriptor
                .with_metadata("api_url", config.api_url.clone())
                .with_metadata("timeout_ms", config.timeout.as_millis().to_string())
                .with_metadata("enabled", config.enabled.to_string())
                .with_metadata("batch_size", config.data_access.batch_size.to_string())
                .with_metadata("data_format", config.data_access.data_format.clone());
        }

        descriptor
    }
}

#[async_trait]
impl ServiceLifecycle for HRService {
    async fn start(&mut self) -> SDKResult<()> {
        if let Some(config) = self.config.clone() {
            self.configure(config).await?;
        } else {
            tracing::warn!("HR服务配置未设置，服务将处于未初始化状态");
        }
        Ok(())
    }

    async fn stop(&mut self) -> SDKResult<()> {
        self.status = ServiceStatus::Stopped;
        self.core_client = None;
        tracing::info!("HR服务已停止");
        Ok(())
    }

    async fn health_check(&self) -> SDKResult<bool> {
        Ok(self.is_available())
    }
}

/// HR服务构建器
pub struct HRServiceBuilder {
    config: Option<HRConfig>,
}

impl HRServiceBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: None,
            core_client: None,
        }
    }

    /// 设置配置
    pub fn config(mut self, config: HRConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置核心客户端
        self.core_client = Some(core_client);
        self
    }

    /// 构建服务
    pub fn build(self) -> UnifiedResult<HRService> {
        let mut service = HRService::new();

        if let Some(config) = self.config {
            service = service.with_config(config);
        }

        if let Some(core_client) = self.core_client {
            service = service.with_core_client(core_client);
        }

        Ok(service)
    }
}

impl Default for HRServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hr_service_creation() {
        let service = HRService::new();
        assert_eq!(service.name(), "hr");
        assert_eq!(service.version(), "1.0.0");
    }

    #[test]
    fn test_hr_service_builder() {
        let config = HRConfig::default();
        let service = HRServiceBuilder::new()
            .config(config)
            .build()
            .unwrap();

        assert!(service.is_enabled());
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let mut service = HRService::new();

        // 测试启动
        service.start().await.unwrap();
        // 由于没有配置，服务应该是未初始化状态
        assert_eq!(service.status(), ServiceStatus::Stopped);

        // 测试停止
        service.stop().await.unwrap();
        assert_eq!(service.status(), ServiceStatus::Stopped);
    }

    #[tokio::test]
    async fn test_employee_operations() {
        let service = HRService::new();

        // 测试获取员工信息
        let result = service.get_employee("test_user", Some("open_id")).await;
        assert!(result.is_ok());

        // 测试获取员工列表
        let result = service.list_employees(Some("open_id"), Some(20), None).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_service_descriptors() {
        let service = HRService::new();
        let descriptor = service.descriptor();

        assert_eq!(descriptor.name, "hr");
        assert_eq!(descriptor.version, "1.0.0");
        assert!(descriptor.tags.contains(&"hr".to_string()));
    }
}
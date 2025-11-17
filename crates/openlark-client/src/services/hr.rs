//! HR服务适配器
//!
//! 将openlark-hr服务适配到统一客户端接口。

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::unified::{
    traits::{UnifiedService, ServiceDescriptor, ServiceStatus, ServiceLifecycle},
    config::{UnifiedConfig, HRConfig},
    error::{UnifiedError, UnifiedResult},
};

/// HR服务适配器
///
/// 将openlark-hr的功能适配到统一客户端接口。
#[derive(Debug, Clone)]
pub struct HRService {
    /// 服务配置
    config: Option<HRConfig>,
    /// 服务状态
    status: ServiceStatus,
    /// 核心客户端（用于实际API调用）
    core_client: Option<Arc<openlark_core::client::LarkClient>>,
    /// 服务元数据
    metadata: HashMap<String, String>,
}

impl HRService {
    /// 创建新的HR服务适配器
    pub fn new() -> Self {
        Self {
            config: None,
            status: ServiceStatus::Uninitialized,
            core_client: None,
            metadata: HashMap::new(),
        }
    }

    /// 从配置创建服务
    pub fn with_config(mut self, config: HRConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 从核心客户端创建服务
    pub fn with_core_client(mut self, core_client: Arc<openlark_core::client::LarkClient>) -> Self {
        self.core_client = Some(core_client);
        self
    }

    /// 检查服务是否可用
    pub fn is_enabled(&self) -> bool {
        self.config
            .as_ref()
            .map(|config| config.enabled)
            .unwrap_or(false)
    }

    /// 获取员工列表
    pub async fn list_employees(
        &self,
        user_id_type: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> UnifiedResult<EmployeeListResult> {
        self.ensure_available()?;

        let list_request = serde_json::json!({
            "user_id_type": user_id_type.unwrap_or("open_id"),
            "page_size": page_size.unwrap_or(50),
            "page_token": page_token
        });

        tracing::info!("获取员工列表: {:?}", list_request);

        Ok(EmployeeListResult {
            employees: vec![],
            page_token: None,
            has_more: false,
        })
    }

    /// 获取单个员工信息
    pub async fn get_employee(&self, user_id: &str, user_id_type: Option<&str>) -> UnifiedResult<Employee> {
        self.ensure_available()?;

        let get_request = serde_json::json!({
            "user_id": user_id,
            "user_id_type": user_id_type.unwrap_or("open_id")
        });

        tracing::info!("获取员工信息: {:?}", get_request);

        // 模拟返回员工数据
        Ok(Employee {
            user_id: user_id.to_string(),
            name: "Mock Employee".to_string(),
            email: Some("mock@example.com".to_string()),
            mobile: Some("13800138000".to_string()),
            department_ids: vec!["dept_001".to_string()],
            position: Some("Developer".to_string()),
            employee_type: Some("full_time".to_string()),
            join_time: chrono::Utc::now(),
            status: EmployeeStatus::Active,
        })
    }

    /// 获取考勤组列表
    pub async fn list_attendance_groups(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> UnifiedResult<AttendanceGroupListResult> {
        self.ensure_available()?;

        let list_request = serde_json::json!({
            "page_size": page_size.unwrap_or(50),
            "page_token": page_token
        });

        tracing::info!("获取考勤组列表: {:?}", list_request);

        Ok(AttendanceGroupListResult {
            groups: vec![],
            page_token: None,
            has_more: false,
        })
    }

    /// 获取考勤记录
    pub async fn get_attendance_records(
        &self,
        user_id: &str,
        from_date: chrono::NaiveDate,
        to_date: chrono::NaiveDate,
    ) -> UnifiedResult<AttendanceRecordListResult> {
        self.ensure_available()?;

        let records_request = serde_json::json!({
            "user_id": user_id,
            "from_date": from_date.to_string(),
            "to_date": to_date.to_string()
        });

        tracing::info!("获取考勤记录: {:?}", records_request);

        Ok(AttendanceRecordListResult {
            records: vec![],
        })
    }

    /// 确保服务可用
    fn ensure_available(&self) -> UnifiedResult<()> {
        if !self.is_enabled() {
            return Err(UnifiedError::ServiceNotAvailable("hr".to_string()));
        }

        if self.status != ServiceStatus::Running {
            return Err(UnifiedError::ServiceNotAvailable(
                "hr service not running".to_string(),
            ));
        }

        Ok(())
    }
}

/// 员工信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Employee {
    /// 用户ID
    pub user_id: String,
    /// 姓名
    pub name: String,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 部门ID列表
    pub department_ids: Vec<String>,
    /// 职位
    pub position: Option<String>,
    /// 员工类型
    pub employee_type: Option<String>,
    /// 入职时间
    pub join_time: chrono::DateTime<chrono::Utc>,
    /// 员工状态
    pub status: EmployeeStatus,
}

/// 员工状态
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmployeeStatus {
    /// 在职
    Active,
    /// 离职
    Inactive,
    /// 停薪留职
    Suspended,
}

/// 员工列表结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmployeeListResult {
    /// 员工列表
    pub employees: Vec<Employee>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否还有更多
    pub has_more: bool,
}

/// 考勤组
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AttendanceGroup {
    /// 考勤组ID
    pub group_id: String,
    /// 考勤组名称
    pub name: String,
    /// 考勤组类型
    pub group_type: String,
    /// 考勤时段设置
    pub shift_settings: Vec<ShiftSetting>,
}

/// 考勤时段设置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShiftSetting {
    /// 时段ID
    pub shift_id: String,
    /// 时段名称
    pub name: String,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
}

/// 考勤组列表结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AttendanceGroupListResult {
    /// 考勤组列表
    pub groups: Vec<AttendanceGroup>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否还有更多
    pub has_more: bool,
}

/// 考勤记录
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AttendanceRecord {
    /// 记录ID
    pub record_id: String,
    /// 用户ID
    pub user_id: String,
    /// 考勤日期
    pub attendance_date: String,
    /// 打卡时间
    pub check_times: Vec<CheckTime>,
    /// 工作时长（分钟）
    pub work_duration_minutes: Option<u32>,
}

/// 打卡时间
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckTime {
    /// 打卡时间
    pub check_time: chrono::DateTime<chrono::Utc>,
    /// 打卡类型
    pub check_type: String,
    /// 位置信息
    pub location: Option<String>,
}

/// 考勤记录列表结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AttendanceRecordListResult {
    /// 考勤记录列表
    pub records: Vec<AttendanceRecord>,
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
            match openlark_core::client::LarkClient::new(
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
    core_client: Option<Arc<openlark_core::client::LarkClient>>,
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
    pub fn core_client(mut self, core_client: Arc<openlark_core::client::LarkClient>) -> Self {
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
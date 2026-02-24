//! Open-Lark HR Module
//!
//! 飞书人力资源服务模块，提供完整的人力资源管理功能。
//!
//! ## 主要功能
//!
//! - **考勤管理**: 考勤组、班次、用户任务、统计数据、请假加班管理
//! - **核心人力资源**: 员工、岗位、部门、合同、工作地点管理
//! - **OKR管理**: 目标与关键成果的制定、跟踪和评估
//! - **薪资管理**: 薪资组、薪资调整、工资单、薪资条目管理
//! - **绩效管理**: 绩效周期、评估、反馈、目标管理
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use openlark_hr::prelude::*;
//! use openlark_hr::HrClient;
//!
//! let client = HrClient::new(config);
//! // 链式调用
//! client.attendance().v1().group().create()
//!     .group_name("技术部".to_string())
//!     .execute()
//!     .await?;
//! ```
//!
//! ## API 端点
//!
//! 推荐使用枚举类型的端点系统（位于 `common::api_endpoints`）：
//! - `AttendanceApiV1` - 考勤管理
//! - `HireApiV1` - 招聘管理
//! - `FeishuPeopleApiV1` / `FeishuPeopleApiV2` - 核心人力资源
//! - `OkrApiV1` - OKR管理
//! - `PayrollApiV1` - 薪资管理
//! - `PerformanceApiV1` - 绩效管理

#![allow(missing_docs)]

pub mod common;

#[cfg(feature = "attendance")]
pub mod attendance;
#[cfg(feature = "compensation")]
pub mod compensation_management;
#[cfg(feature = "ehr")]
pub mod ehr;
#[cfg(feature = "corehr")]
pub mod feishu_people;
#[cfg(feature = "hire")]
pub mod hire;
#[cfg(feature = "okr")]
pub mod okr;
#[cfg(feature = "payroll")]
pub mod payroll;
#[cfg(feature = "performance")]
pub mod performance;

// 端点保留（已废弃，请使用 common::api_endpoints 中的枚举系统）
#[allow(deprecated)]
mod endpoints;

pub mod prelude {
    pub use openlark_core::{config::Config, SDKResult};
}

use openlark_core::config::Config;
use std::sync::Arc;

/// HRClient：统一入口，提供 project-version-resource 链式访问
#[derive(Clone)]
pub struct HrClient {
    #[allow(dead_code)]
    config: Arc<Config>,
}


impl HrClient {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[cfg(feature = "attendance")]
    pub fn attendance(&self) -> attendance::Attendance {
        attendance::Attendance::new((*self.config).clone())
    }

    #[cfg(feature = "corehr")]
    pub fn corehr(&self) -> feishu_people::Corehr {
        feishu_people::Corehr::new((*self.config).clone())
    }

    #[cfg(feature = "compensation")]
    pub fn compensation(&self) -> compensation_management::CompensationManagement {
        compensation_management::CompensationManagement::new((*self.config).clone())
    }

    #[cfg(feature = "payroll")]
    pub fn payroll(&self) -> payroll::Payroll {
        payroll::Payroll::new((*self.config).clone())
    }

    #[cfg(feature = "performance")]
    pub fn performance(&self) -> performance::Performance {
        performance::Performance::new((*self.config).clone())
    }

    #[cfg(feature = "okr")]
    pub fn okr(&self) -> okr::Okr {
        okr::Okr::new((*self.config).clone())
    }

    #[cfg(feature = "hire")]
    pub fn hire(&self) -> hire::Hire {
        hire::Hire::new((*self.config).clone())
    }

    #[cfg(feature = "ehr")]
    pub fn ehr(&self) -> ehr::Ehr {
        ehr::Ehr::new((*self.config).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_hr_client_creation() {
        let config = create_test_config();
        let client = HrClient::new(config);
        assert!(client.config.app_id() == "test_app");
    }

    #[test]
    fn test_hr_client_clone() {
        let config = create_test_config();
        let client = HrClient::new(config);
        let cloned = client.clone();
        assert!(cloned.config.app_id() == "test_app");
    }

    #[cfg(feature = "attendance")]
    #[test]
    fn test_hr_client_attendance() {
        let config = create_test_config();
        let client = HrClient::new(config);
        let _attendance = client.attendance();
    }

    #[cfg(feature = "corehr")]
    #[test]
    fn test_hr_client_corehr() {
        let config = create_test_config();
        let client = HrClient::new(config);
        let _corehr = client.corehr();
    }

    #[cfg(feature = "compensation")]
    #[test]
    fn test_hr_client_compensation() {
        let config = create_test_config();
        let client = HrClient::new(config);
        let _compensation = client.compensation();
    }

    #[cfg(feature = "payroll")]
    #[test]
    fn test_hr_client_payroll() {
        let config = create_test_config();
        let client = HrClient::new(config);
        let _payroll = client.payroll();
    }

    #[cfg(feature = "performance")]
    #[test]
    fn test_hr_client_performance() {
        let config = create_test_config();
        let client = HrClient::new(config);
        let _performance = client.performance();
    }

    #[cfg(feature = "okr")]
    #[test]
    fn test_hr_client_okr() {
        let config = create_test_config();
        let client = HrClient::new(config);
        let _okr = client.okr();
    }

    #[cfg(feature = "hire")]
    #[test]
    fn test_hr_client_hire() {
        let config = create_test_config();
        let client = HrClient::new(config);
        let _hire = client.hire();
    }

    #[cfg(feature = "ehr")]
    #[test]
    fn test_hr_client_ehr() {
        let config = create_test_config();
        let client = HrClient::new(config);
        let _ehr = client.ehr();
    }
}

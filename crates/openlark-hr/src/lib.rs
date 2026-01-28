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

pub mod attendance;
pub mod compensation_management;
pub mod ehr;
pub mod feishu_people;
pub mod hire;
pub mod okr;
pub mod payroll;
pub mod performance;

// 端点保留（已废弃，请使用 common::api_endpoints 中的枚举系统）
#[allow(deprecated)]
mod endpoints;

pub mod prelude {
    pub use openlark_core::{config::Config, SDKResult};
}

use common::service::HrService;
use std::sync::Arc;

/// HRClient：统一入口，提供 project-version-resource 链式访问
#[derive(Clone)]
pub struct HrClient {
    service: Arc<HrService>,
}

impl HrClient {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            service: Arc::new(HrService::new(config)),
        }
    }

    pub fn attendance(&self) -> attendance::Attendance {
        attendance::Attendance::new(self.service.config().clone())
    }

    pub fn corehr(&self) -> feishu_people::Corehr {
        feishu_people::Corehr::new(self.service.config().clone())
    }

    pub fn compensation(&self) -> compensation_management::CompensationManagement {
        compensation_management::CompensationManagement::new(self.service.config().clone())
    }

    pub fn payroll(&self) -> payroll::Payroll {
        payroll::Payroll::new(self.service.config().clone())
    }

    pub fn performance(&self) -> performance::Performance {
        performance::Performance::new(self.service.config().clone())
    }

    pub fn okr(&self) -> okr::Okr {
        okr::Okr::new(self.service.config().clone())
    }

    pub fn hire(&self) -> hire::Hire {
        hire::Hire::new(self.service.config().clone())
    }

    pub fn ehr(&self) -> ehr::Ehr {
        ehr::Ehr::new(self.service.config().clone())
    }
}

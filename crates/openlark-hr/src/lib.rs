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
//! ```rust
//! use openlark_hr::endpoints::*;
//!
//! // 使用端点常量
//! let employees_endpoint = COREHR_V1_EMPLOYEES;
//! let attendance_endpoint = ATTENDANCE_V1_GROUPS;
//! println!("员工端点: {}", employees_endpoint);
//! println!("考勤端点: {}", attendance_endpoint);
//! ```
//!
//! ## 端点组织
//!
//! - `attendance`: 考勤相关API端点
//! - `corehr`: 核心人力资源API端点
//! - `okr`: 目标管理API端点
//! - `payroll`: 薪资管理API端点
//! - `performance`: 绩效管理API端点

#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

// Core service
pub mod service;

// 项目模块（按 meta.Project）
pub mod attendance;
pub mod compensation;
pub mod corehr;
pub mod ehr;
pub mod hire;
pub mod okr;
pub mod payroll;
pub mod performance;

// 端点保留
pub mod endpoints;
pub use endpoints::*;

pub mod prelude {
    pub use openlark_core::{config::Config, SDKResult};
}

use service::HrService;
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
        attendance::Attendance::new(self.service.clone())
    }

    pub fn corehr(&self) -> corehr::Corehr {
        corehr::Corehr::new(self.service.clone())
    }

    pub fn compensation(&self) -> compensation::Compensation {
        compensation::Compensation::new(self.service.clone())
    }

    pub fn payroll(&self) -> payroll::Payroll {
        payroll::Payroll::new(self.service.clone())
    }

    pub fn performance(&self) -> performance::Performance {
        performance::Performance::new(self.service.clone())
    }

    pub fn okr(&self) -> okr::Okr {
        okr::Okr::new(self.service.clone())
    }

    pub fn hire(&self) -> hire::Hire {
        hire::Hire::new(self.service.clone())
    }

    pub fn ehr(&self) -> ehr::Ehr {
        ehr::Ehr::new(self.service.clone())
    }
}

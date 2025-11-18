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

// 导入端点模块
pub mod endpoints;

pub mod compensation_management;
pub mod hire;

// 重新导出端点常量，方便外部使用
pub use endpoints::*;

pub mod prelude {
    #[allow(ambiguous_glob_reexports)]
    pub use crate::compensation_management::*;
    #[allow(ambiguous_glob_reexports)]
    pub use crate::hire::*;
    pub use openlark_core::SDKResult;
}

#[allow(ambiguous_glob_reexports)]
pub use crate::compensation_management::*;
#[allow(ambiguous_glob_reexports)]
pub use crate::hire::*;

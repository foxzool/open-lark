#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Attendance服务模块
//!
//! 提供飞书考勤相关的API功能，包括：
//! - 班次管理
//! - 考勤任务查询
//! - 考勤统计数据
//! - 请假管理
//! - 考勤审批

use crate::config::Config;

/// Attendance服务
#[derive(Debug, Clone)]
pub struct AttendanceService {
    pub config: Config,
    pub v1: v1::AttendanceServiceV1,
}

impl AttendanceService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::AttendanceServiceV1::new(config),
        }
    }
}

pub mod v1;

//! EHR人力资源服务模块
//!
//! 提供飞书企业人力资源管理的完整功能，包括：
//! - 员工信息管理：创建、查询、更新、删除员工档案
//! - 组织架构管理：部门层级、职位体系、员工关系
//! - 薪酬福利管理：薪资结构、津贴扣款、薪资调整
//! - 考勤记录管理：打卡记录、考勤统计、异常处理
//! - 绩效评估系统：评估维度、评分体系、改进建议

use crate::core::config::Config;

/// EHR服务
#[derive(Debug, Clone)]
pub struct EhrService {
    pub config: Config,
    pub v1: v1::EhrServiceV1,
}

impl EhrService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::EhrServiceV1::new(config),
        }
    }
}

pub mod v1;

// 重新导出所有模块和类型
pub use v1::*;

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Core - 核心人事管理服务
//!
//! 提供核心人事管理相关的所有功能，包括：
//! - 人员信息管理（创建、更新、查询、删除）
//! - 部门管理
//! - 职位管理（职务、职务序列、职务级别）
//! - 合同管理
//! - 公司管理
//! - 异动管理

use crate::config::Config;

/// 核心人事管理服务
#[derive(Debug, Clone)]
pub struct CoreService {
    pub config: Config,
    pub v1: v1::CoreV1Service,
}

impl CoreService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::CoreV1Service::new(config),
        }
    }
}

/// v1版本API
pub mod v1;

// 重新导出所有模块和类型
pub use v1::*;

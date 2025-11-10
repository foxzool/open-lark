#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! OKR服务模块
//!
//! 提供企业级OKR（目标与关键结果）管理功能，包括：
//! - OKR周期管理（季度、年度等）
//! - 目标(Objective)和关键结果(Key Result)的创建与管理
//! - 进展跟踪和记录
//! - OKR复盘和评分
//! - 多语言支持

use open_lark_core::config::Config;

/// OKR服务
#[derive(Debug, Clone)]
pub struct OkrService {
    pub config: Config,
    pub v1: v1::OkrServiceV1,
}

impl OkrService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::OkrServiceV1::new(config),
        }
    }
}

pub mod models;
pub mod v1;

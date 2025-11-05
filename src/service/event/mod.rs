#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Event服务模块
//!
//! 提供飞书事件相关的API功能，包括：
//! - 事件出口IP查询
//! - 事件订阅管理
//! - 回调地址管理

use crate::core::config::Config;

/// Event服务
#[derive(Debug, Clone)]
pub struct EventService {
    pub config: Config,
    pub v1: v1::EventServiceV1,
}

impl EventService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::EventServiceV1::new(config),
        }
    }
}

pub mod v1;
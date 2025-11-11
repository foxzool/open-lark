#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Calendar服务模块
//!
//! 提供飞书日历相关的API功能，包括：
//! - 日程创建和管理
//! - 日历信息查询
//! - 参与人和会议室管理
//! - 忙闲时间查询

use open_lark_core::config::Config;

/// Calendar服务
#[derive(Debug, Clone)]
pub struct CalendarService {
    pub config: Config,
    pub v4: v4::CalendarServiceV4,
}

impl CalendarService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v4: v4::CalendarServiceV4::new(config),
        }
    }
}

pub mod v4;

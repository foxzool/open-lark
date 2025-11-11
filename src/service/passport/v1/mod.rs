//! Passport v1 API 服务
//!
//! 提供飞书开放平台护照相关API的v1版本实现，包括会话管理等功能。

use open_lark_core::config::Config;
use crate::prelude::*;
use crate::service::passport::v1::sessions::SessionsService;

/// Passport v1 服务
#[derive(Debug, Clone)]
pub struct PassportV1Service {
    /// 服务配置
    pub config: Config,
    /// 会话管理服务
    pub sessions: SessionsService,
}

impl PassportV1Service {
    /// 创建新的Passport v1服务实例
    pub fn new(config: Config) -> Self {
        Self {
            sessions: SessionsService::new(config.clone()),
            config,
        }
    }
}

pub mod sessions;

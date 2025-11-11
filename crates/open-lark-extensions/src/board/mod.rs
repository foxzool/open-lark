//! 画板服务
//!
//! 提供飞书画板相关的API功能，包括：
//! - 画板主题管理
//! - 画板缩略图获取
//! - 节点创建和管理
//! - 画板内容查询

use openlark_core::config::Config;

/// 画板服务
#[derive(Debug, Clone)]
pub struct BoardService {
    pub config: Config,
    pub v1: v1::BoardServiceV1,
}

impl BoardService {
    /// 创建画板服务实例
    pub fn new(config: Config) -> Self {
        let v1 = v1::BoardServiceV1::new(config.clone());
        Self { config, v1 }
    }
}

/// v1版本API
pub mod v1;

pub use v1::*;

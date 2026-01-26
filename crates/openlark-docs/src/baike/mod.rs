#![allow(clippy::module_inception)]

/// Baike 模块
pub mod baike;
pub mod lingo;
pub mod models;
pub mod v1;

// 使用通配符导出所有子模块
pub use baike::*;
pub use lingo::*;
pub use models::*;
pub use v1::*;

use openlark_core::config::Config;

/// Baike 知识库服务
#[derive(Debug, Clone)]
pub struct BaikeService {
    config: Config,
}

impl BaikeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

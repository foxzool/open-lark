//! # 群管理服务模块
//!
//! 本模块包含飞书群聊相关的所有管理功能。
//!
//! ## 规划中的功能模块：
//!
//! - `create`: 创建群聊
//! - `get`: 获取群信息
//! - `list`: 获取群列表
//! - `delete`: 解散群聊
//! - `update`: 更新群信息
//! - `search`: 搜索群聊
//! - `put_top_notice`: 设置群置顶
//! - `delete_top_notice`: 撤销群置顶
//! - `link`: 获取群分享链接
//!
//! 🚧 **待实现** - 以上功能模块尚未实现，敬请期待。

use crate::core::{config::Config, trait_system::Service};

// 规划中的功能模块（待实现）
// pub mod create;
// pub mod get;
// pub mod list;
// pub mod delete;
// pub mod update;
// pub mod search;
// pub mod put_top_notice;
// pub mod delete_top_notice;
// pub mod link;

/// 群管理服务
///
/// 提供群的创建、删除、更新、查询等管理功能
pub struct ChatService {
    pub config: Config,
}

impl ChatService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Service for ChatService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "chat"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

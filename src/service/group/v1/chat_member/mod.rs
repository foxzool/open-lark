//! # 群成员管理服务模块
//!
//! 本模块包含飞书群聊成员相关的所有管理功能。
//!
//! ## 规划中的功能模块：
//!
//! - `create`: 添加群成员
//! - `get`: 获取群成员信息
//! - `delete`: 移除群成员
//! - `add_managers`: 指定群管理员
//! - `delete_managers`: 删除群管理员
//! - `me_join`: 主动加入群聊
//! - `is_in_chat`: 判断是否在群里
//!
//! 🚧 **待实现** - 以上功能模块尚未实现，敬请期待。

use crate::core::config::Config;

// 规划中的功能模块（待实现）
// pub mod create;
// pub mod get;
// pub mod delete;
// pub mod add_managers;
// pub mod delete_managers;
// pub mod me_join;
// pub mod is_in_chat;

/// 群成员管理服务
///
/// 提供群成员的添加、删除、查询、权限管理等功能
pub struct ChatMemberService {
    pub config: Config,
}

impl ChatMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

use crate::core::config::Config;

// TODO: 以下功能待实现
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

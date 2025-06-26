use crate::core::config::Config;

// TODO: 以下功能待实现
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

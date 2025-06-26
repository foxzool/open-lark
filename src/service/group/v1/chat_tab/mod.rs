use crate::core::config::Config;

// TODO: 以下功能待实现
// pub mod create;
// pub mod delete;
// pub mod update;
// pub mod sort;
// pub mod list;

/// 会话标签页服务
///
/// 提供会话标签页的创建、删除、更新、排序、列表查询等功能
pub struct ChatTabService {
    pub config: Config,
}

impl ChatTabService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

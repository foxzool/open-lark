use crate::core::config::Config;

// TODO: 以下功能待实现
// pub mod create;
// pub mod delete;
// pub mod patch;
// pub mod sort;
// pub mod get;

/// 群菜单服务
///
/// 提供群菜单的创建、删除、修改、排序、获取等功能
pub struct ChatMenuTreeService {
    pub config: Config,
}

impl ChatMenuTreeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

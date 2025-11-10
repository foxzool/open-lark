#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::config::Config;
// TODO: 以下功能待实现
// pub mod get;
// pub mod list;
// pub mod create_block;
// pub mod batch_update_block;
// pub mod get_block;
// pub mod get_children_blocks;
// pub mod batch_delete_block;
/// 群公告服务
///
/// 提供群公告和公告块的管理功能
pub struct ChatAnnouncementService {
pub config: Config,
}
impl ChatAnnouncementService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}
}
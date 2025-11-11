#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use config::Config;
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
        Self { config 
}
}
}
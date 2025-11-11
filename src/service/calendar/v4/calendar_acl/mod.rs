#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::config::Config;
pub mod create;
pub mod delete;
pub mod list;
pub mod subscription;
pub mod unsubscription;
// 重新导出所有请求和响应类型
/// 日历访问控制服务
///
/// 提供日历访问控制的管理功能
pub struct CalendarAclService {
pub config: Config,
}
impl CalendarAclService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}
}
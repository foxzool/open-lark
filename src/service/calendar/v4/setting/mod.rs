#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::config::Config;
pub mod generate_caldav_conf;
/// 设置服务
pub struct SettingService {
pub config: Config,
}
impl SettingService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}
}
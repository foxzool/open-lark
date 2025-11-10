#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::config::Config;
pub mod create;
/// 会议纪要服务
pub struct MeetingMinuteService {
pub config: Config,
}
impl MeetingMinuteService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}
}
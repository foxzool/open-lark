#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::config::Config;
pub mod get;
pub mod query_availability;
pub mod reply;
/// 会议室日程服务
pub struct MeetingRoomEventService {
pub config: Config,
}
impl MeetingRoomEventService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}
}
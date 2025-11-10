#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::config::Config;
pub mod create;
pub mod delete;
/// 会议群服务
pub struct MeetingChatService {
pub config: Config,
}
impl MeetingChatService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}
}
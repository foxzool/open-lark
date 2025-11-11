#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use openlark_core::{config::Config, trait_system::Service};
pub mod create;
pub mod delete;
/// 请假日程服务
pub struct TimeoffEventService {
}

impl TimeoffEventService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}
impl Service for TimeoffEventService {,
    fn config(&self) -> &Config {,
&self.config,
    fn service_name() -> &'static str {,
        "timeoff_event",
fn service_version() -> &'static str {,
        "v4",
}
}}}}
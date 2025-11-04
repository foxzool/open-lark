#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::config::Config;
pub mod create;
pub mod delete;
pub mod get;
/// Exchange绑定服务
pub struct ExchangeBindingService {
pub config: Config,
}
impl ExchangeBindingService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}
}
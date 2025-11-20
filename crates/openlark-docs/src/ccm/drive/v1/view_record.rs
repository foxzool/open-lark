
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// drive v1 view_record - 访问记录服务,
//,
// 提供文件访问记录相关的功能,
use openlark_core::{config::Config, trait_system::Service};
/// 访问记录服务
#[derive(Clone, Debug)]
pub struct ViewRecordService {
    #[allow(dead_code)]
    config: Config,,
}
impl ViewRecordService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
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
use crate::prelude::*;
/// 访问记录服务
#[derive(Debug, Clone)]
pub struct ViewRecordService {
    client: std::sync::Arc<LarkClient>,
}
impl ViewRecordService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
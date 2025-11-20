
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// sheets v2 worksheet - 工作表操作API,
//,
// 实现工作表级别的操作,
use openlark_core::prelude::*;
/// 工作表操作服务
#[derive(Clone, Debug)]
pub struct WorksheetService {
    #[allow(dead_code)]
    config: Config,,
}
impl WorksheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
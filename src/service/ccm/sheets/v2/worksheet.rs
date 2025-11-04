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
use crate::prelude::*;
/// 工作表操作服务
#[derive(Debug, Clone)]
pub struct WorksheetService {
    client: std::sync::Arc<LarkClient>,
}
impl WorksheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
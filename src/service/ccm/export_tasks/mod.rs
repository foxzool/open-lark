#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// export_tasks - 导出任务API
//,
// 提供文档导出任务相关的功能
use openlark_core::prelude::*;
/// 导出任务服务
#[derive(Debug, Clone)]
pub struct ExportTasksService {
    client: std::sync::Arc<LarkClient>,
}
impl ExportTasksService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}
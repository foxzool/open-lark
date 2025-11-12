#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// sheets - 电子表格服务
//,
// 提供电子表格相关的所有功能，包括：
// - 表格的创建、读取、更新、删除
// - 单元格操作（读写、样式设置、数据验证）
// - 工作表管理
// - 数据处理（导入、导出、筛选、排序）
// - 图表和图片操作
// - 条件格式和保护范围
//,
// 覆盖60个API接口（sheets/v2: 33个，sheets/v3: 27个）
use openlark_core::prelude::*;
use super::ccm::sheets::v2::SheetsV2Service;
use super::ccm::sheets::v3::SheetsV3Service;
/// 电子表格服务
#[derive(Debug, Clone)]
pub struct SheetsService {
}

impl SheetsService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v2版本API
pub mod v2;
/// v3版本API
pub mod v3;
}
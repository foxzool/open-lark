#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// sheets v2 - 电子表格v2版本API
//,
// 包含电子表格的核心功能，这是广泛使用的版本
use crate::prelude::*;
use crate::service::ccm::sheets::v2::spreadsheet::SpreadsheetService;
use crate::service::ccm::sheets::v2::worksheet::WorksheetService;
use crate::service::ccm::sheets::v2::range::RangeService;
use crate::service::ccm::sheets::v2::style::StyleService;
/// 电子表格v2版本服务
#[derive(Debug, Clone)]
pub struct SheetsV2Service {
}

impl SheetsV2Service {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// 表格操作服务
pub mod spreadsheet;
/// 工作表操作服务
pub mod worksheet;
/// 范围操作服务
pub mod range;
/// 样式操作服务
pub mod style;
}
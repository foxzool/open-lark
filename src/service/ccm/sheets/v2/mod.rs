// sheets v2 - 电子表格v2版本API
//
// 包含电子表格的核心功能，这是广泛使用的版本

use crate::prelude::*;
use crate::service::ccm::sheets::v2::spreadsheet::SpreadsheetService;
use crate::service::ccm::sheets::v2::worksheet::WorksheetService;
use crate::service::ccm::sheets::v2::range::RangeService;
use crate::service::ccm::sheets::v2::style::StyleService;

/// 电子表格v2版本服务
#[derive(Debug, Clone)]
pub struct SheetsV2Service {
    client: std::sync::Arc<LarkClient>,
    /// 表格操作服务
    pub spreadsheet: SpreadsheetService,
    /// 工作表操作服务
    pub worksheet: WorksheetService,
    /// 范围操作服务
    pub range: RangeService,
    /// 样式操作服务
    pub style: StyleService,
}

impl SheetsV2Service {
    /// 创建新的v2版本服务实例
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self {
            spreadsheet: SpreadsheetService::new(client.clone()),
            worksheet: WorksheetService::new(client.clone()),
            range: RangeService::new(client.clone()),
            style: StyleService::new(client.clone()),
        }
    }
}

/// 表格操作服务
pub mod spreadsheet;
/// 工作表操作服务
pub mod worksheet;
/// 范围操作服务
pub mod range;
/// 样式操作服务
pub mod style;
//! Sheets电子表格服务 v3
//!
//! 提供飞书电子表格v3版本的完整管理功能。

#![allow(ambiguous_glob_reexports)]
// ============================================================================
// 统一类型定义
// ============================================================================

/// 范围类型 - 统一定义避免模块间冲突
pub type Range = String;

/// 电子表格令牌类型
pub type SpreadsheetToken = String;

/// 工作表ID类型
pub type SheetId = String;

use openlark_core::{config::Config, trait_system::Service};

pub mod charts;
pub mod comments;
pub mod conditional_format;
pub mod data_filter;
pub mod filter_views;
pub mod find_replace;
pub mod float_images;
pub mod macros;
pub mod move_dimension;
pub mod pivot_tables;
pub mod sheet;
pub mod sheet_protection;
pub mod spreadsheet;
pub mod spreadsheet_create;
pub mod spreadsheet_info;

// 重新导出所有服务类型
pub use charts::*;
pub use comments::*;
pub use conditional_format::*;
pub use data_filter::*;
pub use filter_views::*;
pub use find_replace::*;
pub use float_images::*;
pub use macros::*;
pub use move_dimension::*;
pub use pivot_tables::*;
pub use sheet::*;
pub use sheet_protection::*;
pub use spreadsheet::*;
pub use spreadsheet_create::*;
pub use spreadsheet_info::*;

/// Sheets v3 服务主结构
#[derive(Clone, Debug)]
pub struct SheetsV3Service {
    /// 配置信息
    config: Config,
}

impl SheetsV3Service {
    /// 创建新的 Sheets v3 服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Service for SheetsV3Service {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "SheetsV3Service"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheets_v3_service_creation() {
        let config = Config::default();
        let service = SheetsV3Service::new(config);
        assert_eq!(service.service_name(), "SheetsV3Service");
    }

    #[test]
    fn test_type_aliases() {
        let range: Range = "A1:B10".to_string();
        let token: SpreadsheetToken = "sheet_token_123".to_string();
        let sheet_id: SheetId = "sheet_id_456".to_string();

        assert_eq!(range, "A1:B10");
        assert_eq!(token, "sheet_token_123");
        assert_eq!(sheet_id, "sheet_id_456");
    }
}

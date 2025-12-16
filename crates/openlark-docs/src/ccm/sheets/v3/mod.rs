#![allow(ambiguous_glob_reexports)]

/// Sheets电子表格服务 v3
///
/// 提供飞书电子表格v3版本的完整管理功能。

// ============================================================================
// 统一类型定义
// ============================================================================

/// 范围类型 - 统一定义避免模块间冲突
pub type Range = String;

/// 电子表格令牌类型
pub type SpreadsheetToken = String;

/// 工作表ID类型
pub type SheetId = String;

use openlark_core::config::Config;

pub mod charts;
pub mod comments;
pub mod conditional_format;
pub mod data_filter;
pub mod filter_views;
// pub mod find_replace; // 暂时注释
pub mod float_images;
pub mod macros;
pub mod models;
// pub mod move_dimension; // 暂时注释
pub mod pivot_tables;
pub mod sheet;
pub mod sheet_protection;
pub mod spreadsheet;
pub mod spreadsheet_create;
pub mod spreadsheet_info;

// 集成测试
#[cfg(test)]
mod integration_test;

// 重新导出所有服务类型
pub use charts::*;
pub use comments::*;
pub use conditional_format::*;
pub use data_filter::*;
pub use filter_views::*;
// pub use find_replace::*; // 暂时注释
pub use float_images::*;
pub use macros::*;
pub use models::*;
// pub use move_dimension::*; // 暂时注释
pub use pivot_tables::*;
pub use sheet::*;
pub use sheet_protection::*;
// pub use crate::ccm::sheets::*;
// pub use crate::ccm::sheets_create::*;
// pub use crate::ccm::sheets_info::*;

/// Sheets 服务主结构
#[derive(Clone, Debug)]
pub struct SheetsService {
    /// 配置信息
    config: Config,
}

impl SheetsService {
    /// 创建新的 Sheets 服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Default for SheetsService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheets_service_creation() {
        let config = Config::default();
        let service = SheetsService::new(config);
        // 验证服务实例创建成功
        assert!(!service.config.base_url.is_empty());
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

    #[test]
    fn test_sheets_service_default() {
        let service = SheetsService::default();
        assert!(!service.config.base_url.is_empty());
    }
}

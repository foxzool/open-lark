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

// pub mod charts; // Generated: Module file not found
// pub mod comments; // Generated: Module file not found
// pub mod conditional_format; // Generated: Module file not found
// pub mod data_filter; // Generated: Module file not found
// pub mod filter_views; // Generated: Module file not found
// pub mod find_replace; // 暂时注释
// pub mod float_images; // Generated: Module file not found
// pub mod macros; // Generated: Module file not found
pub mod models;
// pub mod move_dimension; // 暂时注释
// pub mod pivot_tables; // Generated: Module file not found
// pub mod sheet; // Generated: Module file not found
// pub mod sheet_protection; // Generated: Module file not found
pub mod spreadsheet;
// pub mod spreadsheet_create; // Generated: Module file not found
// pub mod spreadsheet_info; // Generated: Module file not found

// 集成测试
#[cfg(test)]
// mod integration_test; // Generated: Module file not found

// 重新导出所有服务类型
// pub use charts::*; // Generated: Module use not found
// pub use comments::*; // Generated: Module use not found
// pub use conditional_format::*; // Generated: Module use not found
// pub use data_filter::*; // Generated: Module use not found
// pub use filter_views::*; // Generated: Module use not found
// pub use find_replace::*; // 暂时注释
// pub use float_images::*; // Generated: Module use not found
// pub use macros::*; // Generated: Module use not found
pub use models::*;
// pub use move_dimension::*; // 暂时注释
// pub use pivot_tables::*; // Generated: Module use not found
// pub use sheet::*; // Generated: Module use not found
// pub use sheet_protection::*; // Generated: Module use not found
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

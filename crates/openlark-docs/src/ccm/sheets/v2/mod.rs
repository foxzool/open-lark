//! Sheets电子表格服务 v2
//!
//! 提供飞书电子表格v2版本的完整管理功能。

pub mod batch_range_read;
pub mod batch_read;
pub mod batch_read_ranges;
pub mod batch_write;
pub mod condition_formats;
pub mod data_validation;
pub mod dimension_operations;
pub mod image_write;
pub mod image_write_enhanced;
pub mod import;
pub mod import_result;
pub mod merge_cells;
pub mod metainfo;
pub mod properties;
pub mod protected_ranges;
pub mod sheet_cells;
pub mod sheet_management;
pub mod sheets_batch_update;
pub mod single_range_read;
pub mod single_read;
pub mod single_write;
pub mod style_operations;
pub mod values_append;
pub mod values_batch_write;
pub mod values_prepend;
pub mod values_single_write;

// 暂时注释掉模糊的重导出以避免警告
// 模块中的服务可以直接通过完整路径访问

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport, SDKResult,
};

/// Sheets电子表格服务 v2版本
///
/// 提供飞书电子表格v2版本的统一入口。
#[derive(Clone, Debug)]
pub struct SheetsV2Service {
    config: Config,
}

impl SheetsV2Service {
    /// 创建Sheets v2服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Default for SheetsV2Service {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

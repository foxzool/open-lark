/// Sheets电子表格服务 v2 (Legacy)
///
/// ⚠️ **已弃用** - 这是飞书电子表格v2版本的历史实现。
///
/// **推荐使用**: 请使用v3版本的API，位于 `../v3/` 目录。
///
/// v2版本的API将在未来版本中移除，建议迁移到v3版本以获得更好的性能和功能支持。
///
/// # 迁移指南
///
/// - v2 `dimension_operations` → v3 `data_operation/*`
/// - v2 `merge_cells` → v3 `data_operation/merge_cells`
/// - v2 `image_write` → v3 `float_image/*`
/// - v2 `protected_ranges` → v3 `filter_view/condition/*`
///
/// # 注意事项
///
/// 1. 这些API不被计入27个sheets v3新API
/// 2. 仅用于历史兼容性维护
/// 3. 新项目应直接使用v3 API
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

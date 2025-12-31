//! Sheets电子表格服务 v2（Legacy）
//!
//! ⚠️ **已弃用** - 这是飞书电子表格 v2 版本的历史实现。
//!
//! **推荐使用**: 请使用 v3 版本的 API，位于 `../v3/` 目录。
//!
//! v2 版本的 API 将在未来版本中移除，建议迁移到 v3 版本以获得更好的性能和功能支持。
//!
//! # 迁移指南
//!
//! - v2 `dimension_operations` → v3 `data_operation/*`
//! - v2 `merge_cells` → v3 `data_operation/merge_cells`
//! - v2 `image_write` → v3 `float_image/*`
//! - v2 `protected_ranges` → v3 `filter_view/condition/*`
//!
//! # 注意事项
//!
//! 1. 这些 API 不被计入 27 个 sheets v3 新 API
//! 2. 仅用于历史兼容性维护
//! 3. 新项目应直接使用 v3 API
// pub mod batch_range_read; // Generated: Module file not found
// pub mod batch_read; // Generated: Module file not found
// pub mod batch_read_ranges; // Generated: Module file not found
// pub mod batch_write; // Generated: Module file not found
// pub mod condition_formats; // Generated: Module file not found
// pub mod data_validation; // Generated: Module file not found
// pub mod dimension_operations; // Generated: Module file not found
// pub mod image_write; // Generated: Module file not found
// pub mod image_write_enhanced; // Generated: Module file not found
// pub mod import; // Generated: Module file not found
// pub mod import_result; // Generated: Module file not found
// pub mod merge_cells; // Generated: Module file not found
// pub mod metainfo; // Generated: Module file not found
// pub mod properties; // Generated: Module file not found
// pub mod protected_ranges; // Generated: Module file not found
// pub mod sheet_cells; // Generated: Module file not found
// pub mod sheet_management; // Generated: Module file not found
// pub mod sheets_batch_update; // Generated: Module file not found
// pub mod single_range_read; // Generated: Module file not found
// pub mod single_read; // Generated: Module file not found
// pub mod single_write; // Generated: Module file not found
// pub mod style_operations; // Generated: Module file not found
// pub mod values_append; // Generated: Module file not found
// pub mod values_batch_write; // Generated: Module file not found
// pub mod values_prepend; // Generated: Module file not found
// pub mod values_single_write; // Generated: Module file not found

// 暂时注释掉模糊的重导出以避免警告
// 模块中的服务可以直接通过完整路径访问
use openlark_core::config::Config;

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

    /// 获取配置引用（避免 `config` 字段被认为未使用）
    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Default for SheetsV2Service {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

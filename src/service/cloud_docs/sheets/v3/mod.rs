//! Sheets API v3 版本服务
//!
//! 提供飞书电子表格的完整v3版本API功能，包括数据操作、样式设置、
//! 工作表管理、图表操作等高级功能。这是推荐的API版本。

use crate::core::config::Config;

pub mod condition_format;
pub mod data_operation;
pub mod data_validation;
pub mod float_image;
pub mod protect_range;
pub mod sheet_row_col;
pub mod spreadsheet;
pub mod spreadsheet_sheet;
pub mod spreadsheet_sheet_filter;
pub mod spreadsheet_sheet_filter_view;
pub mod spreadsheet_sheet_filter_view_condition;

/// Sheets API v3 版本服务
///
/// 聚合所有v3版本的Sheets相关服务，提供统一的电子表格操作接口。
/// 包含数据操作、样式设置、工作表管理等完整功能集。
#[derive(Debug, Clone)]
pub struct V3 {
    config: Config,
    /// 电子表格基础操作服务
    pub spreadsheet: spreadsheet::SpreadsheetService,
    /// 数据操作服务（读写、样式、合并等）
    pub data_operation: data_operation::DataOperationService,
    /// 工作表操作服务
    pub spreadsheet_sheet: spreadsheet_sheet::SpreadsheetSheetService,
    /// 行列操作服务
    pub sheet_row_col: sheet_row_col::SheetRowColService,
    /// 条件格式化服务
    pub condition_format: condition_format::ConditionFormatService,
    /// 数据验证服务
    pub data_validation: data_validation::DataValidationService,
    /// 单元格保护服务
    pub protect_range: protect_range::ProtectRangeService,
    /// 浮动图片服务
    pub float_image: float_image::FloatImageService,
    /// 筛选器服务
    pub spreadsheet_sheet_filter: spreadsheet_sheet_filter::SpreadsheetSheetFilterService,
    /// 筛选视图服务
    pub spreadsheet_sheet_filter_view: spreadsheet_sheet_filter_view::SpreadsheetSheetFilterViewService,
    /// 筛选视图条件服务
    pub spreadsheet_sheet_filter_view_condition: spreadsheet_sheet_filter_view_condition::SpreadsheetSheetFilterViewConditionService,
}

impl V3 {
    /// 创建新的v3服务实例
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            spreadsheet: spreadsheet::SpreadsheetService::new(config.clone()),
            data_operation: data_operation::DataOperationService::new(config.clone()),
            spreadsheet_sheet: spreadsheet_sheet::SpreadsheetSheetService::new(config.clone()),
            sheet_row_col: sheet_row_col::SheetRowColService::new(config.clone()),
            condition_format: condition_format::ConditionFormatService::new(config.clone()),
            data_validation: data_validation::DataValidationService::new(config.clone()),
            protect_range: protect_range::ProtectRangeService::new(config.clone()),
            float_image: float_image::FloatImageService::new(config.clone()),
            spreadsheet_sheet_filter: spreadsheet_sheet_filter::SpreadsheetSheetFilterService::new(config.clone()),
            spreadsheet_sheet_filter_view: spreadsheet_sheet_filter_view::SpreadsheetSheetFilterViewService::new(config.clone()),
            spreadsheet_sheet_filter_view_condition: spreadsheet_sheet_filter_view_condition::SpreadsheetSheetFilterViewConditionService::new(config),
        }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 为了向后兼容，提供类型别名
pub type SpreadsheetService = spreadsheet::SpreadsheetService;
pub type DataOperationService = data_operation::DataOperationService;
pub type SpreadsheetSheetService = spreadsheet_sheet::SpreadsheetSheetService;
pub type SheetRowColService = sheet_row_col::SheetRowColService;
pub type ConditionFormatService = condition_format::ConditionFormatService;
pub type DataValidationService = data_validation::DataValidationService;
pub type ProtectRangeService = protect_range::ProtectRangeService;
pub type FloatImageService = float_image::FloatImageService;
pub type SpreadsheetSheetFilterService = spreadsheet_sheet_filter::SpreadsheetSheetFilterService;
pub type SpreadsheetSheetFilterViewService = spreadsheet_sheet_filter_view::SpreadsheetSheetFilterViewService;
pub type SpreadsheetSheetFilterViewConditionService = spreadsheet_sheet_filter_view_condition::SpreadsheetSheetFilterViewConditionService;
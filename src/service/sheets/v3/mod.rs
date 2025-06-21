use crate::core::config::Config;

pub mod data_operation;
pub mod protect_range;
pub mod sheet_row_col;
pub mod spreadsheet;
pub mod spreadsheet_sheet;
pub mod spreadsheet_sheet_filter;
pub mod spreadsheet_sheet_filter_view;
pub mod spreadsheet_sheet_filter_view_condition;

pub struct V3 {
    pub spreadsheet: SpreadsheetService,
    pub spreadsheet_sheet: SpreadsheetSheetService,
    pub spreadsheet_sheet_filter: SpreadsheetSheetFilterService,
    pub spreadsheet_sheet_filter_view: SpreadsheetSheetFilterViewService,
    pub data_operation: DataOperationService,
    pub sheet_row_col: SheetRowColService,
}

impl V3 {
    pub fn new(config: Config) -> Self {
        Self {
            spreadsheet: SpreadsheetService::new(config.clone()),
            spreadsheet_sheet: SpreadsheetSheetService::new(config.clone()),
            spreadsheet_sheet_filter: SpreadsheetSheetFilterService::new(config.clone()),
            spreadsheet_sheet_filter_view: SpreadsheetSheetFilterViewService::new(config.clone()),
            data_operation: DataOperationService::new(config.clone()),
            sheet_row_col: SheetRowColService::new(config),
        }
    }
}

/// 电子表格
pub struct SpreadsheetService {
    config: Config,
}

impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 工作表
pub struct SpreadsheetSheetService {
    config: Config,
}

impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 工作表筛选
pub struct SpreadsheetSheetFilterService {
    config: Config,
}

impl SpreadsheetSheetFilterService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 工作表筛选视图
pub struct SpreadsheetSheetFilterViewService {
    config: Config,
}

impl SpreadsheetSheetFilterViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 数据操作
pub struct DataOperationService {
    config: Config,
}

impl DataOperationService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 行列操作
pub struct SheetRowColService {
    #[allow(dead_code)]
    config: Config,
}

impl SheetRowColService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

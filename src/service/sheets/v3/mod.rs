use crate::core::config::Config;

pub mod data_operation;
pub mod sheet_row_col;
pub mod spreadsheet;
pub mod spreadsheet_sheet;
pub mod spreadsheet_sheet_filter;

pub struct V3 {
    pub spreadsheet: SpreadsheetService,
    pub spreadsheet_sheet: SpreadsheetSheetService,
    pub spreadsheet_sheet_filter: SpreadsheetSheetFilterService,
}

impl V3 {
    pub fn new(config: Config) -> Self {
        Self {
            spreadsheet: SpreadsheetService::new(config.clone()),
            spreadsheet_sheet: SpreadsheetSheetService::new(config.clone()),
            spreadsheet_sheet_filter: SpreadsheetSheetFilterService::new(config),
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

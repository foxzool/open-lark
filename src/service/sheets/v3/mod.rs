use crate::core::config::Config;

pub mod spreadsheet;
pub mod spreadsheet_sheet;

pub struct V3 {
    pub spreadsheet: SpreadsheetService,
    pub spreadsheet_sheet: SpreadsheetSheetService,
}

impl V3 {
    pub fn new(config: Config) -> Self {
        Self {
            spreadsheet: SpreadsheetService::new(config.clone()),
            spreadsheet_sheet: SpreadsheetSheetService::new(config),
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

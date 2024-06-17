use crate::core::config::Config;

pub use query_spreadsheet_sheet::*;
mod query_spreadsheet_sheet;

/// 工作表
pub struct SpreadsheetSheetService {
    config: Config,
}

impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
use crate::core::config::Config;

pub use operate_sheets::*;
pub use update_sheet_properties::*;
mod operate_sheets;
mod update_sheet_properties;


/// 工作表
pub struct SpreadsheetSheetService {
    config: Config,
}

impl SpreadsheetSheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

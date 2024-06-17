pub mod spreadsheet;
pub mod spreadsheet_sheet;

pub struct V3 {
    pub spreadsheet: spreadsheet::SpreadsheetService,
    pub spreadsheet_sheet: spreadsheet_sheet::SpreadsheetSheetService,
}

impl V3 {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self {
            spreadsheet: spreadsheet::SpreadsheetService::new(config.clone()),
            spreadsheet_sheet: spreadsheet_sheet::SpreadsheetSheetService::new(config),
        }
    }
}

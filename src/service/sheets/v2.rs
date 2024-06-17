
pub mod spreadsheet_sheet;

pub struct V2 {

    pub spreadsheet_sheet: spreadsheet_sheet::SpreadsheetSheetService,
}

impl V2 {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self {
            spreadsheet_sheet: spreadsheet_sheet::SpreadsheetSheetService::new(config),
        }
    }
}

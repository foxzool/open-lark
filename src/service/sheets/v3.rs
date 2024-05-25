pub mod spreadsheet;

pub struct V3 {
    pub spreadsheet: spreadsheet::SpreadsheetService,
}

impl V3 {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self {
            spreadsheet: spreadsheet::SpreadsheetService::new(config),
        }
    }
}



pub use create_spreadsheet::*;
pub use get_spreadsheet::*;
pub use patch_spreadsheet::*;

use crate::core::{api_resp::ApiResponseTrait, config::Config};

/// 电子表格
pub struct SpreadsheetService {
    config: Config,
}

mod create_spreadsheet;
mod get_spreadsheet;
mod patch_spreadsheet;

impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

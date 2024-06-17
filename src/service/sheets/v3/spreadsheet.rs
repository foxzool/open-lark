

pub use create::*;
pub use get::*;
pub use patch::*;

use crate::core::{config::Config};

/// 电子表格
pub struct SpreadsheetService {
    config: Config,
}

mod create;
mod get;
mod patch;

impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

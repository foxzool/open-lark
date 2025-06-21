use crate::core::config::Config;

pub mod create;
pub mod patch;
pub mod list;
pub mod get;
pub mod delete;

pub use create::*;
pub use patch::*;
pub use list::*;
pub use get::*;
pub use delete::*;

/// 视图服务
pub struct AppTableViewService {
    config: Config,
}

impl AppTableViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
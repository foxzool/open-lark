use crate::core::config::Config;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod patch;

pub use create::*;
pub use delete::*;
pub use get::*;
pub use list::*;
pub use patch::*;

/// 视图服务
pub struct AppTableViewService {
    config: Config,
}

impl AppTableViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

}

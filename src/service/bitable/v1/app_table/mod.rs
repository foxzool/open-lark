use crate::core::config::Config;

pub mod create;
pub mod batch_create;
pub mod patch;
pub mod list;
pub mod delete;
pub mod batch_delete;

pub use create::*;
pub use batch_create::*;
pub use patch::*;
pub use list::*;
pub use delete::*;
pub use batch_delete::*;

/// 数据表服务
pub struct AppTableService {
    config: Config,
}

impl AppTableService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
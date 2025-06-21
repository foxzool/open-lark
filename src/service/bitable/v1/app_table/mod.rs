use crate::core::config::Config;

pub mod batch_create;
pub mod batch_delete;
pub mod create;
pub mod delete;
pub mod list;
pub mod patch;

pub use batch_create::*;
pub use batch_delete::*;
pub use create::*;
pub use delete::*;
pub use list::*;
pub use patch::*;

/// 数据表服务
pub struct AppTableService {
    config: Config,
}

impl AppTableService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

use crate::core::config::Config;

pub mod copy;
pub mod create;
pub mod get;
pub mod update;

pub use copy::*;
pub use create::*;
pub use get::*;
pub use update::*;

/// 多维表格服务
pub struct AppService {
    config: Config,
}

impl AppService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

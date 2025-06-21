use crate::core::config::Config;

pub mod create;
pub mod copy;
pub mod get;
pub mod update;

pub use create::*;
pub use copy::*;
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
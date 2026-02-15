pub mod app;

pub use app::*;

use openlark_core::config::Config;
use std::sync::Arc;

/// ApplicationV1：应用 API v1 访问入口
#[derive(Clone)]
pub struct ApplicationV1 {
    config: Arc<Config>,
}

impl ApplicationV1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问应用资源
    pub fn app(&self) -> app::App {
        app::App::new(self.config.clone())
    }
}

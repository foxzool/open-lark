pub use app::*;

use crate::core::config::Config;

mod app;

pub struct V1 {
    pub app: App,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            app: App::new(config),
        }
    }
}



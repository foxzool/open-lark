//! Text translation module

pub mod detect;
pub mod translate;

use openlark_core::config::Config;
use std::sync::Arc;

/// Text translation API
#[derive(Clone)]
pub struct Text {
    #[allow(dead_code)]
    config: Arc<Config>,
}

impl Text {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

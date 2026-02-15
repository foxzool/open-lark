//! Text translation module

pub mod translate;
pub mod detect;

use openlark_core::config::Config;
use std::sync::Arc;

/// Text translation API
#[derive(Clone)]
pub struct Text {
    config: Arc<Config>,
}

impl Text {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

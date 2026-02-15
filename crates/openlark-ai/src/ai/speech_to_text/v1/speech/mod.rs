//! Speech recognition module

pub mod file_recognize;
pub mod stream_recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// Speech recognition API
#[derive(Clone)]
pub struct Speech {
    config: Arc<Config>,
}

impl Speech {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

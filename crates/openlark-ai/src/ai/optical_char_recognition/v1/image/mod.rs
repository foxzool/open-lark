//! Image OCR module

pub mod basic_recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// Image OCR API
#[derive(Clone)]
pub struct Image {
    config: Arc<Config>,
}

impl Image {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

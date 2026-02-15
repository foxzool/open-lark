//! Optical Character Recognition (OCR) 模块

pub mod v1;

use openlark_core::config::Config;
use std::sync::Arc;

/// OCR API 入口
#[derive(Clone)]
pub struct OpticalCharRecognition {
    config: Arc<Config>,
}

impl OpticalCharRecognition {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn v1(&self) -> v1::OcrV1 {
        v1::OcrV1::new(self.config.clone())
    }
}

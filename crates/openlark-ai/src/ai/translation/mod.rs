//! Translation 模块
//!
//! 提供翻译服务，包括文本翻译和语言检测。

pub mod v1;

use openlark_core::config::Config;
use std::sync::Arc;

/// Translation API 入口
#[derive(Clone)]
pub struct Translation {
    config: Arc<Config>,
}

impl Translation {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn v1(&self) -> v1::TranslationV1 {
        v1::TranslationV1::new(self.config.clone())
    }
}

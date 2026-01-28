//! AI API v1 模块
//!
//! 提供 AI 服务的 v1 版本 API。

use crate::prelude::Config;
use std::sync::Arc;

/// AI API v1 服务（顶层聚合）
#[derive(Debug, Clone)]
pub struct V1 {
    config: Arc<Config>,
    /// 文档 AI V1 服务
    pub document_ai: DocumentAiV1,
    /// OCR V1 服务
    pub ocr: OcrV1,
    /// 语音转文字 V1 服务
    pub speech_to_text: SpeechToTextV1,
    /// 翻译 V1 服务
    pub translation: TranslationV1,
}

impl V1 {
    /// 创建新的 AI v1 服务
    pub fn new(config: Config) -> Self {
        let config_arc = Arc::new(config);
        Self {
            config: config_arc.clone(),
            document_ai: DocumentAiV1::new(config_arc.clone()),
            ocr: OcrV1::new(config_arc.clone()),
            speech_to_text: SpeechToTextV1::new(config_arc.clone()),
            translation: TranslationV1::new(config_arc),
        }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 文档 AI V1 服务
#[derive(Debug, Clone)]
pub struct DocumentAiV1 {
    config: Arc<Config>,
}

impl DocumentAiV1 {
    /// 创建新的文档 AI v1 服务
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// OCR V1 服务
#[derive(Debug, Clone)]
pub struct OcrV1 {
    config: Arc<Config>,
}

impl OcrV1 {
    /// 创建新的 OCR v1 服务
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 语音转文字 V1 服务
#[derive(Debug, Clone)]
pub struct SpeechToTextV1 {
    config: Arc<Config>,
}

impl SpeechToTextV1 {
    /// 创建新的语音转文字 v1 服务
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 翻译 V1 服务
#[derive(Debug, Clone)]
pub struct TranslationV1 {
    config: Arc<Config>,
}

impl TranslationV1 {
    /// 创建新的翻译 v1 服务
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}

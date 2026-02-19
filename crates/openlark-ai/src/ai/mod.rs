#![allow(clippy::module_inception)]
//! AI service module
//!
//! 提供 AI 服务模块，包括文档 AI、OCR、语音转文字和翻译服务。

pub mod document_ai;
pub mod optical_char_recognition;
pub mod speech_to_text;
pub mod translation;
pub mod v1;

use crate::prelude::Config;
use openlark_core::trait_system::Service;

/// AI service
#[derive(Debug, Clone)]
pub struct AiService {
    config: Config,
    /// AI API v1 service
    pub v1: v1::V1,
}

impl AiService {
    /// Create new AI service instance
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::V1::new(config),
        }
    }
}

impl Service for AiService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "ai"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = AiService::new(config);
        assert_eq!(service.config().app_id(), "test_app_id");
    }

    #[test]
    fn test_ai_service_service_name() {
        assert_eq!(AiService::service_name(), "ai");
    }

    #[test]
    fn test_ai_service_service_version() {
        assert_eq!(AiService::service_version(), "v1");
    }
}

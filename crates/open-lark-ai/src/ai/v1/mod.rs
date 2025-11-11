// AI v1 服务模块

pub mod document_ai;
pub mod optical_char_recognition;
pub mod speech_to_text;
pub mod translation;

/// AI 服务
///
/// 提供完整的人工智能服务能力，包括：
/// - 🤖 **文档处理**: AI 文档解析、理解和总结
/// - 👁 **光学字符识别**: 图片文字识别和OCR功能
/// - 🗣️ **语音转文字**: 语音识别和转写服务
/// - 🌐 **翻译服务**: 多语言翻译和本地化支持
/// - 🧠 **智能对话**: 基于大语言模型的对话和问答系统
///
/// 为企业提供先进的AI能力集成，支持智能化业务流程和自动化决策。

#[derive(Debug)]
pub struct AiService {
    config: openlark_core::config::Config,
}

impl AiService {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }
}

impl openlark_core::trait_system::Service for AiService {
    fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "ai"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

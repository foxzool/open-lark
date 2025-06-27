use crate::core::config::Config;

pub mod document_ai;
pub mod models;
pub mod optical_char_recognition;
pub mod speech_to_text;
pub mod translation;

use document_ai::DocumentAiService;
use optical_char_recognition::OpticalCharRecognitionService;
use speech_to_text::SpeechToTextService;
use translation::TranslationService;

/// AI 能力服务
///
/// 提供飞书平台的AI能力接口，包括：
/// - 智能文档处理：简历解析、证件识别、发票识别、合同识别、名片识别等
/// - 光学字符识别：图片文字识别
/// - 语音识别：语音文件识别、流式语音识别
/// - 机器翻译：语种检测、文本翻译
pub struct AiService {
    /// 智能文档处理服务
    pub document_ai: DocumentAiService,
    /// 光学字符识别服务
    pub optical_char_recognition: OpticalCharRecognitionService,
    /// 语音识别服务
    pub speech_to_text: SpeechToTextService,
    /// 机器翻译服务
    pub translation: TranslationService,
}

impl AiService {
    pub fn new(config: Config) -> Self {
        Self {
            document_ai: DocumentAiService::new(config.clone()),
            optical_char_recognition: OpticalCharRecognitionService::new(config.clone()),
            speech_to_text: SpeechToTextService::new(config.clone()),
            translation: TranslationService::new(config),
        }
    }
}

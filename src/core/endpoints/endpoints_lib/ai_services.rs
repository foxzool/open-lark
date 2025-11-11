//! AI服务端点
//!
//! 包含文档识别、OCR、语音转文字、翻译等AI相关的API端点。

/// AI服务相关端点
pub struct AiServices;

impl AiServices {
    // ==================== 文档AI识别 ====================

    /// 简历解析
    pub const DOCUMENT_AI_RESUME_PARSE: &'static str = "/open-apis/document_ai/v1/resume_parse";

    /// 身份证识别
    pub const DOCUMENT_AI_ID_CARD_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/id_card_recognize";

    /// 银行卡识别
    pub const DOCUMENT_AI_BANK_CARD_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/bank_card_recognize";

    // ==================== OCR光学字符识别 ====================

    /// 基础OCR识别
    pub const OCR_BASIC_RECOGNIZE: &'static str = "/open-apis/optical_char_recognition/v1/image/basic_recognize";

    // ==================== 语音转文字 ====================

    /// 文件语音识别
    pub const SPEECH_FILE_RECOGNIZE: &'static str = "/open-apis/speech_to_text/v1/speech/file_recognize";

    /// 流式语音识别
    pub const SPEECH_STREAM_RECOGNIZE: &'static str = "/open-apis/speech_to_text/v1/speech/stream_recognize";

    // ==================== 翻译服务 ====================

    /// 文本检测
    pub const TRANSLATION_TEXT_DETECT: &'static str = "/open-apis/translation/v1/text/detect";

    /// 文本翻译
    pub const TRANSLATION_TEXT_TRANSLATE: &'static str = "/open-apis/translation/v1/text/translate";
}
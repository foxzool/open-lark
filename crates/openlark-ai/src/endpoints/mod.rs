//! OpenLark AI 服务端点定义
//!
//! 此模块包含AI相关的所有API端点，从 openlark-core 迁移而来。
//! 包含AI助手、文档识别、光学字符识别、语音转文字、翻译服务等完整功能。
//!
//! # 服务模块包含
//!
//! - **ai**: AI助手和智能分析（已在ai/目录中实现）
//! - **document_ai**: 文档AI识别服务
//! - **ocr**: 光学字符识别服务
//! - **speech_to_text**: 语音转文字服务
//! - **translation**: 翻译服务
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_ai::endpoints::*;
//!
//! // 文档AI识别
//! let resume_endpoint = DOCUMENT_AI_RESUME_PARSE;
//! let id_card_endpoint = DOCUMENT_AI_ID_CARD_RECOGNIZE;
//!
//! // OCR识别
//! let ocr_basic_endpoint = OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE;
//!
//! // 语音转文字
//! let speech_file_endpoint = SPEECH_TO_TEXT_V1_FILE_RECOGNIZE;
//! let speech_stream_endpoint = SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE;
//!
//! // 翻译服务
//! let translate_endpoint = TRANSLATION_V1_TEXT_TRANSLATE;
//! let detect_endpoint = TRANSLATION_V1_TEXT_DETECT;
//! ```

// 导入核心端点（auth, application等基础端点）
pub use openlark_core::endpoints::{apass, application, auth, platform_integration};

// ==================== Document AI (文档AI识别) ====================
// 文档AI识别服务 - 支持各种证件和文档的智能识别

/// Document AI - 简历解析
/// 智能解析简历文档，提取关键信息
pub const DOCUMENT_AI_RESUME_PARSE: &str = "/open-apis/document_ai/v1/resume_parse";

/// Document AI - 身份证识别
/// 识别身份证中的关键信息
pub const DOCUMENT_AI_ID_CARD_RECOGNIZE: &str = "/open-apis/document_ai/v1/id_card_recognize";

/// Document AI - 驾驶证识别
/// 识别驾驶证中的关键信息
pub const DOCUMENT_AI_DRIVING_LICENSE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/driving_license_recognize";

/// Document AI - 银行卡识别
/// 识别银行卡中的关键信息
pub const DOCUMENT_AI_BANK_CARD_RECOGNIZE: &str = "/open-apis/document_ai/v1/bank_card_recognize";

/// Document AI - 名片识别
/// 智能识别名片中的联系信息
pub const DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/business_card_recognize";

/// Document AI - 营业执照识别
/// 识别营业执照中的企业信息
pub const DOCUMENT_AI_BUSINESS_LICENSE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/business_license_recognize";

/// Document AI - 中国护照识别
/// 识别中国护照中的关键信息
pub const DOCUMENT_AI_CHINESE_PASSPORT_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/chinese_passport_recognize";

/// Document AI - 合同字段提取
/// 智能提取合同中的关键字段
pub const DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION: &str =
    "/open-apis/document_ai/v1/contract_field_extraction";

/// Document AI - 食品经营许可证识别
/// 识别食品经营许可证信息
pub const DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/food_manage_license_recognize";

/// Document AI - 食品生产许可证识别
/// 识别食品生产许可证信息
pub const DOCUMENT_AI_FOOD_PRODUCE_LICENSE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/food_produce_license_recognize";

/// Document AI - 健康证识别
/// 识别健康证信息
pub const DOCUMENT_AI_HEALTH_CERTIFICATE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/health_certificate_recognize";

/// Document AI - 港澳通行证识别
/// 识别港澳通行证信息
pub const DOCUMENT_AI_HKM_MAINLAND_TRAVEL_PERMIT_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/hkm_mainland_travel_permit_recognize";

/// Document AI - 出租车发票识别
/// 识别出租车发票信息
pub const DOCUMENT_AI_TAXI_INVOICE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/taxi_invoice_recognize";

/// Document AI - 火车票识别
/// 识别火车票信息
pub const DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/train_invoice_recognize";

/// Document AI - 台湾通行证识别
/// 识别台湾通行证信息
pub const DOCUMENT_AI_TW_MAINLAND_TRAVEL_PERMIT_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/tw_mainland_travel_permit_recognize";

/// Document AI - 增值税发票识别
/// 识别增值税发票信息
pub const DOCUMENT_AI_VAT_INVOICE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/vat_invoice_recognize";

/// Document AI - 机动车发票识别
/// 识别机动车发票信息
pub const DOCUMENT_AI_VEHICLE_INVOICE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/vehicle_invoice_recognize";

/// Document AI - 行驶证识别
/// 识别行驶证信息
pub const DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/vehicle_license_recognize";

// ==================== OCR (光学字符识别) ====================
// 光学字符识别服务 - 将图片中的文字转换为可编辑文本

/// OCR光学字符识别 - 基础识别
/// 基础的OCR文字识别服务
pub const OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE: &str =
    "/open-apis/optical_char_recognition/v1/basic_recognize";

/// OCR光学字符识别 - 图片基础识别
/// 专门针对图片的基础OCR识别
pub const OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE: &str =
    "/open-apis/optical_char_recognition/v1/image/basic_recognize";

// ==================== Speech to Text (语音转文字) ====================
// 语音转文字服务 - 将语音转换为文本

/// 语音转文字 - 文件识别
/// 识别音频文件中的语音内容
pub const SPEECH_TO_TEXT_V1_FILE_RECOGNIZE: &str = "/open-apis/speech_to_text/v1/file/recognize";

/// 语音转文字 - 流式识别
/// 实时流式语音识别
pub const SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE: &str =
    "/open-apis/speech_to_text/v1/stream/recognize";

/// 语音转文字 - 语音识别
/// 通用语音识别服务
pub const SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE: &str =
    "/open-apis/speech_to_text/v1/speech/recognize";

// ==================== Translation (翻译服务) ====================
// 翻译服务 - 文本翻译和语言检测

/// 翻译服务 - 文本检测
/// 检测文本的语言类型
pub const TRANSLATION_V1_TEXT_DETECT: &str = "/open-apis/translation/v1/text/detect";

/// 翻译服务 - 文本翻译
/// 文本翻译服务
pub const TRANSLATION_V1_TEXT_TRANSLATE: &str = "/open-apis/translation/v1/text/translate";

// ==================== 兼容性别名 ====================

/// 为保持向后兼容性，提供一些简短的别名
/// DocumentAI别名
/// 简历解析别名
pub const RESUME_PARSE: &str = DOCUMENT_AI_RESUME_PARSE;
/// 身份证识别别名
pub const ID_CARD_RECOGNIZE: &str = DOCUMENT_AI_ID_CARD_RECOGNIZE;
/// 银行卡识别别名
pub const BANK_CARD_RECOGNIZE: &str = DOCUMENT_AI_BANK_CARD_RECOGNIZE;

/// OCR别名
pub const OCR_BASIC_RECOGNIZE: &str = OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE;

/// Speech别名
pub const SPEECH_RECOGNIZE: &str = SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE;

/// Translation别名
/// 文本翻译别名
pub const TEXT_TRANSLATE: &str = TRANSLATION_V1_TEXT_TRANSLATE;
/// 文本检测别名
pub const TEXT_DETECT: &str = TRANSLATION_V1_TEXT_DETECT;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_ai_endpoints() {
        // 验证Document AI端点
        assert!(DOCUMENT_AI_RESUME_PARSE.starts_with("/open-apis/document_ai/"));
        assert!(DOCUMENT_AI_RESUME_PARSE.contains("resume_parse"));
        assert!(DOCUMENT_AI_ID_CARD_RECOGNIZE.contains("id_card_recognize"));
        assert!(DOCUMENT_AI_BANK_CARD_RECOGNIZE.contains("bank_card_recognize"));
        assert!(DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE.contains("business_card_recognize"));
    }

    #[test]
    fn test_ocr_endpoints() {
        // 验证OCR端点
        assert!(OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE
            .starts_with("/open-apis/optical_char_recognition/"));
        assert!(OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE.contains("recognize"));
        assert!(OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE.contains("image"));
    }

    #[test]
    fn test_speech_to_text_endpoints() {
        // 验证语音转文字端点
        assert!(SPEECH_TO_TEXT_V1_FILE_RECOGNIZE.starts_with("/open-apis/speech_to_text/"));
        assert!(SPEECH_TO_TEXT_V1_FILE_RECOGNIZE.contains("file"));
        assert!(SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE.contains("stream"));
        assert!(SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE.contains("speech"));
    }

    #[test]
    fn test_translation_endpoints() {
        // 验证翻译服务端点
        assert!(TRANSLATION_V1_TEXT_DETECT.starts_with("/open-apis/translation/"));
        assert!(TRANSLATION_V1_TEXT_DETECT.contains("detect"));
        assert!(TRANSLATION_V1_TEXT_TRANSLATE.contains("translate"));
    }

    #[test]
    fn test_service_grouping() {
        // 测试服务分组的正确性
        let document_ai_endpoints = [
            DOCUMENT_AI_RESUME_PARSE,
            DOCUMENT_AI_ID_CARD_RECOGNIZE,
            DOCUMENT_AI_BANK_CARD_RECOGNIZE,
        ];
        for endpoint in document_ai_endpoints {
            assert!(
                endpoint.contains("/document_ai/"),
                "{} 应该包含 /document_ai/",
                endpoint
            );
        }

        let ocr_endpoints = [
            OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE,
            OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE,
        ];
        for endpoint in ocr_endpoints {
            assert!(
                endpoint.contains("/optical_char_recognition/"),
                "{} 应该包含 /optical_char_recognition/",
                endpoint
            );
        }

        let speech_endpoints = [
            SPEECH_TO_TEXT_V1_FILE_RECOGNIZE,
            SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE,
            SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE,
        ];
        for endpoint in speech_endpoints {
            assert!(
                endpoint.contains("/speech_to_text/"),
                "{} 应该包含 /speech_to_text/",
                endpoint
            );
        }

        let translation_endpoints = [TRANSLATION_V1_TEXT_DETECT, TRANSLATION_V1_TEXT_TRANSLATE];
        for endpoint in translation_endpoints {
            assert!(
                endpoint.contains("/translation/"),
                "{} 应该包含 /translation/",
                endpoint
            );
        }
    }

    #[test]
    fn test_backward_compatibility() {
        // 验证兼容性别名
        assert_eq!(RESUME_PARSE, DOCUMENT_AI_RESUME_PARSE);
        assert_eq!(ID_CARD_RECOGNIZE, DOCUMENT_AI_ID_CARD_RECOGNIZE);
        assert_eq!(BANK_CARD_RECOGNIZE, DOCUMENT_AI_BANK_CARD_RECOGNIZE);
        assert_eq!(
            OCR_BASIC_RECOGNIZE,
            OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE
        );
        assert_eq!(SPEECH_RECOGNIZE, SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE);
        assert_eq!(TEXT_TRANSLATE, TRANSLATION_V1_TEXT_TRANSLATE);
        assert_eq!(TEXT_DETECT, TRANSLATION_V1_TEXT_DETECT);
    }

    #[test]
    fn test_version_consistency() {
        // 测试版本一致性
        let v1_endpoints = [
            OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE,
            SPEECH_TO_TEXT_V1_FILE_RECOGNIZE,
            TRANSLATION_V1_TEXT_DETECT,
        ];
        for endpoint in v1_endpoints {
            assert!(endpoint.contains("/v1/"), "{} 应该包含 /v1/", endpoint);
        }
    }
} // Endpoints and EndpointBuilder are now available directly from openlark_core::endpoints

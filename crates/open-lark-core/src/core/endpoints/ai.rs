//! ai 服务端点常量定义
//!
//! AI人工智能相关 API 端点常量，包括：
//! - 文档AI识别
//! - 光学字符识别 (OCR)
//! - 语音转文字
//! - 翻译服务

/// 文档AI - 简历解析
pub const DOCUMENT_AI_RESUME_PARSE: &str = "/open-apis/document_ai/v1/resume_parse";

/// 文档AI - 身份证识别
pub const DOCUMENT_AI_ID_CARD_RECOGNIZE: &str = "/open-apis/document_ai/v1/id_card_recognize";

/// 文档AI - 驾驶证识别
pub const DOCUMENT_AI_DRIVING_LICENSE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/driving_license_recognize";

/// 文档AI - 银行卡识别
pub const DOCUMENT_AI_BANK_CARD_RECOGNIZE: &str = "/open-apis/document_ai/v1/bank_card_recognize";

/// 文档AI - 名片识别
pub const DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/business_card_recognize";

/// 文档AI - 营业执照识别
pub const DOCUMENT_AI_BUSINESS_LICENSE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/business_license_recognize";

/// 文档AI - 中国护照识别
pub const DOCUMENT_AI_CHINESE_PASSPORT_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/chinese_passport_recognize";

/// 文档AI - 合同字段提取
pub const DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION: &str =
    "/open-apis/document_ai/v1/contract_field_extraction";

/// 文档AI - 食品经营许可证识别
pub const DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/food_manage_license_recognize";

/// 文档AI - 食品生产许可证识别
pub const DOCUMENT_AI_FOOD_PRODUCE_LICENSE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/food_produce_license_recognize";

/// 文档AI - 健康证识别
pub const DOCUMENT_AI_HEALTH_CERTIFICATE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/health_certificate_recognize";

/// 文档AI - 港澳通行证识别
pub const DOCUMENT_AI_HKM_MAINLAND_TRAVEL_PERMIT_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/hkm_mainland_travel_permit_recognize";

/// 文档AI - 出租车发票识别
pub const DOCUMENT_AI_TAXI_INVOICE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/taxi_invoice_recognize";

/// 文档AI - 火车票识别
pub const DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/train_invoice_recognize";

/// 文档AI - 台湾通行证识别
pub const DOCUMENT_AI_TW_MAINLAND_TRAVEL_PERMIT_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/tw_mainland_travel_permit_recognize";

/// 文档AI - 增值税发票识别
pub const DOCUMENT_AI_VAT_INVOICE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/vat_invoice_recognize";

/// 文档AI - 机动车发票识别
pub const DOCUMENT_AI_VEHICLE_INVOICE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/vehicle_invoice_recognize";

/// 文档AI - 行驶证识别
pub const DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE: &str =
    "/open-apis/document_ai/v1/vehicle_license_recognize";

/// OCR光学字符识别 - 基础识别
pub const OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE: &str =
    "/open-apis/optical_char_recognition/v1/image/basic_recognize";

/// OCR光学字符识别 - 图片基础识别
pub const OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE: &str =
    "/open-apis/optical_char_recognition/v1/image/basic_recognize";

/// 语音转文字 - 文件识别
pub const SPEECH_TO_TEXT_V1_FILE_RECOGNIZE: &str =
    "/open-apis/speech_to_text/v1/speech/file_recognize";

/// 语音转文字 - 流式识别
pub const SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE: &str =
    "/open-apis/speech_to_text/v1/speech/stream_recognize";

/// 语音转文字 - 语音识别
pub const SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE: &str =
    "/open-apis/speech_to_text/v1/speech/recognize";

/// 翻译服务 - 文本检测
pub const TRANSLATION_V1_TEXT_DETECT: &str = "/open-apis/translation/v1/text/detect";

/// 翻译服务 - 文本翻译
pub const TRANSLATION_V1_TEXT_TRANSLATE: &str = "/open-apis/translation/v1/text/translate";

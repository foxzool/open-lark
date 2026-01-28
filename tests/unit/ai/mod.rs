//! OpenLark AI 模块集成测试
//!
//! 测试所有 AI 相关 API 的请求构建和响应处理。

use open_lark::prelude::*;
use open_lark::AiClient;
use openlark_core::config::Config;

#[cfg(test)]
mod document_ai_tests {
    use super::*;
    use openlark_ai::document_ai::document_ai::v1::recognize::*;

    /// 测试简历解析请求构建
    #[test]
    fn test_resume_parse_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = ResumeParseRequestBuilder::new(config.clone())
            .file_token("file_token_123")
            .is_async(true)
            .body();

        assert_eq!(body.file_token, "file_token_123");
        assert_eq!(body.is_async, Some(true));
    }

    /// 测试身份证识别请求构建
    #[test]
    fn test_id_card_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = IdCardRecognizeRequestBuilder::new(config.clone())
            .file_token("file_token_456")
            .body();

        assert_eq!(body.file_token, "file_token_456");
    }

    /// 测试银行卡识别请求构建
    #[test]
    fn test_bank_card_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = BankCardRecognizeRequestBuilder::new(config.clone())
            .file_token("file_token_789")
            .body();

        assert_eq!(body.file_token, "file_token_789");
    }

    /// 测试营业执照识别请求构建
    #[test]
    fn test_business_license_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = BusinessLicenseRecognizeRequestBuilder::new(config.clone())
            .file_token("file_token_abc")
            .body();

        assert_eq!(body.file_token, "file_token_abc");
    }

    /// 测试增值税发票识别请求构建
    #[test]
    fn test_vat_invoice_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = VatInvoiceRecognizeRequestBuilder::new(config.clone())
            .file_token("file_token_def")
            .body();

        assert_eq!(body.file_token, "file_token_def");
    }

    /// 测试驾驶证识别请求构建
    #[test]
    fn test_driving_license_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = DrivingLicenseRecognizeRequestBuilder::new(config.clone())
            .file_token("file_token_driver")
            .is_async(true)
            .body();

        assert_eq!(body.file_token, "file_token_driver");
        assert_eq!(body.is_async, Some(true));
    }

    /// 测试港澳通行证识别请求构建
    #[test]
    fn test_hkm_mainland_travel_permit_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = HkmMainlandTravelPermitRecognizeRequestBuilder::new(config.clone())
            .file_token("file_token_hkm")
            .body();

        assert_eq!(body.file_token, "file_token_hkm");
    }

    /// 测试行驶证识别请求构建
    #[test]
    fn test_vehicle_license_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = VehicleLicenseRecognizeRequestBuilder::new(config.clone())
            .file_token("file_token_vehicle")
            .body();

        assert_eq!(body.file_token, "file_token_vehicle");
    }
}

#[cfg(test)]
mod ocr_tests {
    use super::*;
    use openlark_ai::ocr::ocr::v1::basic_recognize::*;
    use openlark_ai::ocr::ocr::v1::image_basic_recognize::*;

    /// 测试 OCR 基础识别请求构建
    #[test]
    fn test_basic_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = BasicRecognizeRequestBuilder::new(config.clone())
            .file_token("file_token_ocr")
            .recognition_model(RecognitionModel::TextBox)
            .body();

        assert_eq!(body.file_token, "file_token_ocr");
        assert_eq!(body.recognition_model, Some(RecognitionModel::TextBox));
    }

    /// 测试 OCR 图片基础识别请求构建
    #[test]
    fn test_image_basic_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = ImageBasicRecognizeRequestBuilder::new(config.clone())
            .image_key("image_key_ocr")
            .need_rotation_correction(true)
            .body();

        assert_eq!(body.image_key, "image_key_ocr");
        assert_eq!(body.need_rotation_correction, Some(true));
    }
}

#[cfg(test)]
mod translation_tests {
    use super::*;
    use openlark_ai::translation::translation::v1::text_detect::*;
    use openlark_ai::translation::translation::v1::text_translate::*;

    /// 测试文本翻译请求构建
    #[test]
    fn test_text_translate_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = TextTranslateRequestBuilder::new(config.clone())
            .add_text("Hello")
            .add_text("World")
            .source_language("en-US")
            .target_language("zh-CN")
            .body();

        assert_eq!(body.texts.len(), 2);
        assert_eq!(body.source_language, "en-US");
        assert_eq!(body.target_language, "zh-CN");
    }

    /// 测试文本语言检测请求构建
    #[test]
    fn test_text_detect_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = TextDetectRequestBuilder::new(config.clone())
            .text("你好世界")
            .body();

        assert_eq!(body.text, "你好世界");
    }
}

#[cfg(test)]
mod speech_to_text_tests {
    use super::*;
    use openlark_ai::speech_to_text::speech_to_text::v1::file::*;
    use openlark_ai::speech_to_text::speech_to_text::v1::speech::*;
    use openlark_ai::speech_to_text::speech_to_text::v1::stream::*;

    /// 测试文件识别请求构建
    #[test]
    fn test_file_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = FileRecognizeRequestBuilder::new(config.clone())
            .file_token("file_token_audio")
            .audio_format(AudioFormat::Wav)
            .language("zh-CN")
            .body();

        assert_eq!(body.file_token, "file_token_audio");
        assert_eq!(body.audio_format, AudioFormat::Wav);
        assert_eq!(body.language, "zh-CN");
    }

    /// 测试流式识别请求构建
    #[test]
    fn test_stream_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = StreamRecognizeRequestBuilder::new(config.clone())
            .audio_format(AudioFormat::Mp3)
            .language("en-US")
            .need_intermediate_result(true)
            .body();

        assert_eq!(body.audio_format, AudioFormat::Mp3);
        assert_eq!(body.language, "en-US");
        assert_eq!(body.need_intermediate_result, Some(true));
    }

    /// 测试语音识别请求构建
    #[test]
    fn test_speech_recognize_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let body = SpeechRecognizeRequestBuilder::new(config.clone())
            .audio_data("base64_encoded_data")
            .audio_format(AudioFormat::M4a)
            .language("zh-CN")
            .body();

        assert_eq!(body.audio_data, "base64_encoded_data");
        assert_eq!(body.audio_format, AudioFormat::M4a);
        assert_eq!(body.language, "zh-CN");
    }
}

#[cfg(test)]
mod client_tests {
    use super::*;

    /// 测试 AI 客户端创建
    #[test]
    fn test_ai_client_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let client = AiClient::new(config.clone());
        assert_eq!(client.config().app_id(), "test_app_id");
    }

    /// 测试文档 AI 客户端
    #[test]
    fn test_document_ai_client() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let client = AiClient::new(config.clone());
        let doc_ai = client.document_ai();
        assert_eq!(doc_ai.config().app_id(), "test_app_id");
    }

    /// 测试 OCR 客户端
    #[test]
    fn test_ocr_client() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let client = AiClient::new(config.clone());
        let ocr = client.ocr();
        assert_eq!(ocr.config().app_id(), "test_app_id");
    }

    /// 测试翻译客户端
    #[test]
    fn test_translation_client() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let client = AiClient::new(config.clone());
        let translation = client.translation();
        assert_eq!(translation.config().app_id(), "test_app_id");
    }

    /// 测试语音转文字客户端
    #[test]
    fn test_speech_to_text_client() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let client = AiClient::new(config.clone());
        let speech = client.speech_to_text();
        assert_eq!(speech.config().app_id(), "test_app_id");
    }
}

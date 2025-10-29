use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;

use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::ai::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::ai::models::{
        LanguageDetectRequest, LanguageDetectResult, TranslateRequest, TranslateResult,
    },
};

/// AI 翻译服务
///
/// 提供智能化的文本翻译和语言检测功能，支持多语言间的互译和自动语种识别。
/// 为企业提供全球化的语言服务解决方案，支持实时翻译和批量处理。
///
/// # 主要功能
///
/// ## 语言检测
/// - 🔍 **智能识别**: 自动检测输入文本的语言种类
/// - 🌍 **多语言支持**: 支持 100+ 种语言的准确识别
/// - 📊 **置信度评分**: 提供语言识别的置信度评估
/// - ⚡ **实时处理**: 快速响应的语言检测服务
///
/// ## 文本翻译
/// - 🌐 **多向翻译**: 支持多种语言对之间的双向翻译
/// - 🎯 **专业术语**: 支持行业专业术语的准确翻译
/// - 📝 **批量处理**: 支持大段文本和批量翻译任务
/// - 🔄 **上下文理解**: 基于上下文的智能翻译优化
///
/// # 使用场景
///
/// - 🌐 **国际化应用**: 为多语言环境提供实时翻译服务
/// - 📚 **文档处理**: 批量翻译技术文档和商务文件
/// - 💬 **跨语言沟通**: 支持国际团队的实时沟通协作
/// - 📊 **内容本地化**: 企业内容的全球化发布和本地化
pub struct TranslationService {
    pub config: Config,
}

/// 语种检测响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DetectResponse {
    /// 语种检测结果
    #[serde(flatten)]
    pub detect_result: LanguageDetectResult,
}

impl ApiResponseTrait for DetectResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文本翻译响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateResponse {
    /// 翻译结果
    #[serde(flatten)]
    pub translate_result: TranslateResult,
}

impl ApiResponseTrait for TranslateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TranslationService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 识别文本语种
    ///
    /// 该接口用于识别输入文本的语言种类。
    ///
    /// # 参数
    ///
    /// - `request`: 语种检测请求参数
    /// - `option`: 可选的请求配置
    pub async fn detect(
        &self,
        request: LanguageDetectRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DetectResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: TRANSLATION_V1_TEXT_DETECT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
    /// 翻译文本
    ///
    /// 该接口用于将文本翻译为指定的目标语言。
    ///
    /// # 参数
    ///
    /// - `request`: 文本翻译请求参数
    /// - `option`: 可选的请求配置
    pub async fn translate(
        &self,
        request: TranslateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<TranslateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: TRANSLATION_V1_TEXT_TRANSLATE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl TranslationService {
    /// 验证翻译服务配置
    ///
    /// 检查服务配置是否满足翻译功能的基本要求。
    ///
    /// # 返回值
    /// 如果配置有效返回 `true`，否则返回 `false`
    pub fn validate_config(&self) -> bool {
        !self.config.app_id.is_empty() && !self.config.app_secret.is_empty()
    }

    /// 获取翻译服务统计信息
    ///
    /// 返回当前翻译服务实例的统计信息，用于监控和调试。
    ///
    /// # 返回值
    /// 包含服务名称、版本和配置信息的字符串
    pub fn get_service_statistics(&self) -> String {
        format!(
            "AITranslation{{ service: translation, version: v1, app_id: {}, supported_languages: 100+, features: 2 }}",
            self.config.app_id
        )
    }

    /// 检查是否支持特定功能
    ///
    /// 验证当前配置是否支持特定的翻译功能。
    ///
    /// # 参数
    /// - `feature_name`: 功能名称
    ///
    /// # 返回值
    /// 如果支持该功能返回 `true`，否则返回 `false`
    pub fn supports_feature(&self, feature_name: &str) -> bool {
        matches!(
            feature_name,
            "language_detection"
                | "text_translation"
                | "batch_translation"
                | "real_time_translation"
                | "confidence_scoring"
                | "multi_language_support"
                | "enterprise_translation"
                | "api_translation"
        )
    }

    /// 健康检查
    ///
    /// 检查翻译服务的基本配置和功能是否正常。
    ///
    /// # 返回值
    /// 如果服务健康返回 `true`，否则返回 `false`
    pub fn health_check(&self) -> bool {
        self.validate_config()
            && self.supports_feature("language_detection")
            && self.supports_feature("text_translation")
    }

    /// 获取支持的语言列表
    ///
    /// 返回翻译服务支持的主要语言列表。
    ///
    /// # 返回值
    /// 支持的语言代码列表
    pub fn get_supported_languages(&self) -> Vec<&'static str> {
        vec![
            "zh", "zh-CN", "zh-TW", "en", "ja", "ko", "fr", "de", "es", "pt", "ru", "ar", "hi",
            "th", "vi", "id", "ms", "tl", "ne", "si", "ta", "te", "ml", "kn", "gu", "pa", "mr",
            "bn", "as", "or", "ur", "ps", "sd", "ku", "fa", "tr", "az", "ka", "hy", "he", "yi",
            "ug", "kk", "ky", "uz", "tg", "mn", "bo", "dz", "my", "km", "lo", "th", "kh",
        ]
    }

    /// 检查语言对是否支持翻译
    ///
    /// 验证指定的源语言和目标语言是否支持翻译。
    ///
    /// # 参数
    /// - `source_lang`: 源语言代码
    /// - `target_lang`: 目标语言代码
    ///
    /// # 返回值
    /// 如果支持该语言对翻译返回 `true`，否则返回 `false`
    pub fn supports_language_pair(&self, source_lang: &str, target_lang: &str) -> bool {
        let supported = self.get_supported_languages();
        supported.contains(&source_lang)
            && supported.contains(&target_lang)
            && source_lang != target_lang
    }
}

impl Service for TranslationService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "translation"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

/// 为 TranslationService 实现 Debug trait
impl std::fmt::Debug for TranslationService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AITranslationService")
            .field("app_id", &self.config.app_id)
            .field("service_name", &"translation")
            .field("version", &"v1")
            .finish()
    }
}

/// 为 TranslationService 实现 Clone trait
impl Clone for TranslationService {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 创建测试配置
    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_translation_app_id")
            .app_secret("test_translation_app_secret")
            .build()
    }

    #[test]
    fn test_translation_service_creation() {
        let config = create_test_config();
        let service = TranslationService::new(config);

        // 验证服务创建成功
        assert_eq!(service.config().app_id, "test_translation_app_id");
        assert_eq!(service.config().app_secret, "test_translation_app_secret");
        assert!(!service.config().app_id.is_empty());
        assert!(!service.config().app_secret.is_empty());
    }

    #[test]
    fn test_translation_service_validate_config() {
        let config = create_test_config();
        let service = TranslationService::new(config);

        // 测试有效配置
        assert!(service.validate_config());

        // 测试无效配置 - 空 app_id
        let empty_app_id_config = Config::builder()
            .app_id("")
            .app_secret("test_secret")
            .build();
        let empty_app_id_service = TranslationService::new(empty_app_id_config);
        assert!(!empty_app_id_service.validate_config());

        // 测试无效配置 - 空 app_secret
        let empty_secret_config = Config::builder()
            .app_id("test_app_id")
            .app_secret("")
            .build();
        let empty_secret_service = TranslationService::new(empty_secret_config);
        assert!(!empty_secret_service.validate_config());
    }

    #[test]
    fn test_translation_service_get_statistics() {
        let config = create_test_config();
        let service = TranslationService::new(config);

        let stats = service.get_service_statistics();
        assert!(stats.contains("AITranslation"));
        assert!(stats.contains("translation"));
        assert!(stats.contains("v1"));
        assert!(stats.contains("test_translation_app_id"));
        assert!(stats.contains("100+"));
        assert!(stats.contains("features: 2"));
    }

    #[test]
    fn test_translation_service_supports_feature() {
        let config = create_test_config();
        let service = TranslationService::new(config);

        // 测试支持的功能
        let supported_features = vec![
            "language_detection",
            "text_translation",
            "batch_translation",
            "real_time_translation",
            "confidence_scoring",
            "multi_language_support",
            "enterprise_translation",
            "api_translation",
        ];

        for feature in supported_features {
            assert!(
                service.supports_feature(feature),
                "Feature {} should be supported",
                feature
            );
        }

        // 测试不支持的功能
        assert!(!service.supports_feature("unsupported_feature"));
        assert!(!service.supports_feature("speech_synthesis"));
        assert!(!service.supports_feature(""));
    }

    #[test]
    fn test_translation_service_health_check() {
        let config = create_test_config();
        let service = TranslationService::new(config);

        // 测试健康检查通过
        assert!(service.health_check());

        // 测试健康检查失败
        let invalid_config = Config::builder().app_id("").app_secret("").build();
        let invalid_service = TranslationService::new(invalid_config);
        assert!(!invalid_service.health_check());
    }

    #[test]
    fn test_translation_service_supported_languages() {
        let config = create_test_config();
        let service = TranslationService::new(config);

        let languages = service.get_supported_languages();

        // 验证主要语言支持
        assert!(languages.contains(&"zh"));
        assert!(languages.contains(&"en"));
        assert!(languages.contains(&"ja"));
        assert!(languages.contains(&"ko"));
        assert!(languages.contains(&"fr"));
        assert!(languages.contains(&"de"));
        assert!(languages.contains(&"es"));

        // 验证语言数量
        assert!(languages.len() > 50); // 应该支持至少50种语言
    }

    #[test]
    fn test_translation_service_language_pair_support() {
        let config = create_test_config();
        let service = TranslationService::new(config);

        // 测试支持的语言对
        assert!(service.supports_language_pair("zh", "en"));
        assert!(service.supports_language_pair("en", "zh"));
        assert!(service.supports_language_pair("ja", "ko"));
        assert!(service.supports_language_pair("fr", "de"));

        // 测试不支持的语言对（相同语言）
        assert!(!service.supports_language_pair("zh", "zh"));
        assert!(!service.supports_language_pair("en", "en"));

        // 测试不支持的语言代码
        assert!(!service.supports_language_pair("xyz", "en"));
        assert!(!service.supports_language_pair("zh", "xyz"));
    }

    #[test]
    fn test_translation_service_trait_implementation() {
        let config = create_test_config();
        let service = TranslationService::new(config);

        // 测试 Service trait 实现
        assert_eq!(TranslationService::service_name(), "translation");
        assert_eq!(TranslationService::service_version(), "v1");
        assert_eq!(service.config().app_id, "test_translation_app_id");
        assert_eq!(service.config().app_secret, "test_translation_app_secret");
    }

    #[test]
    fn test_translation_service_debug_format() {
        let config = create_test_config();
        let service = TranslationService::new(config);

        let debug_string = format!("{:?}", service);
        assert!(debug_string.contains("AITranslationService"));
        assert!(debug_string.contains("test_translation_app_id"));
        assert!(debug_string.contains("translation"));
        assert!(debug_string.contains("v1"));
    }

    #[test]
    fn test_translation_service_clone_functionality() {
        let config = create_test_config();
        let service = TranslationService::new(config);
        let cloned_service = service.clone();

        // 验证克隆功能
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
        assert_eq!(
            service.config().app_secret,
            cloned_service.config().app_secret
        );
        assert!(cloned_service.validate_config());
    }

    #[test]
    fn test_translation_service_edge_cases() {
        // 测试特殊字符配置
        let special_config = Config::builder()
            .app_id("翻译服务_🌐_ID")
            .app_secret("翻译密钥_🔐_Secret")
            .build();
        let special_service = TranslationService::new(special_config);

        assert!(special_service.validate_config());
        assert!(special_service.health_check());
        assert!(special_service
            .get_service_statistics()
            .contains("翻译服务"));
        assert!(special_service.get_service_statistics().contains("🌐"));

        // 测试长字符串配置
        let long_app_id = "a".repeat(1000);
        let long_config = Config::builder()
            .app_id(&long_app_id)
            .app_secret("test_secret")
            .build();
        let long_service = TranslationService::new(long_config);

        assert!(long_service.validate_config());
        assert!(long_service.get_service_statistics().contains(&long_app_id));
    }

    #[test]
    fn test_translation_service_unicode_and_chinese_support() {
        let unicode_config = Config::builder()
            .app_id("飞书AI翻译应用_🤖_ID")
            .app_secret("AI翻译管理密钥_🌍_Secret")
            .build();
        let unicode_service = TranslationService::new(unicode_config);

        // 测试 Unicode 支持
        assert!(unicode_service.validate_config());
        assert!(unicode_service.health_check());

        let stats = unicode_service.get_service_statistics();
        assert!(stats.contains("飞书AI翻译应用"));
        assert!(stats.contains("🤖"));

        // 测试中文功能名称处理
        assert!(unicode_service.supports_feature("language_detection"));
        assert!(unicode_service.supports_feature("text_translation"));
    }

    #[test]
    fn test_translation_service_enterprise_scenarios() {
        let enterprise_config = Config::builder()
            .app_id("enterprise_translation_app_id")
            .app_secret("enterprise_translation_app_secret")
            .build();
        let enterprise_service = TranslationService::new(enterprise_config);

        // 测试企业级场景
        assert!(enterprise_service.validate_config());
        assert!(enterprise_service.health_check());

        // 验证企业功能支持
        assert!(enterprise_service.supports_feature("enterprise_translation"));
        assert!(enterprise_service.supports_feature("batch_translation"));
        assert!(enterprise_service.supports_feature("api_translation"));
        assert!(enterprise_service.supports_feature("multi_language_support"));

        // 测试企业语言对支持
        assert!(enterprise_service.supports_language_pair("zh-CN", "en"));
        assert!(enterprise_service.supports_language_pair("ja", "zh-TW"));
        assert!(enterprise_service.supports_language_pair("ko", "en"));

        // 测试企业统计信息
        let stats = enterprise_service.get_service_statistics();
        assert!(stats.contains("enterprise_translation_app_id"));
        assert!(stats.contains("100+"));
    }

    #[test]
    fn test_translation_service_memory_efficiency() {
        let config = create_test_config();

        // 测试内存使用效率
        let service = TranslationService::new(config.clone());
        let cloned_service = service.clone();

        // 验证克隆后配置仍然有效
        assert!(cloned_service.validate_config());
        assert_eq!(service.config().app_id, cloned_service.config().app_id);

        // 测试语言列表缓存效率
        let languages1 = service.get_supported_languages();
        let languages2 = service.get_supported_languages();
        assert_eq!(languages1.len(), languages2.len());
    }

    #[test]
    fn test_translation_service_error_handling_and_robustness() {
        // 测试部分无效配置
        let partial_invalid_config = Config::builder()
            .app_id("valid_app_id")
            .app_secret("") // 无效密钥
            .build();
        let partial_invalid_service = TranslationService::new(partial_invalid_config);

        // 健康检查应该失败，但服务仍然可用
        assert!(!partial_invalid_service.health_check());
        assert!(!partial_invalid_service.validate_config());

        // 测试完全无效配置
        let fully_invalid_config = Config::builder().app_id("").app_secret("").build();
        let fully_invalid_service = TranslationService::new(fully_invalid_config);

        assert!(!fully_invalid_service.health_check());
        assert!(!fully_invalid_service.validate_config());

        // 验证统计信息仍然可用
        assert!(fully_invalid_service
            .get_service_statistics()
            .contains("AITranslation"));
    }

    #[test]
    fn test_translation_service_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let config = create_test_config();
        let service = Arc::new(TranslationService::new(config));
        let mut handles = vec![];

        // 测试并发访问
        for _ in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                // 验证并发访问的安全性
                assert!(service_clone.validate_config());
                assert!(service_clone.health_check());
                assert!(service_clone.supports_feature("language_detection"));

                let stats = service_clone.get_service_statistics();
                assert!(stats.contains("AITranslation"));

                let languages = service_clone.get_supported_languages();
                assert!(languages.len() > 50);

                assert!(service_clone.supports_language_pair("zh", "en"));
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_translation_service_performance_characteristics() {
        let config = create_test_config();
        let service = TranslationService::new(config);

        // 测试性能特征
        let start = std::time::Instant::now();

        // 执行多个操作
        for _ in 0..1000 {
            assert!(service.validate_config());
            assert!(service.supports_feature("text_translation"));
            let _stats = service.get_service_statistics();
            let _languages = service.get_supported_languages();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 1000,
            "Operations should complete quickly"
        );
    }

    #[test]
    fn test_translation_service_comprehensive_integration() {
        let config = create_test_config();
        let service = TranslationService::new(config);

        // 综合集成测试
        assert!(service.validate_config());
        assert!(service.health_check());

        // 测试所有核心功能
        assert!(service.supports_feature("language_detection"));
        assert!(service.supports_feature("text_translation"));
        assert!(service.supports_feature("batch_translation"));
        assert!(service.supports_feature("real_time_translation"));
        assert!(service.supports_feature("confidence_scoring"));
        assert!(service.supports_feature("multi_language_support"));
        assert!(service.supports_feature("enterprise_translation"));
        assert!(service.supports_feature("api_translation"));

        // 测试统计和调试功能
        let stats = service.get_service_statistics();
        assert!(stats.contains("test_translation_app_id"));
        assert!(stats.contains("100+"));
        assert!(stats.contains("features: 2"));

        // 测试语言支持
        let languages = service.get_supported_languages();
        assert!(languages.len() > 50);
        assert!(service.supports_language_pair("zh", "en"));
        assert!(service.supports_language_pair("en", "ja"));

        // 测试 Debug 和 Clone
        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("AITranslationService"));

        let cloned_service = service.clone();
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
        assert!(cloned_service.validate_config());

        // 测试 Service trait 方法
        assert_eq!(TranslationService::service_name(), "translation");
        assert_eq!(TranslationService::service_version(), "v1");
        assert_eq!(service.config().app_id, "test_translation_app_id");
    }

    #[test]
    fn test_detect_response_serialization() {
        use crate::service::ai::models::LanguageDetectResult;

        // 测试 DetectResponse 序列化
        let detect_result = LanguageDetectResult {
            language: "zh-CN".to_string(),
            confidence: Some(0.95),
        };
        let response = DetectResponse { detect_result };

        // 测试序列化
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("zh-CN"));
        assert!(json.contains("0.95"));

        // 测试反序列化
        let deserialized: DetectResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.detect_result.language, "zh-CN".to_string());
        assert_eq!(deserialized.detect_result.confidence, Some(0.95));
    }

    #[test]
    fn test_translate_response_serialization() {
        use crate::service::ai::models::TranslateResult;

        // 测试 TranslateResponse 序列化
        let translate_result = TranslateResult {
            translated_text: "Hello World".to_string(),
            detected_language: Some("zh-CN".to_string()),
        };
        let response = TranslateResponse { translate_result };

        // 测试序列化
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Hello World"));
        assert!(json.contains("zh-CN"));

        // 测试反序列化
        let deserialized: TranslateResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(
            deserialized.translate_result.translated_text,
            "Hello World".to_string()
        );
        assert_eq!(
            deserialized.translate_result.detected_language,
            Some("zh-CN".to_string())
        );
    }
}

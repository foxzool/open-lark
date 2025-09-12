//! 人工智能（AI）服务
//!
//! 提供飞书开放平台的AI能力接口，整合了文档智能、图像识别、语音处理、
//! 机器翻译等多种人工智能技术，为企业应用提供强大的智能化能力支持。
//!
//! # 核心功能
//!
//! ## 智能文档处理 (Document AI)
//! - 📄 简历智能解析和结构化
//! - 🆔 证件识别（身份证、护照等）
//! - 🧾 发票识别和信息提取
//! - 📝 合同识别和关键信息抽取
//! - 💳 名片识别和联系人信息提取
//! - 📊 表格识别和数据结构化
//!
//! ## 光学字符识别 (OCR)
//! - 🖼️ 图片文字识别和提取
//! - 📱 多语言文字识别支持
//! - 🎯 高精度文字定位
//! - 📝 手写文字识别
//! - 📋 表格和结构化文本识别
//!
//! ## 语音识别 (Speech-to-Text)
//! - 🎤 语音文件转文字
//! - 🔄 流式语音实时识别
//! - 🌍 多语言语音识别
//! - 📞 通话录音转写
//! - 🎯 语音关键词检测
//!
//! ## 机器翻译 (Translation)
//! - 🌐 多语种自动检测
//! - 📝 文本翻译和语言转换
//! - 🎯 专业领域术语翻译
//! - 📱 实时翻译能力
//! - 🔄 批量文本翻译
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取AI服务
//! let ai = &client.ai;
//!
//! // 文档AI - 简历解析
//! // let resume_request = ParseResumeRequest::builder()
//! //     .file_content(file_data)
//! //     .build();
//! // let resume_data = ai.document_ai.parse_resume(resume_request, None).await?;
//!
//! // OCR - 图片文字识别
//! // let ocr_request = RecognizeTextRequest::builder()
//! //     .image_content(image_data)
//! //     .build();
//! // let text_result = ai.optical_char_recognition.recognize_text(ocr_request, None).await?;
//!
//! // 语音识别
//! // let speech_request = SpeechToTextRequest::builder()
//! //     .audio_content(audio_data)
//! //     .format("wav")
//! //     .build();
//! // let text_result = ai.speech_to_text.recognize(speech_request, None).await?;
//!
//! // 机器翻译
//! // let translate_request = TranslateRequest::builder()
//! //     .source_language("zh")
//! //     .target_language("en")
//! //     .text("你好世界")
//! //     .build();
//! // let translation = ai.translation.translate(translate_request, None).await?;
//! ```
//!
//! # AI能力特性
//!
//! - 🤖 先进的深度学习模型
//! - ⚡ 毫秒级响应时间
//! - 🎯 高精度识别准确率
//! - 🌍 多语言和多格式支持
//! - 🔒 数据安全和隐私保护
//!
//! # 应用场景
//!
//! - 📋 人力资源自动化处理
//! - 🏢 办公文档数字化
//! - 📞 会议记录和转写
//! - 🌐 多语言内容处理
//! - 📊 数据录入自动化
//!
//! # 技术优势
//!
//! - 基于飞书自研AI模型
//! - 持续学习和模型优化
//! - 企业级服务可用性
//! - 灵活的API调用方式
//! - 完善的错误处理机制

use std::sync::Arc;

use crate::core::config::Config;

/// 智能文档处理功能
pub mod document_ai;
/// 数据模型定义
pub mod models;
/// 光学字符识别功能
pub mod optical_char_recognition;
/// 语音识别功能
pub mod speech_to_text;
/// 机器翻译功能
pub mod translation;

use document_ai::DocumentAiService;
use optical_char_recognition::OpticalCharRecognitionService;
use speech_to_text::SpeechToTextService;
use translation::TranslationService;

/// AI服务
///
/// 人工智能能力的统一管理入口，整合了文档处理、图像识别、
/// 语音处理、机器翻译等多种AI技术服务。
///
/// # 服务架构
///
/// - **document_ai**: 智能文档处理和信息抽取
/// - **optical_char_recognition**: 光学字符识别和文字提取
/// - **speech_to_text**: 语音识别和语音转文字
/// - **translation**: 机器翻译和语言转换
/// - **models**: 数据模型和结构定义
///
/// # 核心特性
///
/// - 🧠 先进的AI算法和模型
/// - 🚀 高性能并发处理能力
/// - 🎯 高精度识别和转换
/// - 🌍 多语言和多格式支持
/// - 🔐 企业级安全和隐私保护
///
/// # 适用场景
///
/// - 企业办公自动化
/// - 文档数字化转换
/// - 多媒体内容处理
/// - 跨语言沟通协作
/// - 数据智能提取分析
///
/// # 最佳实践
///
/// - 选择合适的AI服务类型
/// - 优化输入数据质量
/// - 合理设置请求参数
/// - 处理异步和批量任务
/// - 监控API使用量和成本
pub struct AiService {
    /// 智能文档处理服务 - 解析各类文档并提取结构化信息
    pub document_ai: DocumentAiService,
    /// 光学字符识别服务 - 从图像中识别和提取文字
    pub optical_char_recognition: OpticalCharRecognitionService,
    /// 语音识别服务 - 将语音转换为文字
    pub speech_to_text: SpeechToTextService,
    /// 机器翻译服务 - 提供多语言翻译能力
    pub translation: TranslationService,
}

impl AiService {
    /// 创建新的AI服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的AI服务实例，包含所有AI能力子服务
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            document_ai: DocumentAiService::new(Arc::clone(&config)),
            optical_char_recognition: OpticalCharRecognitionService::new(Arc::clone(&config)),
            speech_to_text: SpeechToTextService::new(Arc::clone(&config)),
            translation: TranslationService::new(config),
        }
    }
}

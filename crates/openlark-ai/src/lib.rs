//! Open-Lark AI Module
//!
//! 飞书AI智能服务模块，提供完整的AI和智能助手功能。
//!
//! ## 主要功能
//!
//! - **文档AI识别**: 简历、身份证、驾驶证、银行卡、营业执照等证件识别
//! - **AI嵌入服务**: 文本、对话、图像嵌入和相似度计算
//! - **AI工作流**: 智能工作流的创建、运行和管理
//! - **Aily智能助手**: 会话管理、知识库、技能管理、消息交互
//!
//! ## 使用示例
//!
//! ```rust
//! use openlark_ai::{AiService, endpoints::*};
//!
//! // 使用端点常量
//! let resume_endpoint = DOCUMENT_AI_RESUME_PARSE;
//! let ocr_endpoint = OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE;
//! let translate_endpoint = TRANSLATION_V1_TEXT_TRANSLATE;
//! println!("简历解析端点: {}", resume_endpoint);
//! println!("OCR识别端点: {}", ocr_endpoint);
//! println!("文本翻译端点: {}", translate_endpoint);
//! ```
//!
//! ## 端点组织
//!
//! - `document_ai`: 文档AI识别API端点
//! - `ai_embedding`: AI嵌入服务API端点
//! - `ai_workflow`: AI工作流API端点
//! - `aily`: Aily智能助手API端点

#![deny(missing_docs)]

// 导入端点模块
pub mod endpoints;

// AI service modules
pub mod ai;

/// Re-exports from open-lark-core for convenience.
pub mod prelude {
    pub use openlark_core::SDKResult;
}

// 重新导出端点常量，方便外部使用
pub use endpoints::*;

// Re-export service types for convenience
pub use ai::AiService;

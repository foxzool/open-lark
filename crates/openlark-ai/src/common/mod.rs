//! 通用工具模块
//!
//! 提供 API 相关的通用工具函数，包括响应数据提取、参数序列化等。

pub mod api_utils;

// 链式调用入口
pub mod chain;

// 重导出 API 工具函数，方便外部使用
pub use api_utils::{ensure_success, extract_response_data, serialize_params};

// 重导出链式调用入口
pub use chain::{DocumentAiClient, DocumentAiV1Client, RecognizeResource};

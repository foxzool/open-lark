/// Lingo智能文本处理模块
///
/// 提供Lingo语言的智能处理功能，包括摘要生成、关键词提取、文本翻译等。

use openlark_core::config::Config;

// 导出具体的API实现
pub mod generate_summary;
pub mod extract_keywords;
pub mod translate_text;

// 重新导出API函数
pub use generate_summary::*;
pub use extract_keywords::*;
pub use translate_text::*;

/// Lingo智能处理服务
#[derive(Debug, Clone)]
pub struct LingoTextService {
    config: Config,
}

impl LingoTextService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Default for LingoTextService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}
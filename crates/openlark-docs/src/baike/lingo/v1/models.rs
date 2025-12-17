/// Lingo语言服务数据模型
///
/// 定义Lingo相关的数据结构。

use serde::{Deserialize, Serialize};

/// 草稿数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LingoDraft {
    /// 草稿ID
    pub draft_id: String,
    /// 草稿标题
    pub title: String,
    /// 草稿内容
    pub content: String,
    /// 草稿状态
    pub status: DraftStatus,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
}

/// 草稿状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftStatus {
    /// 草稿
    Draft,
    /// 已发布
    Published,
}

/// 词条数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LingoEntity {
    /// 词条ID
    pub entity_id: String,
    /// 词条标题
    pub title: String,
    /// 词条内容
    pub content: String,
    /// 词条类型
    pub entity_type: String,
    /// 词条标签
    pub tags: Vec<String>,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
}

/// 文本处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextProcessResult {
    /// 处理后的文本
    pub processed_text: String,
    /// 置信度
    pub confidence: f64,
    /// 处理时间戳
    pub process_time: String,
}

/// 摘要生成结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryResult {
    /// 摘要内容
    pub summary: String,
    /// 摘要长度
    pub length: usize,
    /// 关键句子
    pub key_sentences: Vec<String>,
}

/// 关键词提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordsResult {
    /// 关键词列表
    pub keywords: Vec<Keyword>,
    /// 总权重
    pub total_weight: f64,
}

/// 关键词
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword {
    /// 关键词
    pub word: String,
    /// 权重
    pub weight: f64,
    /// 位置
    pub positions: Vec<usize>,
}

/// 翻译结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    /// 翻译后的文本
    pub translated_text: String,
    /// 源语言
    pub source_language: String,
    /// 目标语言
    pub target_language: String,
    /// 翻译置信度
    pub confidence: f64,
}
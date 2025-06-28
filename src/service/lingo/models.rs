use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 草稿状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftStatus {
    /// 草稿中
    Draft,
    /// 已发布
    Published,
    /// 已拒绝
    Rejected,
}

/// 草稿信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Draft {
    /// 草稿ID
    pub draft_id: String,
    /// 词条ID（如果是更新现有词条的草稿）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// 主名称
    pub main_keys: Vec<String>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// 词条释义富文本
    pub description: String,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_id: Option<String>,
    /// 外链（用于跳转到释义页面）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    /// 相关词条ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    /// 统计信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
    /// 草稿状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DraftStatus>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 词条信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// 词条ID
    pub id: String,
    /// 主名称
    pub main_keys: Vec<String>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// 词条释义富文本
    pub description: String,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_id: Option<String>,
    /// 外链（用于跳转到释义页面）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    /// 相关词条ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    /// 统计信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
    /// 词库ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 外链信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OuterInfo {
    /// 链接提供方
    pub provider: String,
    /// 外部链接
    pub outer_url: String,
}

/// 相关词条信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedMeta {
    /// 相关用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// 相关群组列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chats: Option<Vec<String>>,
    /// 相关文档列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<Vec<String>>,
    /// 相关链接列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oncalls: Option<Vec<String>>,
    /// 相关链接列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// 相关词条列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviations: Option<Vec<String>>,
    /// 分类列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifications: Option<Vec<String>>,
    /// 图片列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
}

/// 统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    /// 点赞数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i32>,
    /// 不喜欢数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dislike_count: Option<i32>,
}

/// 分类信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classification {
    /// 分类ID
    pub id: String,
    /// 分类名称
    pub name: String,
    /// 父分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub father_id: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 词库信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    /// 词库ID
    pub id: String,
    /// 词库名称
    pub name: String,
    /// 词库描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 词库类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_type: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 词条搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySearchResult {
    /// 词条信息
    pub entity: Entity,
    /// 匹配的关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_keys: Option<Vec<String>>,
    /// 匹配分数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

/// 词条匹配结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMatchResult {
    /// 词条信息
    pub entity: Entity,
    /// 匹配的词
    pub matched_word: String,
    /// 匹配类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
}

/// 高亮范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightRange {
    /// 开始位置
    pub start: i32,
    /// 结束位置
    pub end: i32,
    /// 词条ID
    pub entity_id: String,
}

/// 高亮结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightResult {
    /// 原始文本
    pub text: String,
    /// 高亮范围列表
    pub ranges: Vec<HighlightRange>,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件ID
    pub file_token: String,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 上传时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<i64>,
}

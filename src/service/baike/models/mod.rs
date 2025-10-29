// baike模块的数据模型定义

use serde::{Deserialize, Serialize};
/// 词条信息
#[derive(.*?)]
pub struct Entity {
    /// 词条ID
#[serde(rename = "entity_id")],
    pub entity_id: String,
    /// 词条名称
    pub name: String,
    /// 词条描述
    pub description: Option<String>,
    /// 词条内容
    pub content: Option<String>,
    /// 词条状态
#[serde(rename = "entity_status")],
    pub entity_status: EntityStatus,
    /// 词条类型
#[serde(rename = "entity_type")],
    pub entity_type: EntityType,
    /// 分类列表
    pub categories: Option<Vec<Category>>,
    /// 创建者信息
    pub creator: Option<CreatorInfo>,
    /// 创建时间
#[serde(rename = "create_time")],
    pub create_time: Option<String>,
    /// 更新时间
#[serde(rename = "update_time")],
    pub update_time: Option<String>,
    /// 审核状态
#[serde(rename = "audit_status")],
    pub audit_status: Option<AuditStatus>,
    /// 审核人信息
#[serde(rename = "auditor")],
    pub auditor: Option<CreatorInfo>,
    /// 审核时间
#[serde(rename = "audit_time")],
    pub audit_time: Option<String>,
    /// 标签列表
    pub tags: Option<Vec<String>>,
    /// 关联词条
#[serde(rename = "related_entities")],
    pub related_entities: Option<Vec<Entity>>,
}
/// 词条状态
#[derive(.*?)]
#[serde(rename_all = "snake_case")],
pub enum EntityStatus {,
/// 草稿
    Draft,
    /// 待审核
    Pending,
    /// 已发布
    Published,
    /// 已驳回
    Rejected,
    /// 已下线
    Offline,
}
/// 词条类型
#[derive(.*?)]
#[serde(rename_all = "snake_case")],
pub enum EntityType {,
/// 普通词条
    Normal,
    /// 专业术语
    Professional,
    /// 业务术语
    Business,
    /// 技术术语
    Technical,
}
/// 分类信息
#[derive(.*?)]
pub struct Category {
    /// 分类ID
#[serde(rename = "category_id")],
    pub category_id: String,
    /// 分类名称
    pub name: String,
    /// 父分类ID
#[serde(rename = "parent_category_id")],
    pub parent_category_id: Option<String>,
    /// 分类层级
    pub level: Option<i32>,
    /// 分类描述
    pub description: Option<String>,
}
/// 创建者信息
#[derive(.*?)]
pub struct CreatorInfo {
    /// 用户ID
#[serde(rename = "user_id")],
    pub user_id: String,
    /// 用户名
#[serde(rename = "user_name")],
    pub user_name: Option<String>,
    /// 用户头像
#[serde(rename = "avatar")],
    pub avatar: Option<String>,
}
/// 审核状态
#[derive(.*?)]
#[serde(rename_all = "snake_case")],
pub enum AuditStatus {,
/// 待审核
    Pending,
    /// 已通过
    Approved,
    /// 已驳回
    Rejected,
}
/// 草稿信息
#[derive(.*?)]
pub struct Draft {
    /// 草稿ID
#[serde(rename = "draft_id")],
    pub draft_id: String,
    /// 草稿标题
    pub title: String,
    /// 草稿内容
    pub content: String,
    /// 草稿类型
#[serde(rename = "draft_type")],
    pub draft_type: DraftType,
    /// 创建者信息
    pub creator: CreatorInfo,
    /// 创建时间
#[serde(rename = "create_time")],
    pub create_time: String,
    /// 更新时间
#[serde(rename = "update_time")],
    pub update_time: String,
    /// 是否已提交审核
#[serde(rename = "submitted_for_audit")],
    pub submitted_for_audit: bool,
    /// 关联的词条ID（如果已发布）
#[serde(rename = "entity_id")],
    pub entity_id: Option<String>,
}
/// 草稿类型
#[derive(.*?)]
#[serde(rename_all = "snake_case")],
pub enum DraftType {,
/// 新建词条
    NewEntity,
    /// 编辑词条
    EditEntity,
}
/// 搜索结果
#[derive(.*?)]
pub struct SearchResult {
    /// 词条信息
    pub entity: Entity,
    /// 匹配度分数
#[serde(rename = "match_score")],
    pub match_score: f64,
    /// 高亮片段
#[serde(rename = "highlight_snippets")],
    pub highlight_snippets: Option<Vec<String>>,
    /// 匹配位置
#[serde(rename = "match_positions")],
    pub match_positions: Option<Vec<MatchPosition>>,
}
/// 匹配位置
#[derive(.*?)]
pub struct MatchPosition {
    /// 匹配开始位置
#[serde(rename = "start_position")],
    pub start_position: i32,
    /// 匹配结束位置
#[serde(rename = "end_position")],
    pub end_position: i32,
    /// 匹配类型
#[serde(rename = "match_type")],
    pub match_type: MatchType,
}
/// 匹配类型
#[derive(.*?)]
#[serde(rename_all = "snake_case")],
pub enum MatchType {,
/// 精确匹配
    Exact,
    /// 模糊匹配
    Fuzzy,
    /// 前缀匹配
    Prefix,
    /// 后缀匹配
    Suffix,
}
/// 高亮信息
#[derive(.*?)]
pub struct HighlightInfo {
    /// 高亮文本
#[serde(rename = "highlighted_text")],
    pub highlighted_text: String,
    /// 原始文本
#[serde(rename = "original_text")],
    pub original_text: String,
    /// 高亮位置
#[serde(rename = "highlight_positions")],
    pub highlight_positions: Vec<HighlightPosition>,
}
/// 高亮位置
#[derive(.*?)]
pub struct HighlightPosition {
    /// 开始位置
#[serde(rename = "start")],
    pub start: i32,
    /// 结束位置
#[serde(rename = "end")],
    pub end: i32,
    /// 高亮样式
#[serde(rename = "highlight_style")],
    pub highlight_style: Option<String>,
}
/// 统计信息
#[derive(.*?)]
pub struct DictionaryStats {
    /// 总词条数
#[serde(rename = "total_entities")],
    pub total_entities: i32,
    /// 已发布词条数
#[serde(rename = "published_entities")],
    pub published_entities: i32,
    /// 待审核词条数
#[serde(rename = "pending_entities")],
    pub pending_entities: i32,
    /// 草稿数
#[serde(rename = "draft_count")],
    pub draft_count: i32,
    /// 分类数
#[serde(rename = "category_count")],
    pub category_count: i32,
    /// 贡献者数
#[serde(rename = "contributor_count")],
    pub contributor_count: i32,
}
/// 通用响应结构
#[derive(.*?)]
pub struct BaikeResponse<T> {,
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
}
/// 分页响应结构
#[derive(.*?)]
pub struct PaginatedBaikeResponse<T> {,
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<PaginatedBaikeData<T>>,
}
/// 分页数据
#[derive(.*?)]
pub struct PaginatedBaikeData<T> {,
    /// 数据列表
    pub items: Option<Vec<T>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 总数
    pub total: Option<i32>,
}
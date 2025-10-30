// baike模块的数据模型定义
use serde::{Deserialize, Serialize};
/// 词条信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
}
    /// 词条ID
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    /// 词条名称
    pub name: String,
    /// 词条描述
    pub description: Option<String>,
    /// 词条内容
    pub content: Option<String>,
    /// 词条状态
    #[serde(rename = "entity_status")]
    pub entity_status: EntityStatus,
    /// 词条类型
    #[serde(rename = "entity_type")]
    pub entity_type: EntityType,
    /// 分类列表
    pub categories: Option<Vec<Category>>,
    /// 创建者信息
    pub creator: Option<CreatorInfo>,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: Option<String>,
    /// 审核状态
    #[serde(rename = "audit_status")]
    pub audit_status: Option<AuditStatus>,
    /// 审核人信息
    #[serde(rename = "auditor")]
    pub auditor: Option<CreatorInfo>,
    /// 审核时间
    #[serde(rename = "audit_time")]
    pub audit_time: Option<String>,
    /// 标签列表
    pub tags: Option<Vec<String>>,
    /// 关联词条
    #[serde(rename = "related_entities")]
    pub related_entities: Option<Vec<Entity>>,
/// 词条状态
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityStatus {
    /// 草稿
    Draft,
    /// 已发布
    Published,
    /// 已下线
    Offline,
    /// 已删除
    Deleted,
/// 词条类型
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    /// 普通词条
    Normal,
    /// 人物词条
    Person,
    /// 组织词条
    Organization,
    /// 产品词条
    Product,
    /// 事件词条
    Event,
/// 分类信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
}
    /// 分类ID
    #[serde(rename = "category_id")]
    pub category_id: String,
    /// 分类名称
    pub name: String,
    /// 分类描述
    pub description: Option<String>,
    /// 父分类ID
    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,
    /// 分类层级
    pub level: i32,
/// 创建者信息
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatorInfo {
}
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名
    #[serde(rename = "user_name")]
    pub user_name: Option<String>,
    /// 用户类型
    #[serde(rename = "user_type")]
    pub user_type: Option<String>,
/// 审核状态
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditStatus {
    /// 待审核
    Pending,
    /// 审核通过
    Approved,
    /// 审核拒绝
    Rejected,
/// 草稿信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Draft {
}
    /// 草稿ID
    #[serde(rename = "draft_id")]
    pub draft_id: String,
    /// 词条ID
    #[serde(rename = "entity_id")]
    pub entity_id: Option<String>,
    /// 草稿内容
    pub content: String,
    /// 草稿类型
    #[serde(rename = "draft_type")]
    pub draft_type: DraftType,
    /// 创建者
    pub creator: CreatorInfo,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: String,
/// 草稿类型
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftType {
    /// 新建
    Create,
    /// 更新
    Update,
    /// 删除
    Delete,
/// 搜索匹配结果
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
}
    /// 词条信息
    pub entity: Entity,
    /// 匹配类型
    #[serde(rename = "match_type")]
    pub match_type: MatchType,
    /// 匹配分数
    pub score: f32,
/// 匹配类型
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchType {
    /// 精确匹配
    Exact,
    /// 模糊匹配
    Fuzzy,
    /// 前缀匹配
    Prefix,
    /// 后缀匹配
    Suffix,
/// 通用响应结构
#[derive(Debug, Deserialize, Serialize)]
pub struct BaikeResponse<T> {
}
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
/// 分页响应结构
#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedBaikeResponse<T> {
}
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<PaginatedBaikeData<T>>,
/// 分页数据
#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedBaikeData<T> {
    /// 数据列表
    pub items: Option<Vec<T>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 总数
    pub total: Option<i32>,
}
}}}}}
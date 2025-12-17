/// 知识库公共数据模型
///
/// 包含baike和lingo服务共用的数据结构定义
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 词条实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// 词条ID
    pub entity_id: String,
    /// 词条名称
    pub name: String,
    /// 词条别名列表
    pub aliases: Vec<String>,
    /// 词条分类
    pub classifications: Vec<Classification>,
    /// 词条释义
    pub definition: String,
    /// 词条状态
    pub status: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 创建者
    pub creator: String,
    /// 词典ID
    pub repo_id: String,
    /// 词条封面图片
    pub cover: Option<EntityCover>,
    /// 词条扩展属性
    pub extra: Option<HashMap<String, serde_json::Value>>,
}

/// 词条封面图片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityCover {
    /// 图片token
    pub file_token: String,
    /// 图片URL
    pub url: String,
}

/// 分类信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classification {
    /// 分类ID
    pub classification_id: String,
    /// 分类名称
    pub name: String,
    /// 父分类ID
    pub parent_id: Option<String>,
    /// 分类层级
    pub level: i32,
}

/// 词典信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    /// 词典ID
    pub repo_id: String,
    /// 词典名称
    pub name: String,
    /// 词典描述
    pub description: Option<String>,
    /// 词条数量
    pub entity_count: i32,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
}

/// 草稿信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Draft {
    /// 草稿ID
    pub draft_id: String,
    /// 草稿标题
    pub title: String,
    /// 草稿内容
    pub content: String,
    /// 草稿状态
    pub status: String,
    /// 操作类型：create/update
    pub operation_type: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 创建者
    pub creator: String,
    /// 审批状态
    pub approval_status: String,
}

/// 词条搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySearchResult {
    /// 词条ID
    pub entity_id: String,
    /// 词条名称
    pub name: String,
    /// 词条别名
    pub aliases: Vec<String>,
    /// 词条释义
    pub definition: String,
    /// 匹配分数
    pub score: f64,
    /// 高亮信息
    pub highlights: Option<Vec<HighlightInfo>>,
}

/// 高亮信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightInfo {
    /// 字段名
    pub field: String,
    /// 高亮片段
    pub fragments: Vec<String>,
    /// 开始位置
    pub start_position: i32,
    /// 结束位置
    pub end_position: i32,
}

/// 词条匹配结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMatchResult {
    /// 词条ID
    pub entity_id: String,
    /// 词条名称
    pub name: String,
    /// 匹配类型：name/alias
    pub match_type: String,
}

/// 词条提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityExtractResult {
    /// 提取的词条
    pub entity: String,
    /// 建议的别名
    pub suggested_aliases: Vec<String>,
    /// 置信度
    pub confidence: f64,
    /// 开始位置
    pub start_position: i32,
    /// 结束位置
    pub end_position: i32,
}

/// 文件上传请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileUploadRequest {
    /// 文件名
    pub file_name: String,
    /// 文件内容（base64编码）
    pub file_content: String,
    /// 文件大小
    pub file_size: i64,
    /// 文件类型
    pub content_type: String,
}

/// 文件上传结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResult {
    /// 文件token
    pub file_token: String,
    /// 文件URL
    pub url: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小
    pub file_size: i64,
    /// 文件类型
    pub content_type: String,
}

/// 分页响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 页码
    pub page: i32,
    /// 每页大小
    pub page_size: i32,
    /// 总数
    pub total: i32,
    /// 是否有下一页
    pub has_next: bool,
}

/// API响应包装结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 响应数据
    pub data: Option<T>,
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 请求ID
    pub request_id: Option<String>,
}

/// 词条类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    /// 普通词条
    Normal,
    /// 专业词条
    Professional,
    /// 自定义词条
    Custom,
}

/// 草稿状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DraftStatus {
    /// 编辑中
    Editing,
    /// 待审批
    Pending,
    /// 已批准
    Approved,
    /// 已拒绝
    Rejected,
}

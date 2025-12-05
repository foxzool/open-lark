//! Cloud Docs API 响应类型定义

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::models::*;

// ==================== 文档响应类型 ====================

/// 文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 文档数据
    pub data: Option<Document>,
}

/// 文档列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentListResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 文档列表数据
    pub data: Option<PageResponse<Document>>,
}

/// 批量文档操作响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDocumentResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 批量操作结果
    pub data: Option<BatchOperationResult>,
}

/// 批量操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResult {
    /// 成功数量
    pub success_count: i32,
    /// 失败数量
    pub failure_count: i32,
    /// 成功项目ID列表
    pub success_items: Vec<String>,
    /// 失败项目详情
    pub failure_items: Vec<BatchFailureItem>,
}

/// 批量操作失败项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFailureItem {
    /// 项目ID
    pub item_id: String,
    /// 错误代码
    pub error_code: i32,
    /// 错误消息
    pub error_message: String,
}

// ==================== 文件夹响应类型 ====================

/// 文件夹响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 文件夹数据
    pub data: Option<Folder>,
}

/// 文件夹内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderContentsResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 文件夹内容数据
    pub data: Option<PageResponse<FolderContent>>,
}

/// 文件夹内容项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderContent {
    /// 内容ID
    pub id: String,
    /// 内容名称
    pub name: String,
    /// 内容类型
    pub content_type: FolderContentType,
    /// 所有者ID
    pub owner_id: String,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 文件大小（文档时使用）
    pub size: Option<i64>,
    /// 文档类型（文档时使用）
    pub document_type: Option<DocumentType>,
}

// ==================== 评论响应类型 ====================

/// 评论响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 评论数据
    pub data: Option<Comment>,
}

/// 评论列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentListResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 评论列表数据
    pub data: Option<PageResponse<Comment>>,
}

/// 评论反应响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReactionResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 反应数据
    pub data: Option<CommentReaction>,
}

// ==================== 版本控制响应类型 ====================

/// 版本列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionListResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 版本列表数据
    pub data: Option<PageResponse<DocumentVersion>>,
}

/// 版本响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 版本数据
    pub data: Option<DocumentVersion>,
}

/// 版本比较响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionCompareResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 版本比较数据
    pub data: Option<VersionDiff>,
}

/// 版本差异
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDiff {
    /// 源版本号
    pub from_version: i32,
    /// 目标版本号
    pub to_version: i32,
    /// 差异类型
    pub diff_type: String,
    /// 差异内容
    pub diff_content: String,
    /// 变更行数
    pub changed_lines: i32,
    /// 添加行数
    pub added_lines: i32,
    /// 删除行数
    pub deleted_lines: i32,
}

// ==================== 模板响应类型 ====================

/// 模板响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 模板数据
    pub data: Option<DocumentTemplate>,
}

/// 模板列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateListResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 模板列表数据
    pub data: Option<PageResponse<DocumentTemplate>>,
}

// ==================== 搜索响应类型 ====================

/// 搜索响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 搜索结果数据
    pub data: Option<SearchResult>,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// 搜索结果项
    pub items: Vec<SearchItem>,
    /// 总数
    pub total: i32,
    /// 搜索用时
    pub search_time: f64,
    /// 搜索建议
    pub suggestions: Option<Vec<String>>,
}

/// 搜索结果项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchItem {
    /// 文档ID
    pub document_id: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub document_type: DocumentType,
    /// 所有者ID
    pub owner_id: String,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 摘要
    pub snippet: Option<String>,
    /// 高亮片段
    pub highlights: Option<Vec<String>>,
    /// 相关度评分
    pub score: f64,
}

/// 搜索建议响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestionResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 建议列表
    pub data: Option<Vec<String>>,
}

/// 推荐响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 推荐结果
    pub data: Option<RecommendResult>,
}

/// 推荐结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendResult {
    /// 推荐项目
    pub items: Vec<RecommendItem>,
    /// 推荐类型
    pub recommend_type: RecommendType,
    /// 推荐原因
    pub reason: String,
}

/// 推荐项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendItem {
    /// 文档ID
    pub document_id: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub document_type: DocumentType,
    /// 所有者ID
    pub owner_id: String,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 推荐评分
    pub score: f64,
    /// 推荐原因
    pub reason: Option<String>,
}

/// 搜索历史响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistoryResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 搜索历史数据
    pub data: Option<Vec<SearchHistoryItem>>,
}

/// 搜索历史项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistoryItem {
    /// 搜索关键词
    pub keyword: String,
    /// 搜索时间
    pub search_time: DateTime<Utc>,
    /// 搜索结果数量
    pub result_count: i32,
}

// ==================== 权限管理响应类型 ====================

/// 权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionsResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 权限数据
    pub data: Option<DocumentPermissions>,
}

/// 协作者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaboratorResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 协作者数据
    pub data: Option<Collaborator>,
}

// ==================== 通用响应类型 ====================

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
    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

/// 空响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
}

// 实现Default trait
impl Default for DocumentResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }
}

impl Default for DocumentListResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }
}

impl Default for FolderResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }
}

impl Default for CommentResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }
}

impl Default for VersionResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }
}

impl Default for TemplateResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }
}

impl Default for SearchResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }
}

impl Default for PermissionsResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }
}

impl Default for EmptyResponse {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
        }
    }
}

impl<T> Default for PageResponse<T> {
    fn default() -> Self {
        Self {
            items: vec![],
            page_token: None,
            has_more: Some(false),
            total: Some(0),
        }
    }
}

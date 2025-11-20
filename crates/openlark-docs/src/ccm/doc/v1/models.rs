//! Cloud Docs云文档服务数据模型
//!
//! 定义企业文档管理的核心数据结构，包括：
//! - 文档基础信息管理
//! - 文档版本控制
//! - 权限管理
//! - 评论和协作
//! - 文档模板
//! - 文档搜索

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ==================== 基础数据类型 ====================

/// 文档类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    /// 文本文档
    Doc,
    /// 表格
    Sheet,
    /// 幻灯片
    Slide,
    /// 思维导图
    Mindnote,
    /// 白板
    Bitable,
    /// 流程图
    Docx,
    /// 其他
    Other,
}

/// 文档状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentStatus {
    /// 草稿
    Draft,
    /// 已发布
    Published,
    /// 已归档
    Archived,
    /// 已删除
    Deleted,
}

/// 权限类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionType {
    /// 只读
    View,
    /// 可编辑
    Edit,
    /// 管理员
    Admin,
    /// 所有者
    Owner,
}

/// 评论类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentType {
    /// 普通评论
    Normal,
    /// 建议
    Suggestion,
    /// 问题
    Question,
    /// 解决方案
    Solution,
}

// ==================== 文档基础信息 ====================

/// 文档基础信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Document {
    /// 文档ID
    pub document_id: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub document_type: DocumentType,
    /// 文档状态
    pub status: DocumentStatus,
    /// 文档所有者ID
    pub owner_id: String,
    /// 文档所有者名称
    pub owner_name: Option<String>,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 最后修改时间
    pub update_time: DateTime<Utc>,
    /// 文档版本号
    pub version: i32,
    /// 文档大小（字节）
    pub size: i64,
    /// 文档描述
    pub description: Option<String>,
    /// 文档标签
    pub tags: Option<Vec<String>>,
    /// 文档封面图片URL
    pub cover_url: Option<String>,
    /// 文档预览URL
    pub preview_url: Option<String>,
    /// 文档分享链接
    pub share_url: Option<String>,
    /// 文档阅读次数
    pub view_count: i32,
    /// 文档编辑次数
    pub edit_count: i32,
    /// 文档评论次数
    pub comment_count: i32,
    /// 文档收藏次数
    favorite_count: i32,
    /// 文档所属文件夹ID
    pub folder_id: Option<String>,
    /// 文档权限
    pub permissions: Option<DocumentPermissions>,
    /// 文档协作者列表
    pub collaborators: Option<Vec<Collaborator>>,
    /// 文档扩展属性
    pub extra_properties: Option<serde_json::Value>,
}

/// 文档权限
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentPermissions {
    /// 是否公开
    pub is_public: bool,
    /// 权限列表
    pub permissions: Vec<DocumentPermission>,
    /// 继承权限
    pub inherit_permissions: bool,
}

/// 文档权限条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentPermission {
    /// 用户/用户组ID
    pub principal_id: String,
    /// 用户/用户组类型
    pub principal_type: PrincipalType,
    /// 权限类型
    pub permission_type: PermissionType,
    /// 权限过期时间
    pub expire_time: Option<DateTime<Utc>>,
}

/// 用户/用户组类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrincipalType {
    /// 用户
    User,
    /// 用户组
    Group,
    /// 部门
    Department,
    /// 组织
    Organization,
}

/// 协作者信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Collaborator {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户头像
    pub avatar: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 权限类型
    pub permission: PermissionType,
    /// 加入时间
    pub join_time: DateTime<Utc>,
    /// 最后活跃时间
    pub last_active_time: Option<DateTime<Utc>>,
}

// ==================== 文档版本管理 ====================

/// 文档版本
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentVersion {
    /// 版本ID
    pub version_id: String,
    /// 文档ID
    pub document_id: String,
    /// 版本号
    pub version: i32,
    /// 版本标题
    pub title: String,
    /// 版本描述
    pub description: Option<String>,
    /// 创建者ID
    pub creator_id: String,
    /// 创建者名称
    pub creator_name: String,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 版本大小
    pub size: i64,
    /// 版本差异
    pub diff: Option<String>,
    /// 版本标签
    pub tags: Option<Vec<String>>,
    /// 是否为主要版本
    pub is_major: bool,
    /// 版本状态
    pub status: VersionStatus,
    /// 版本备注
    pub notes: Option<String>,
}

/// 版本状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionStatus {
    /// 进行中
    InProgress,
    /// 已完成
    Completed,
    /// 已废弃
    Deprecated,
}

// ==================== 文件夹管理 ====================

/// 文件夹
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Folder {
    /// 文件夹ID
    pub folder_id: String,
    /// 文件夹名称
    pub name: String,
    /// 父文件夹ID
    pub parent_folder_id: Option<String>,
    /// 文件夹所有者ID
    pub owner_id: String,
    /// 文件夹所有者名称
    pub owner_name: Option<String>,
    /// 文件夹描述
    pub description: Option<String>,
    /// 文件夹图标
    pub icon: Option<String>,
    /// 文件夹颜色
    pub color: Option<String>,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 文件夹中的文档数量
    pub document_count: i32,
    /// 子文件夹数量
    pub subfolder_count: i32,
    /// 文件夹权限
    pub permissions: Option<FolderPermissions>,
    /// 文件夹排序
    pub order: Option<i32>,
    /// 文件夹状态
    pub status: FolderStatus,
}

/// 文件夹权限
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FolderPermissions {
    /// 是否公开
    pub is_public: bool,
    /// 权限列表
    pub permissions: Vec<FolderPermission>,
    /// 继承权限
    pub inherit_permissions: bool,
}

/// 文件夹权限条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderPermission {
    /// 用户/用户组ID
    pub principal_id: String,
    /// 用户/用户组类型
    pub principal_type: PrincipalType,
    /// 权限类型
    pub permission_type: FolderPermissionType,
    /// 权限过期时间
    pub expire_time: Option<DateTime<Utc>>,
}

/// 文件夹权限类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FolderPermissionType {
    /// 只读
    View,
    /// 可编辑
    Edit,
    /// 管理员
    Admin,
    /// 创建者
    Creator,
}

/// 文件夹状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FolderStatus {
    /// 正常
    Active,
    /// 已归档
    Archived,
    /// 已删除
    Deleted,
}

// ==================== 评论和协作 ====================

/// 文档评论
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Comment {
    /// 评论ID
    pub comment_id: String,
    /// 文档ID
    pub document_id: String,
    /// 评论者ID
    pub user_id: String,
    /// 评论者名称
    pub user_name: String,
    /// 评论者头像
    pub avatar: Option<String>,
    /// 评论内容
    pub content: String,
    /// 评论类型
    pub comment_type: CommentType,
    /// 父评论ID（回复时使用）
    pub parent_comment_id: Option<String>,
    /// 评论位置（文档中的位置）
    pub position: Option<CommentPosition>,
    /// 评论被引用次数
    pub reference_count: i32,
    /// 评论点赞数
    pub like_count: i32,
    /// 评论是否已解决
    pub is_resolved: bool,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 评论回复列表
    pub replies: Option<Vec<Comment>>,
}

/// 评论位置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommentPosition {
    /// 页码
    pub page_number: Option<i32>,
    /// 位置X坐标
    pub x: Option<f64>,
    /// 位置Y坐标
    pub y: Option<f64>,
    /// 选中范围
    pub selection: Option<TextSelection>,
}

/// 文本选择范围
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextSelection {
    /// 起始位置
    pub start: TextPosition,
    /// 结束位置
    pub end: TextPosition,
    /// 选中内容
    pub selected_text: Option<String>,
}

/// 文本位置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextPosition {
    /// 段落索引
    pub paragraph_index: i32,
    /// 字符偏移
    pub offset: i32,
    /// 行号
    pub line: i32,
    /// 列号
    pub column: i32,
}

// ==================== 文档模板 ====================

/// 文档模板
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentTemplate {
    /// 模板ID
    pub template_id: String,
    /// 模板名称
    pub name: String,
    /// 模板描述
    pub description: Option<String>,
    /// 模板类型
    pub template_type: DocumentType,
    /// 模板分类
    pub category: Option<String>,
    /// 模板标签
    pub tags: Option<Vec<String>>,
    /// 模板封面图片URL
    pub cover_url: Option<String>,
    /// 模板预览URL
    pub preview_url: Option<String>,
    /// 模板创建者ID
    pub creator_id: String,
    /// 模板创建者名称
    pub creator_name: String,
    /// 是否公开模板
    pub is_public: bool,
    /// 模板使用次数
    pub usage_count: i32,
    /// 模板评分
    pub rating: Option<f64>,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 模板内容
    pub content: Option<serde_json::Value>,
}

// ==================== 搜索和推荐 ====================

/// 文档搜索结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentSearchResult {
    /// 文档ID
    pub document_id: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub document_type: DocumentType,
    /// 文档状态
    pub status: DocumentStatus,
    /// 文档所有者ID
    pub owner_id: String,
    /// 文档所有者名称
    pub owner_name: Option<String>,
    /// 文档描述
    pub description: Option<String>,
    /// 文档标签
    pub tags: Option<Vec<String>>,
    /// 创建时间
    pub create_time: DateTime<Utc>,
    /// 更新时间
    pub update_time: DateTime<Utc>,
    /// 文档大小
    pub size: i64,
    /// 搜索相关性得分
    pub relevance_score: f64,
    /// 搜索高亮内容
    pub highlights: Option<Vec<SearchHighlight>>,
}

/// 搜索高亮
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchHighlight {
    /// 字段名称
    pub field: String,
    /// 高亮片段
    pub fragments: Vec<String>,
    /// 高亮开始位置
    pub ranges: Vec<HighlightRange>,
}

/// 高亮范围
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighlightRange {
    /// 开始位置
    pub start: i32,
    /// 结束位置
    pub end: i32,
}

/// 文档推荐
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRecommendation {
    /// 文档ID
    pub document_id: String,
    /// 文档标题
    pub title: String,
    /// 推荐原因
    pub reason: String,
    /// 推荐类型
    pub recommendation_type: RecommendationType,
    /// 推荐得分
    pub score: f64,
    /// 推荐时间
    pub recommend_time: DateTime<Utc>,
}

/// 推荐类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationType {
    /// 协作推荐
    Collaboration,
    /// 标签推荐
    Tag,
    /// 搜索历史推荐
    SearchHistory,
    /// 热门推荐
    Popular,
    /// 相似文档推荐
    Similar,
}

// ==================== 请求响应模型 ====================

/// 创建文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRequest {
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub document_type: DocumentType,
    /// 文档内容
    pub content: Option<serde_json::Value>,
    /// 文档描述
    pub description: Option<String>,
    /// 文档标签
    pub tags: Option<Vec<String>>,
    /// 所属文件夹ID
    pub folder_id: Option<String>,
    /// 权限设置
    pub permissions: Option<DocumentPermissions>,
    /// 协作者列表
    pub collaborators: Option<Vec<Collaborator>>,
}

/// 更新文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentRequest {
    /// 文档ID
    pub document_id: String,
    /// 文档标题
    pub title: Option<String>,
    /// 文档描述
    pub description: Option<String>,
    /// 文档标签
    pub tags: Option<Vec<String>>,
    /// 文档状态
    pub status: Option<DocumentStatus>,
    /// 所属文件夹ID
    pub folder_id: Option<String>,
}

/// 查询文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryDocumentsRequest {
    /// 文件夹ID筛选
    pub folder_id: Option<String>,
    /// 文档类型筛选
    pub document_type: Option<DocumentType>,
    /// 文档状态筛选
    pub status: Option<DocumentStatus>,
    /// 所有者ID筛选
    pub owner_id: Option<String>,
    /// 标签筛选
    pub tags: Option<Vec<String>>,
    /// 关键词搜索
    pub keyword: Option<String>,
    /// 排序方式
    pub sort_by: Option<SortBy>,
    /// 排序方向
    pub sort_direction: Option<SortDirection>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 排序字段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    /// 创建时间
    CreateTime,
    /// 更新时间
    UpdateTime,
    /// 标题
    Title,
    /// 大小
    Size,
    /// 阅读次数
    ViewCount,
    /// 编辑次数
    EditCount,
    /// 评论次数
    CommentCount,
    /// 收藏次数
    FavoriteCount,
}

/// 排序方向
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    /// 升序
    Asc,
    /// 降序
    Desc,
}

/// 搜索文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocumentsRequest {
    /// 搜索关键词
    pub query: String,
    /// 搜索范围
    pub search_scope: Option<SearchScope>,
    /// 文档类型筛选
    pub document_type: Option<DocumentType>,
    /// 文档状态筛选
    pub status: Option<DocumentStatus>,
    /// 所有者ID筛选
    pub owner_id: Option<String>,
    /// 标签筛选
    pub tags: Option<Vec<String>>,
    /// 搜索方式
    pub search_type: Option<SearchType>,
    /// 排序方式
    pub sort_by: Option<SortBy>,
    /// 排序方向
    pub sort_direction: Option<SortDirection>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 搜索范围
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchScope {
    /// 标题
    Title,
    /// 内容
    Content,
    /// 标签
    Tags,
    /// 描述
    Description,
    /// 作者
    Author,
    /// 全文
    FullText,
}

/// 搜索方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    /// 模糊搜索
    Fuzzy,
    /// 精确搜索
    Exact,
    /// 正则表达式
    Regex,
}

/// 创建评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    /// 文档ID
    pub document_id: String,
    /// 评论内容
    pub content: String,
    /// 评论类型
    pub comment_type: CommentType,
    /// 父评论ID（回复时使用）
    pub parent_comment_id: Option<String>,
    /// 评论位置
    pub position: Option<CommentPosition>,
    /// 提及用户
    pub mentions: Option<Vec<String>>,
}

/// 查询评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCommentsRequest {
    /// 文档ID
    pub document_id: String,
    /// 父评论ID筛选
    pub parent_comment_id: Option<String>,
    /// 评论类型筛选
    pub comment_type: Option<CommentType>,
    /// 是否已解决筛选
    pub is_resolved: Option<bool>,
    /// 排序方式
    pub sort_by: Option<SortBy>,
    /// 排序方向
    pub sort_direction: Option<SortDirection>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

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

/// 基础响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    /// 状态码
    pub code: i32,
    /// 消息
    pub msg: String,
    /// 数据
    pub data: Option<T>,
}

/// 文档列表响应
pub type DocumentListResponse = Response<PageResponse<Document>>;

/// 文档详情响应
pub type DocumentResponse = Response<Document>;

/// 文件夹列表响应
pub type FolderListResponse = Response<PageResponse<Folder>>;

/// 评论列表响应
pub type CommentListResponse = Response<PageResponse<Comment>>;

/// 搜索结果响应
pub type SearchResponse = Response<PageResponse<DocumentSearchResult>>;

/// 推荐文档响应
pub type RecommendResponse = Response<Vec<DocumentRecommendation>>;

/// 文档版本列表响应
pub type VersionListResponse = Response<PageResponse<DocumentVersion>>;

/// 模板列表响应
pub type TemplateListResponse = Response<PageResponse<DocumentTemplate>>;

/// 空响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {
    pub code: i32,
    pub msg: String,
}

// 实现Default trait
impl Default for DocumentType {
    fn default() -> Self {
        DocumentType::Doc
    }
}

impl Default for DocumentStatus {
    fn default() -> Self {
        DocumentStatus::Draft
    }
}

impl Default for PermissionType {
    fn default() -> Self {
        PermissionType::View
    }
}

impl Default for CommentType {
    fn default() -> Self {
        CommentType::Normal
    }
}

impl Default for PrincipalType {
    fn default() -> Self {
        PrincipalType::User
    }
}

impl Default for VersionStatus {
    fn default() -> Self {
        VersionStatus::InProgress
    }
}

impl Default for FolderStatus {
    fn default() -> Self {
        FolderStatus::Active
    }
}

impl Default for FolderPermissionType {
    fn default() -> Self {
        FolderPermissionType::View
    }
}

impl Default for SortBy {
    fn default() -> Self {
        SortBy::CreateTime
    }
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Desc
    }
}

impl Default for SearchScope {
    fn default() -> Self {
        SearchScope::FullText
    }
}

impl Default for SearchType {
    fn default() -> Self {
        SearchType::Fuzzy
    }
}

impl Default for RecommendationType {
    fn default() -> Self {
        RecommendationType::Collaboration
    }
}

impl Default for CreateDocumentRequest {
    fn default() -> Self {
        Self {
            title: String::new(),
            document_type: DocumentType::Doc,
            content: None,
            description: None,
            tags: None,
            folder_id: None,
            permissions: None,
            collaborators: None,
        }
    }
}

impl Default for UpdateDocumentRequest {
    fn default() -> Self {
        Self {
            document_id: String::new(),
            title: None,
            description: None,
            tags: None,
            status: None,
            folder_id: None,
        }
    }
}

impl Default for QueryDocumentsRequest {
    fn default() -> Self {
        Self {
            folder_id: None,
            document_type: None,
            status: None,
            owner_id: None,
            tags: None,
            keyword: None,
            sort_by: None,
            sort_direction: None,
            page_size: None,
            page_token: None,
        }
    }
}

impl Default for SearchDocumentsRequest {
    fn default() -> Self {
        Self {
            query: String::new(),
            search_scope: None,
            document_type: None,
            status: None,
            owner_id: None,
            tags: None,
            search_type: None,
            sort_by: None,
            sort_direction: None,
            page_size: None,
            page_token: None,
        }
    }
}

impl Default for CreateCommentRequest {
    fn default() -> Self {
        Self {
            document_id: String::new(),
            content: String::new(),
            comment_type: CommentType::Normal,
            parent_comment_id: None,
            position: None,
            mentions: None,
        }
    }
}

impl Default for QueryCommentsRequest {
    fn default() -> Self {
        Self {
            document_id: String::new(),
            parent_comment_id: None,
            comment_type: None,
            is_resolved: None,
            sort_by: None,
            sort_direction: None,
            page_size: None,
            page_token: None,
        }
    }
}

impl Default for PageResponse<Document> {
    fn default() -> Self {
        Self {
            items: vec![],
            page_token: None,
            has_more: Some(false),
            total: Some(0),
        }
    }
}

impl Default for PageResponse<Folder> {
    fn default() -> Self {
        Self {
            items: vec![],
            page_token: None,
            has_more: Some(false),
            total: Some(0),
        }
    }
}

impl Default for PageResponse<Comment> {
    fn default() -> Self {
        Self {
            items: vec![],
            page_token: None,
            has_more: Some(false),
            total: Some(0),
        }
    }
}

impl Default for PageResponse<DocumentSearchResult> {
    fn default() -> Self {
        Self {
            items: vec![],
            page_token: None,
            has_more: Some(false),
            total: Some(0),
        }
    }
}

impl Default for Response<Document> {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }
}

impl Default for Response<EmptyResponse> {
    fn default() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: Some(EmptyResponse::default()),
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

// ==================== 补充缺失的类型 ====================

/// 文件夹内容类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FolderContentType {
    /// 文件夹
    Folder,
    /// 文档
    Document,
    /// 全部
    All,
}

/// 评论反应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommentReaction {
    /// 反应ID
    pub reaction_id: String,
    /// 评论ID
    pub comment_id: String,
    /// 用户ID
    pub user_id: String,
    /// 反应类型
    pub reaction_type: String,
    /// 创建时间
    pub create_time: DateTime<Utc>,
}

/// 搜索日期范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDateRange {
    /// 开始日期
    pub start_date: DateTime<Utc>,
    /// 结束日期
    pub end_date: DateTime<Utc>,
}

/// 搜索排序方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchSortBy {
    /// 相关度
    Relevance,
    /// 创建时间
    CreateTime,
    /// 更新时间
    UpdateTime,
    /// 标题
    Title,
}

/// 推荐类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendType {
    /// 协作推荐
    Collaboration,
    /// 热门文档
    Popular,
    /// 相关推荐
    Related,
    /// 最近访问
    Recent,
}

/// 模板参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemplateParameter {
    /// 参数名称
    pub name: String,
    /// 参数类型
    pub param_type: String,
    /// 参数描述
    pub description: Option<String>,
    /// 是否必填
    pub required: bool,
    /// 默认值
    pub default_value: Option<String>,
}

/// 批量操作失败项
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchFailureItem {
    /// 项目ID
    pub item_id: String,
    /// 错误代码
    pub error_code: i32,
    /// 错误消息
    pub error_message: String,
}

/// 批量操作结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// 版本差异
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// 搜索历史项
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchHistoryItem {
    /// 搜索关键词
    pub keyword: String,
    /// 搜索时间
    pub search_time: DateTime<Utc>,
    /// 搜索结果数量
    pub result_count: i32,
}

/// 推荐结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecommendResult {
    /// 推荐项目
    pub items: Vec<RecommendItem>,
    /// 推荐类型
    pub recommend_type: RecommendType,
    /// 推荐原因
    pub reason: String,
}

/// 推荐项目
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// 文件夹内容项
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

// Default实现
impl Default for FolderContentType {
    fn default() -> Self {
        FolderContentType::All
    }
}

impl Default for SearchSortBy {
    fn default() -> Self {
        SearchSortBy::Relevance
    }
}

impl Default for RecommendType {
    fn default() -> Self {
        RecommendType::Collaboration
    }
}

//! Cloud Docs API 请求类型定义

use std::collections::HashMap;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

use super::models::*;

// ==================== 文档请求类型 ====================

/// 创建文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRequest {
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub document_type: DocumentType,
    /// 文档内容
    pub content: Option<String>,
    /// 所属文件夹ID
    pub folder_id: Option<String>,
    /// 模板ID
    pub template_id: Option<String>,
    /// 标签列表
    pub tags: Option<Vec<String>>,
    /// 权限设置
    pub permissions: Option<DocumentPermissions>,
    /// 协作者列表
    pub collaborators: Option<Vec<Collaborator>>,
}

/// 获取文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRequest {
    /// 文档ID
    pub document_id: String,
    /// 是否包含内容
    pub include_content: Option<bool>,
    /// 是否包含评论
    pub include_comments: Option<bool>,
    /// 是否包含协作者
    pub include_collaborators: Option<bool>,
}

/// 更新文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentRequest {
    /// 文档ID
    pub document_id: String,
    /// 文档标题
    pub title: Option<String>,
    /// 文档内容
    pub content: Option<String>,
    /// 标签列表
    pub tags: Option<Vec<String>>,
}

/// 删除文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentRequest {
    /// 文档ID
    pub document_id: String,
}

/// 复制文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyDocumentRequest {
    /// 文档ID
    pub document_id: String,
    /// 新文档标题
    pub title: String,
    /// 目标文件夹ID
    pub folder_id: Option<String>,
    /// 是否包含评论
    pub include_comments: Option<bool>,
}

/// 文档列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDocumentsRequest {
    /// 文件夹ID
    pub folder_id: Option<String>,
    /// 文档类型
    pub document_type: Option<DocumentType>,
    /// 所有者ID
    pub owner_id: Option<String>,
    /// 文档状态
    pub status: Option<DocumentStatus>,
    /// 关键词搜索
    pub keyword: Option<String>,
    /// 标签筛选
    pub tags: Option<Vec<String>>,
    /// 排序字段
    pub sort_field: Option<String>,
    /// 排序方向
    pub sort_order: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 移动文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDocumentRequest {
    /// 文档ID
    pub document_id: String,
    /// 目标文件夹ID
    pub target_folder_id: String,
}

/// 批量文档操作请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDocumentRequest {
    /// 操作类型
    pub operation: String,
    /// 文档ID列表
    pub document_ids: Vec<String>,
    /// 目标文件夹ID（移动操作时使用）
    pub target_folder_id: Option<String>,
}

// ==================== 文件夹请求类型 ====================

/// 创建文件夹请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    /// 文件夹名称
    pub name: String,
    /// 父文件夹ID
    pub parent_folder_id: Option<String>,
    /// 文件夹描述
    pub description: Option<String>,
    /// 权限设置
    pub permissions: Option<DocumentPermissions>,
}

/// 获取文件夹请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderRequest {
    /// 文件夹ID
    pub folder_id: String,
}

/// 更新文件夹请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFolderRequest {
    /// 文件夹ID
    pub folder_id: String,
    /// 文件夹名称
    pub name: Option<String>,
    /// 文件夹描述
    pub description: Option<String>,
}

/// 删除文件夹请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFolderRequest {
    /// 文件夹ID
    pub folder_id: String,
}

/// 文件夹内容列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFolderContentsRequest {
    /// 文件夹ID
    pub folder_id: String,
    /// 内容类型
    pub content_type: Option<FolderContentType>,
    /// 关键词搜索
    pub keyword: Option<String>,
    /// 排序字段
    pub sort_field: Option<String>,
    /// 排序方向
    pub sort_order: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 移动文件夹请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFolderRequest {
    /// 文件夹ID
    pub folder_id: String,
    /// 目标父文件夹ID
    pub target_parent_id: String,
}

// ==================== 评论请求类型 ====================

/// 创建评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    /// 文档ID
    pub document_id: String,
    /// 评论内容
    pub content: String,
    /// 评论类型
    pub comment_type: CommentType,
    /// 评论位置
    pub position: Option<CommentPosition>,
    /// 父评论ID（回复时使用）
    pub parent_comment_id: Option<String>,
}

impl Default for CreateCommentRequest {
    fn default() -> Self {
        Self {
            document_id: String::new(),
            content: String::new(),
            comment_type: CommentType::Normal,
            position: None,
            parent_comment_id: None,
        }
    }
}

/// 评论列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCommentsRequest {
    /// 文档ID
    pub document_id: String,
    /// 评论类型
    pub comment_type: Option<CommentType>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 删除评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCommentRequest {
    /// 评论ID
    pub comment_id: String,
}

/// 回复评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyCommentRequest {
    /// 评论ID
    pub comment_id: String,
    /// 回复内容
    pub content: String,
}

/// 评论反应请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactCommentRequest {
    /// 评论ID
    pub comment_id: String,
    /// 反应类型
    pub reaction_type: String,
    /// 是否添加（true添加，false取消）
    pub add: bool,
}

// ==================== 版本控制请求类型 ====================

/// 版本列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListVersionsRequest {
    /// 文档ID
    pub document_id: String,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 获取版本请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionRequest {
    /// 文档ID
    pub document_id: String,
    /// 版本号
    pub version_number: i32,
}

/// 恢复版本请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreVersionRequest {
    /// 文档ID
    pub document_id: String,
    /// 版本号
    pub version_number: i32,
}

/// 比较版本请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompareVersionsRequest {
    /// 文档ID
    pub document_id: String,
    /// 源版本号
    pub from_version: i32,
    /// 目标版本号
    pub to_version: i32,
}

// ==================== 模板请求类型 ====================

/// 模板列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTemplatesRequest {
    /// 文档类型
    pub document_type: Option<DocumentType>,
    /// 模板分类
    pub category: Option<String>,
    /// 关键词搜索
    pub keyword: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 获取模板请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTemplateRequest {
    /// 模板ID
    pub template_id: String,
}

/// 基于模板创建文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFromTemplateRequest {
    /// 模板ID
    pub template_id: String,
    /// 文档标题
    pub title: String,
    /// 目标文件夹ID
    pub folder_id: Option<String>,
    /// 模板参数
    pub parameters: Option<HashMap<String, String>>,
}

/// 创建模板请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateRequest {
    /// 模板名称
    pub name: String,
    /// 文档类型
    pub document_type: DocumentType,
    /// 模板内容
    pub content: String,
    /// 模板描述
    pub description: Option<String>,
    /// 模板分类
    pub category: Option<String>,
    /// 模板参数定义
    pub parameters: Option<Vec<TemplateParameter>>,
}

// ==================== 搜索请求类型 ====================

/// 搜索文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocumentsRequest {
    /// 搜索查询
    pub query: String,
    /// 文档类型
    pub document_type: Option<DocumentType>,
    /// 文件夹ID
    pub folder_id: Option<String>,
    /// 所有者ID
    pub owner_id: Option<String>,
    /// 标签筛选
    pub tags: Option<Vec<String>>,
    /// 日期范围
    pub date_range: Option<SearchDateRange>,
    /// 排序方式
    pub sort_by: Option<SearchSortBy>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 搜索建议请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestionRequest {
    /// 搜索关键词
    pub keyword: String,
    /// 建议数量
    pub limit: Option<i32>,
}

/// 推荐文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendDocumentsRequest {
    /// 用户ID
    pub user_id: String,
    /// 推荐类型
    pub recommend_type: RecommendType,
    /// 推荐数量
    pub limit: Option<i32>,
}

/// 搜索历史请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistoryRequest {
    /// 用户ID
    pub user_id: String,
    /// 历史记录数量
    pub limit: Option<i32>,
}

// ==================== 权限管理请求类型 ====================

/// 获取权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionsRequest {
    /// 文档ID
    pub document_id: String,
}

/// 更新权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionsRequest {
    /// 文档ID
    pub document_id: String,
    /// 权限设置
    pub permissions: DocumentPermissions,
}

/// 添加协作者请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCollaboratorRequest {
    /// 文档ID
    pub document_id: String,
    /// 用户ID
    pub user_id: String,
    /// 权限类型
    pub permission_type: PermissionType,
    /// 用户ID类型
    pub user_id_type: Option<String>,
}

impl Default for AddCollaboratorRequest {
    fn default() -> Self {
        Self {
            document_id: String::new(),
            user_id: String::new(),
            permission_type: PermissionType::View,
            user_id_type: None,
        }
    }
}

/// 移除协作者请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveCollaboratorRequest {
    /// 文档ID
    pub document_id: String,
    /// 用户ID
    pub user_id: String,
}

/// 更新协作者请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCollaboratorRequest {
    /// 文档ID
    pub document_id: String,
    /// 用户ID
    pub user_id: String,
    /// 新权限类型
    pub permission_type: PermissionType,
}

// 实现Default trait
impl Default for CreateDocumentRequest {
    fn default() -> Self {
        Self {
            title: String::new(),
            document_type: DocumentType::Doc,
            content: None,
            folder_id: None,
            template_id: None,
            tags: None,
            permissions: None,
            collaborators: None,
        }
    }
}

impl Default for GetDocumentRequest {
    fn default() -> Self {
        Self {
            document_id: String::new(),
            include_content: Some(true),
            include_comments: Some(false),
            include_collaborators: Some(false),
        }
    }
}

impl Default for UpdateDocumentRequest {
    fn default() -> Self {
        Self {
            document_id: String::new(),
            title: None,
            content: None,
            tags: None,
        }
    }
}

impl Default for ListDocumentsRequest {
    fn default() -> Self {
        Self {
            folder_id: None,
            document_type: None,
            owner_id: None,
            status: None,
            keyword: None,
            tags: None,
            sort_field: Some("update_time".to_string()),
            sort_order: Some("desc".to_string()),
            page_size: Some(20),
            page_token: None,
        }
    }
}

impl ListDocumentsRequest {
    /// 创建新的请求实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件夹ID
    pub fn folder_id(mut self, folder_id: impl Into<String>) -> Self {
        self.folder_id = Some(folder_id.into());
        self
    }

    /// 设置文档类型
    pub fn document_type(mut self, document_type: DocumentType) -> Self {
        self.document_type = Some(document_type);
        self
    }

    /// 设置所有者ID
    pub fn owner_id(mut self, owner_id: impl Into<String>) -> Self {
        self.owner_id = Some(owner_id.into());
        self
    }

    /// 设置文档状态
    pub fn status(mut self, status: DocumentStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置搜索关键词
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword = Some(keyword.into());
        self
    }

    /// 设置标签筛选
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// 设置排序字段
    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.sort_field = Some(sort_field.into());
        self
    }

    /// 设置排序方向
    pub fn sort_order(mut self, sort_order: impl Into<String>) -> Self {
        self.sort_order = Some(sort_order.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置页面标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }
}

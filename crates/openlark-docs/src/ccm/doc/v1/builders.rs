//! Cloud Docs API 请求构建器
//!
//! 提供类型安全的构建器模式，用于构建API请求

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::models::*;
use super::requests::{
    AddCollaboratorRequest, CreateCommentRequest, CreateDocumentRequest, ListDocumentsRequest,
    UpdateDocumentRequest,
};

// ==================== 文档请求构建器 ====================

/// 创建文档请求构建器
#[derive(Debug, Clone, Default)]
pub struct CreateDocumentRequestBuilder {
    request: CreateDocumentRequest,
}

impl CreateDocumentRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: CreateDocumentRequest::default(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = title.into();
        self
    }

    pub fn document_type(mut self, document_type: DocumentType) -> Self {
        self.request.document_type = document_type;
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.request.content = Some(content.into());
        self
    }

    pub fn folder_id(mut self, folder_id: impl Into<String>) -> Self {
        self.request.folder_id = Some(folder_id.into());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.request.tags = Some(tags);
        self
    }

    pub fn permissions(mut self, permissions: DocumentPermissions) -> Self {
        self.request.permissions = Some(permissions);
        self
    }

    pub fn collaborators(mut self, collaborators: Vec<Collaborator>) -> Self {
        self.request.collaborators = Some(collaborators);
        self
    }

    pub fn build(self) -> CreateDocumentRequest {
        self.request
    }
}

/// 更新文档请求构建器
#[derive(Debug, Clone, Default)]
pub struct UpdateDocumentRequestBuilder {
    request: UpdateDocumentRequest,
}

impl UpdateDocumentRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: UpdateDocumentRequest::default(),
        }
    }

    pub fn document_id(mut self, document_id: impl Into<String>) -> Self {
        self.request.document_id = document_id.into();
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = Some(title.into());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.request.tags = Some(tags);
        self
    }

    pub fn build(self) -> UpdateDocumentRequest {
        self.request
    }
}

/// 文档列表请求构建器
#[derive(Debug, Clone, Default)]
pub struct ListDocumentsRequestBuilder {
    request: ListDocumentsRequest,
}

impl ListDocumentsRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: ListDocumentsRequest::default(),
        }
    }

    pub fn folder_id(mut self, folder_id: impl Into<String>) -> Self {
        self.request.folder_id = Some(folder_id.into());
        self
    }

    pub fn document_type(mut self, document_type: DocumentType) -> Self {
        self.request.document_type = Some(document_type);
        self
    }

    pub fn owner_id(mut self, owner_id: impl Into<String>) -> Self {
        self.request.owner_id = Some(owner_id.into());
        self
    }

    pub fn status(mut self, status: DocumentStatus) -> Self {
        self.request.status = Some(status);
        self
    }

    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.request.keyword = Some(keyword.into());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.request.tags = Some(tags);
        self
    }

    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.request.sort_field = Some(sort_field.into());
        self
    }

    pub fn sort_order(mut self, sort_order: impl Into<String>) -> Self {
        self.request.sort_order = Some(sort_order.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn build(self) -> ListDocumentsRequest {
        self.request
    }
}

/// 创建评论请求构建器
#[derive(Debug, Clone, Default)]
pub struct CreateCommentRequestBuilder {
    request: CreateCommentRequest,
}

impl CreateCommentRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: CreateCommentRequest::default(),
        }
    }

    pub fn document_id(mut self, document_id: impl Into<String>) -> Self {
        self.request.document_id = document_id.into();
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.request.content = content.into();
        self
    }

    pub fn comment_type(mut self, comment_type: CommentType) -> Self {
        self.request.comment_type = comment_type;
        self
    }

    pub fn position(mut self, position: CommentPosition) -> Self {
        self.request.position = Some(position);
        self
    }

    pub fn parent_comment_id(mut self, parent_comment_id: impl Into<String>) -> Self {
        self.request.parent_comment_id = Some(parent_comment_id.into());
        self
    }

    pub fn build(self) -> CreateCommentRequest {
        self.request
    }
}

/// 添加协作者请求构建器
#[derive(Debug, Clone, Default)]
pub struct AddCollaboratorRequestBuilder {
    request: AddCollaboratorRequest,
}

impl AddCollaboratorRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: AddCollaboratorRequest::default(),
        }
    }

    pub fn document_id(mut self, document_id: impl Into<String>) -> Self {
        self.request.document_id = document_id.into();
        self
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.request.user_id = user_id.into();
        self
    }

    pub fn permission_type(mut self, permission_type: PermissionType) -> Self {
        self.request.permission_type = permission_type;
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn build(self) -> AddCollaboratorRequest {
        self.request
    }
}

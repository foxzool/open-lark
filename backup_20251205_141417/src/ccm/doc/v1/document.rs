//! 文档管理服务
//!
//! 提供飞书协作文档的创建、查询、管理等基础功能，包括：
//! - 创建新文档
//! - 获取文档信息
//! - 删除文档
//! - 文档权限管理

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

// 导入请求类型
use super::requests::ListDocumentsRequest;

// 导入模型类型
use super::models::{DocumentStatus, DocumentType};

/// 文档信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    /// 文档ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// 文档标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文档URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 文档版本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    /// 所在文件夹信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 文档状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            document_id: None,
            title: None,
            url: None,
            version: None,
            create_time: None,
            update_time: None,
            creator: None,
            folder_token: None,
            status: None,
        }
    }
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Creator {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl Default for Creator {
    fn default() -> Self {
        Self {
            user_id: None,
            name: None,
            avatar: None,
        }
    }
}

/// 创建文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRequest {
    /// 文档标题
    pub title: String,
    /// 所在文件夹token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

/// 获取文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRequest {
    /// 文档ID
    pub document_id: String,
}

impl GetDocumentRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docx::v1::document::GetDocumentRequest;
    ///
    /// let request = GetDocumentRequest::new("doc_123");
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
        }
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 参数验证通过
    /// - `Err(String)`: 参数验证失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docx::v1::document::GetDocumentRequest;
    ///
    /// let request = GetDocumentRequest::new("doc_123");
    /// assert!(request.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }
        if self.document_id.len() > 200 {
            return Err("文档ID长度不能超过200个字符".to_string());
        }
        Ok(())
    }
}

impl CreateDocumentRequest {
    /// 创建新的请求实例
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            folder_token: None,
        }
    }

    /// 设置所在文件夹
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("标题不能为空".to_string());
        }
        if self.title.len() > 200 {
            return Err("标题长度不能超过200个字符".to_string());
        }
        Ok(())
    }
}

/// 创建文档响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDocumentResponse {
    /// 创建的文档信息
    pub document: Document,
}

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDocumentResponse {
    /// 文档信息
    pub document: Document,
}

impl ApiResponseTrait for GetDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentRequest {
    /// 文档ID
    pub document_id: String,
}

impl DeleteDocumentRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::DeleteDocumentRequest;
    ///
    /// let request = DeleteDocumentRequest::new("doc_123");
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
        }
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 参数验证通过
    /// - `Err(String)`: 参数验证失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::DeleteDocumentRequest;
    ///
    /// let request = DeleteDocumentRequest::new("doc_123");
    /// assert!(request.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }
        if self.document_id.len() > 200 {
            return Err("文档ID长度不能超过200个字符".to_string());
        }
        Ok(())
    }
}

/// 删除文档响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteDocumentResponse {
    /// 是否成功删除
    pub success: Option<bool>,
    /// 操作结果消息
    pub message: Option<String>,
}

impl ApiResponseTrait for DeleteDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档纯文本内容请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRawContentRequest {
    /// 文档ID
    pub document_id: String,
}

impl GetDocumentRawContentRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::GetDocumentRawContentRequest;
    ///
    /// let request = GetDocumentRawContentRequest::new("doc_123");
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
        }
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 参数验证通过
    /// - `Err(String)`: 参数验证失败，返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }
        if self.document_id.len() > 200 {
            return Err("文档ID长度不能超过200个字符".to_string());
        }
        Ok(())
    }
}

/// 获取文档纯文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDocumentRawContentResponse {
    /// 文档纯文本内容
    pub content: Option<String>,
    /// 文档版本号
    pub version: Option<i32>,
}

impl ApiResponseTrait for GetDocumentRawContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档所有块请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlocksRequest {
    /// 文档ID
    pub document_id: String,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
    /// 文档版本号
    pub document_revision_id: Option<i32>,
}

impl GetDocumentBlocksRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::GetDocumentBlocksRequest;
    ///
    /// let request = GetDocumentBlocksRequest::new("doc_123")
    ///     .page_size(20);
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
            page_size: None,
            page_token: None,
            document_revision_id: None,
        }
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

    /// 设置文档版本号
    pub fn document_revision_id(mut self, document_revision_id: i32) -> Self {
        self.document_revision_id = Some(document_revision_id);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }
        if self.document_id.len() > 200 {
            return Err("文档ID长度不能超过200个字符".to_string());
        }
        if let Some(page_size) = self.page_size {
            if page_size <= 0 || page_size > 200 {
                return Err("分页大小必须在1-200之间".to_string());
            }
        }
        Ok(())
    }
}

/// 文档块信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentBlock {
    /// 块ID
    pub block_id: Option<i64>,
    /// 块类型
    pub block_type: Option<i32>,
    /// 块内容
    pub block_content: Option<serde_json::Value>,
    /// 父块ID
    pub parent_block_id: Option<i64>,
    /// 块索引
    pub index: Option<i32>,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 更新时间
    pub update_time: Option<i64>,
}

impl Default for DocumentBlock {
    fn default() -> Self {
        Self {
            block_id: None,
            block_type: None,
            block_content: None,
            parent_block_id: None,
            index: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 获取文档所有块响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDocumentBlocksResponse {
    /// 块列表
    pub items: Option<Vec<DocumentBlock>>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 文档版本号
    pub document_revision_id: Option<i32>,
}

impl ApiResponseTrait for GetDocumentBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建文档块请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentBlockRequest {
    /// 文档ID
    pub document_id: String,
    /// 父块ID
    pub block_id: i64,
    /// 块索引位置
    pub index: i32,
    /// 块内容列表
    pub children: Vec<DocumentBlock>,
}

/// 文档块更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentBlockRequest {
    /// 文档ID
    pub document_id: String,
    /// 块ID
    pub block_id: i64,
    /// 块内容
    pub block: DocumentBlock,
    /// 是否为增量更新
    pub is_insecure: Option<bool>,
}

/// 文档块删除请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentBlockRequest {
    /// 文档ID
    pub document_id: String,
    /// 块ID
    pub block_id: i64,
    /// 块内的子元素ID（可选，如果提供则删除特定子元素）
    pub block_id_list: Option<Vec<i64>>,
}

impl CreateDocumentBlockRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    /// - `block_id`: 父块ID
    /// - `index`: 块索引位置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::CreateDocumentBlockRequest;
    ///
    /// let request = CreateDocumentBlockRequest::new("doc_123", 123456, 0)
    ///     .add_child(child_block);
    /// ```
    pub fn new(document_id: impl Into<String>, block_id: i64, index: i32) -> Self {
        Self {
            document_id: document_id.into(),
            block_id,
            index,
            children: vec![],
        }
    }

    /// 添加子块
    pub fn add_child(mut self, child: DocumentBlock) -> Self {
        self.children.push(child);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }
        if self.document_id.len() > 200 {
            return Err("文档ID长度不能超过200个字符".to_string());
        }
        if self.block_id <= 0 {
            return Err("父块ID必须大于0".to_string());
        }
        if self.index < 0 {
            return Err("块索引不能为负数".to_string());
        }
        if self.children.is_empty() {
            return Err("至少需要添加一个子块".to_string());
        }
        Ok(())
    }
}

/// 创建文档块响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDocumentBlockResponse {
    /// 创建的块列表
    pub items: Option<Vec<DocumentBlock>>,
    /// 文档版本号
    pub document_revision_id: Option<i32>,
}

impl ApiResponseTrait for CreateDocumentBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新文档块响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateDocumentBlockResponse {
    /// 更新的块信息
    pub block: Option<DocumentBlock>,
    /// 文档版本号
    pub document_revision_id: Option<i32>,
}

impl ApiResponseTrait for UpdateDocumentBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除文档块响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteDocumentBlockResponse {
    /// 删除是否成功
    pub success: Option<bool>,
    /// 文档版本号
    pub document_revision_id: Option<i32>,
}

impl ApiResponseTrait for DeleteDocumentBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文档列表查询响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentListResponse {
    /// 文档列表
    pub items: Option<Vec<Document>>,
    /// 分页信息
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for DocumentListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateDocumentBlockRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    /// - `block_id`: 块ID
    /// - `block`: 块内容
    ///
    /// # 返回
    /// 返回新的请求实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{UpdateDocumentBlockRequest, DocumentBlock};
    ///
    /// let block = DocumentBlock::new();
    /// let request = UpdateDocumentBlockRequest::new("doc_123", 123456, block)
    ///     .is_insecure(true);
    /// ```
    pub fn new(document_id: impl Into<String>, block_id: i64, block: DocumentBlock) -> Self {
        Self {
            document_id: document_id.into(),
            block_id,
            block,
            is_insecure: None,
        }
    }

    /// 设置是否为增量更新
    ///
    /// # 参数
    /// - `is_insecure`: 是否为增量更新
    ///
    /// # 返回
    /// 返回自身以支持链式调用
    pub fn is_insecure(mut self, is_insecure: bool) -> Self {
        self.is_insecure = Some(is_insecure);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }
        if self.document_id.len() > 200 {
            return Err("文档ID长度不能超过200个字符".to_string());
        }
        if self.block_id <= 0 {
            return Err("块ID必须大于0".to_string());
        }
        Ok(())
    }
}

impl DeleteDocumentBlockRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    /// - `block_id`: 块ID
    ///
    /// # 返回
    /// 返回新的请求实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::DeleteDocumentBlockRequest;
    ///
    /// let request = DeleteDocumentBlockRequest::new("doc_123", 123456)
    ///     .block_id_list(vec![789, 1011]);
    /// ```
    pub fn new(document_id: impl Into<String>, block_id: i64) -> Self {
        Self {
            document_id: document_id.into(),
            block_id,
            block_id_list: None,
        }
    }

    /// 设置要删除的子元素ID列表
    ///
    /// # 参数
    /// - `block_id_list`: 子元素ID列表
    ///
    /// # 返回
    /// 返回自身以支持链式调用
    pub fn block_id_list(mut self, block_id_list: Vec<i64>) -> Self {
        self.block_id_list = Some(block_id_list);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }
        if self.document_id.len() > 200 {
            return Err("文档ID长度不能超过200个字符".to_string());
        }
        if self.block_id <= 0 {
            return Err("块ID必须大于0".to_string());
        }
        Ok(())
    }
}

// ==================== 群公告块内容获取 API ====================

/// 获取群公告块内容请求
///
/// 用于获取指定群聊中特定公告块的内容信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnnouncementBlockContentRequest {
    /// 群聊ID
    pub chat_id: String,
    /// 公告块ID
    pub block_id: String,
    /// 用户ID类型
    ///
    /// 可选值：
    /// - `open_id`：用户的开放应用ID
    /// - `user_id`：用户的用户ID
    /// - `union_id`：用户的联合ID
    ///
    /// 默认值：`open_id`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetAnnouncementBlockContentRequest {
    /// 创建获取群公告块内容请求
    ///
    /// # 参数
    /// - `chat_id`: 群聊ID
    /// - `block_id`: 公告块ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docx::v1::document::GetAnnouncementBlockContentRequest;
    ///
    /// let request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456");
    /// ```
    pub fn new(chat_id: impl Into<String>, block_id: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            block_id: block_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::docx::v1::document::GetAnnouncementBlockContentRequest;
    /// let mut request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456");
    /// request.set_user_id_type("user_id");
    /// ```
    pub fn set_user_id_type(&mut self, user_id_type: impl Into<String>) -> &mut Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    ///
    /// # 返回
    /// - `Ok(())`: 验证通过
    /// - `Err(String)`: 验证失败，返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.chat_id.trim().is_empty() {
            return Err("群聊ID不能为空".to_string());
        }

        if self.block_id.trim().is_empty() {
            return Err("公告块ID不能为空".to_string());
        }

        if let Some(user_id_type) = &self.user_id_type {
            if user_id_type.trim().is_empty() {
                return Err("用户ID类型不能为空".to_string());
            }

            // 验证用户ID类型的有效性
            match user_id_type.as_str() {
                "open_id" | "user_id" | "union_id" => {}
                _ => return Err("用户ID类型必须是 open_id、user_id 或 union_id".to_string()),
            }
        }

        Ok(())
    }
}

/// 公告块内容信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnnouncementBlockContent {
    /// 公告块ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    /// 公告块类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
    /// 公告块内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 公告块标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
}

impl Default for AnnouncementBlockContent {
    fn default() -> Self {
        Self {
            block_id: None,
            block_type: None,
            content: None,
            title: None,
            create_time: None,
            update_time: None,
            creator: None,
        }
    }
}

/// 获取群公告块内容响应
///
/// 包含获取的群公告块内容信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnnouncementBlockContentResponse {
    /// 公告块内容信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_content: Option<AnnouncementBlockContent>,
    /// 是否成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 操作结果消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Default for GetAnnouncementBlockContentResponse {
    fn default() -> Self {
        Self {
            block_content: None,
            success: None,
            message: None,
        }
    }
}

impl ApiResponseTrait for GetAnnouncementBlockContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群公告块内容构建器
///
/// 提供流式API来构建获取群公告块内容的请求。
pub struct GetAnnouncementBlockContentBuilder {
    request: GetAnnouncementBlockContentRequest,
}

impl GetAnnouncementBlockContentBuilder {
    /// 创建获取群公告块内容构建器
    ///
    /// # 参数
    /// - `chat_id`: 群聊ID
    /// - `block_id`: 公告块ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docx::v1::document::GetAnnouncementBlockContentBuilder;
    ///
    /// let builder = GetAnnouncementBlockContentBuilder::new("chat_123", "block_456");
    /// ```
    pub fn new(chat_id: impl Into<String>, block_id: impl Into<String>) -> Self {
        Self {
            request: GetAnnouncementBlockContentRequest::new(chat_id, block_id),
        }
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::docx::v1::document::GetAnnouncementBlockContentBuilder;
    /// let builder = GetAnnouncementBlockContentBuilder::new("chat_123", "block_456")
    ///     .user_id_type("user_id");
    /// ```
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.set_user_id_type(user_id_type);
        self
    }

    /// 构建请求对象
    ///
    /// # 返回
    /// - `Ok(GetAnnouncementBlockContentRequest)`: 构建成功的请求对象
    /// - `Err(String)`: 构建失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::docx::v1::document::GetAnnouncementBlockContentBuilder;
    /// let request = GetAnnouncementBlockContentBuilder::new("chat_123", "block_456")
    ///     .user_id_type("open_id")
    ///     
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<GetAnnouncementBlockContentRequest, String> {
        self.request.validate()?;
        Ok(self.request)
    }

    /// 执行获取群公告块内容请求
    ///
    /// # 参数
    /// - `service`: 文档服务实例
    ///
    /// # 返回
    /// - `Ok(GetAnnouncementBlockContentResponse)`: 获取成功，返回公告块内容信息
    /// - `Err(SDKError)`: 获取失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::docx::v1::document::GetAnnouncementBlockContentBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret")?;
    /// let response = GetAnnouncementBlockContentBuilder::new("chat_123", "block_456")
    ///     .user_id_type("open_id")
    ///     .execute(&client.docx.v1.document)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute(
        self,
        service: &super::DocumentService,
    ) -> SDKResult<GetAnnouncementBlockContentResponse> {
        let request = self
            
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        service.get_announcement_block_content(&request).await
    }
}

impl Default for GetAnnouncementBlockContentBuilder {
    fn default() -> Self {
        Self {
            request: GetAnnouncementBlockContentRequest {
                chat_id: String::new(),
                block_id: String::new(),
                user_id_type: None,
            },
        }
    }
}

/// 文档管理服务
pub struct DocumentService {
    config: Config,
}

impl DocumentService {
    /// 创建文档管理服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::docx::v1::document::DocumentService;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = DocumentService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建文档
    ///
    /// 创建一个新的协作文档，支持指定标题和所在文件夹
    ///
    /// # 参数
    /// * `req` - 创建文档请求
    ///
    /// # 返回值
    /// 返回创建的文档信息
    pub async fn create(&self, req: &CreateDocumentRequest) -> SDKResult<CreateDocumentResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始创建文档: title={:?}", req.title);

        let api_req = ApiRequest {
            method: openlark_core::api::Post,
            url: openlark_core::endpoints::Endpoints::DOCX_V1_DOCUMENTS.to_string(),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Some(openlark_core::api::RequestData::Json(req))?,
            
        };

        let resp =
            Transport::<CreateDocumentResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档创建成功: title={}, document_id={:?}",
            req.title,
            response.document.document_id
        );

        Ok(response)
    }

    /// 获取群公告块内容
    ///
    /// 获取指定群聊中特定公告块的详细内容信息，包括
    /// 公告块类型、内容、标题、创建时间等。
    ///
    /// # 参数
    /// * `req` - 获取群公告块内容请求
    ///
    /// # 返回值
    /// 返回群公告块的内容信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docx::v1::document::{DocumentService, GetAnnouncementBlockContentRequest};
    ///
    /// let service = DocumentService::new(config);
    /// let request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456");
    ///
    /// let result = service.get_announcement_block_content(&request).await?;
    /// ```
    pub async fn get_announcement_block_content(
        &self,
        req: &GetAnnouncementBlockContentRequest,
    ) -> SDKResult<GetAnnouncementBlockContentResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!(
            "开始获取群公告块内容: chat_id={}, block_id={:?}",
            req.chat_id,
            req.block_id
        );

        // 构建API路径，替换两个路径参数
        let endpoint = format!(
            "/open-apis/docx/v1/chats/{}/announcement/blocks/{}",
            req.chat_id, req.block_id
        );

        let api_req = ApiRequest {
            method: openlark_core::api::Get,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None, // GET请求没有请求体
            
        };

        let resp =
            Transport::<GetAnnouncementBlockContentResponse>::request(api_req, &self.config, None)
                .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "群公告块内容获取完成: chat_id={}, block_id={}, success={:?}",
            req.chat_id,
            req.block_id,
            response.success
        );

        Ok(response)
    }

    /// 获取群公告块内容构建器
    ///
    /// 提供流式API来构建获取群公告块内容的请求。
    ///
    /// # 参数
    /// - `chat_id`: 群聊ID
    /// - `block_id`: 公告块ID
    ///
    /// # 返回
    /// 返回群公告块内容构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docx::v1::document::DocumentService;
    ///
    /// let service = DocumentService::new(config);
    /// let builder = service.get_announcement_block_content_builder("chat_123", "block_456");
    /// ```
    pub fn get_announcement_block_content_builder(
        &self,
        chat_id: impl Into<String>,
        block_id: impl Into<String>,
    ) -> GetAnnouncementBlockContentBuilder {
        GetAnnouncementBlockContentBuilder::new(chat_id, block_id)
    }

    /// 获取文档信息
    ///
    /// 获取指定文档的详细信息，包括标题、版本、创建者、
    /// 更新时间等元数据信息。
    ///
    /// # 参数
    /// * `req` - 获取文档请求
    ///
    /// # 返回值
    /// 返回文档的详细信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, GetDocumentRequest};
    ///
    /// let service = DocumentService::new(config);
    /// let request = GetDocumentRequest::new("doc_123");
    ///
    /// let result = service.get(&request).await?;
    /// println!("文档标题: {:?}", result.document.title);
    /// println!("文档版本: {:?}", result.document.version);
    /// ```
    pub async fn get(&self, req: &GetDocumentRequest) -> SDKResult<GetDocumentResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始获取文档信息: document_id={}", req.document_id);

        // 构建动态端点路径
        let endpoint = openlark_core::endpoints::Endpoints::DOCX_V1_DOCUMENT_GET
            .replace("{}", &req.document_id);

        let api_req = ApiRequest {
            method: openlark_core::api::Get,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None, // GET请求无body
            
        };

        let resp = Transport::<GetDocumentResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档信息获取完成: document_id={}, title={:?}",
            req.document_id,
            response.document.title
        );

        Ok(response)
    }

    /// 获取文档纯文本内容
    ///
    /// 获取指定文档的纯文本内容，不包含富文本格式。
    ///
    /// # 参数
    /// * `req` - 获取文档纯文本内容请求
    ///
    /// # 返回值
    /// 返回文档的纯文本内容和版本信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, GetDocumentRawContentRequest};
    ///
    /// let service = DocumentService::new(config);
    /// let request = GetDocumentRawContentRequest::new("doc_123");
    ///
    /// let result = service.get_raw_content(&request).await?;
    /// if let Some(content) = result.content {
    ///     println!("文档内容: {}", content);
    /// }
    /// ```
    pub async fn get_raw_content(
        &self,
        req: &GetDocumentRawContentRequest,
    ) -> SDKResult<GetDocumentRawContentResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始获取文档纯文本内容: document_id={}", req.document_id);

        // 构建动态端点路径
        let endpoint = openlark_core::endpoints::Endpoints::DOCX_V1_DOCUMENT_RAW_CONTENT
            .replace("{}", &req.document_id);

        let api_req = ApiRequest {
            method: openlark_core::api::Get,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None, // GET请求无body
            
        };

        let resp = Transport::<GetDocumentRawContentResponse>::request(api_req, &self.config, None)
            .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档纯文本内容获取完成: document_id={}, version={:?}",
            req.document_id,
            response.version
        );

        Ok(response)
    }

    /// 获取文档所有块
    ///
    /// 获取指定文档的所有块的富文本内容并分页返回。
    ///
    /// # 参数
    /// * `req` - 获取文档块请求
    ///
    /// # 返回值
    /// 返回文档块的列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, GetDocumentBlocksRequest};
    ///
    /// let service = DocumentService::new(config);
    /// let request = GetDocumentBlocksRequest::new("doc_123")
    ///     .page_size(20);
    ///
    /// let result = service.get_blocks(&request).await?;
    /// if let Some(blocks) = result.items {
    ///     println!("找到 {} 个块", blocks.len());
    /// }
    /// ```
    pub async fn get_blocks(
        &self,
        req: &GetDocumentBlocksRequest,
    ) -> SDKResult<GetDocumentBlocksResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始获取文档块: document_id={}", req.document_id);

        // 构建动态端点路径
        let endpoint = openlark_core::endpoints::Endpoints::DOCX_V1_DOCUMENT_BLOCKS
            .replace("{}", &req.document_id);

        let api_req = ApiRequest {
            method: openlark_core::api::Get,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None, // GET请求无body
            
        };

        let resp =
            Transport::<GetDocumentBlocksResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档块获取完成: document_id={}, block_count={:?}",
            req.document_id,
            response.items.as_ref().map(|blocks| blocks.len())
        );

        Ok(response)
    }

    /// 创建文档块
    ///
    /// 在指定块的子块列表中，新创建一批子块，并放置到指定位置。
    ///
    /// # 参数
    /// * `req` - 创建文档块请求
    ///
    /// # 返回值
    /// 返回创建的块信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, CreateDocumentBlockRequest, DocumentBlock};
    ///
    /// let service = DocumentService::new(config);
    /// let child_block = DocumentBlock { /* 设置块内容 */ };
    /// let request = CreateDocumentBlockRequest::new("doc_123", 123456, 0)
    ///     .add_child(child_block);
    ///
    /// let result = service.create_block(&request).await?;
    /// ```
    pub async fn create_block(
        &self,
        req: &CreateDocumentBlockRequest,
    ) -> SDKResult<CreateDocumentBlockResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!(
            "开始创建文档块: document_id={}, block_id={}",
            req.document_id,
            req.block_id
        );

        // 构建动态端点路径
        let endpoint = openlark_core::endpoints::Endpoints::DOCX_V1_DOCUMENT_BLOCK_CHILDREN
            .replace("{}", &req.document_id)
            .replace("{}", &req.block_id.to_string());

        let api_req = ApiRequest {
            method: openlark_core::api::Post,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Some(openlark_core::api::RequestData::Json(req))?,
            
        };

        let resp =
            Transport::<CreateDocumentBlockResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档块创建完成: document_id={}, block_count={:?}",
            req.document_id,
            response.items.as_ref().map(|blocks| blocks.len())
        );

        Ok(response)
    }

    /// 更新文档块
    ///
    /// 更新指定文档中的块内容，支持完整的块内容更新或增量更新。
    ///
    /// # 参数
    /// - `req`: 更新文档块请求
    ///
    /// # 返回
    /// 返回更新后的文档块信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, UpdateDocumentBlockRequest, DocumentBlock};
    ///
    /// let service = DocumentService::new(config);
    /// let block = DocumentBlock::new();
    /// let request = UpdateDocumentBlockRequest::new("doc_123", 123456, block);
    ///
    /// let result = service.update_block(&request).await?;
    /// if let Some(updated_block) = result.block {
    ///     println!("更新成功: {:?}", updated_block);
    /// }
    /// ```
    pub async fn update_block(
        &self,
        req: &UpdateDocumentBlockRequest,
    ) -> SDKResult<UpdateDocumentBlockResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!(
            "开始更新文档块: document_id={}, block_id={}",
            req.document_id,
            req.block_id
        );

        // 构建动态端点路径
        let endpoint = format!(
            "/open-apis/docx/v1/documents/{}/blocks/{}",
            req.document_id, req.block_id
        );

        let api_req = ApiRequest {
            method: openlark_core::api::Patch,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Some(openlark_core::api::RequestData::Json(req))?,
            
        };

        let resp =
            Transport::<UpdateDocumentBlockResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档块更新完成: document_id={}, block_id={}, revision_id={:?}",
            req.document_id,
            req.block_id,
            response.document_revision_id
        );

        Ok(response)
    }

    /// 删除文档块
    ///
    /// 删除指定文档中的块，支持删除整个块或块内的特定子元素。
    ///
    /// # 参数
    /// - `req`: 删除文档块请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, DeleteDocumentBlockRequest};
    ///
    /// let service = DocumentService::new(config);
    /// let request = DeleteDocumentBlockRequest::new("doc_123", 123456);
    ///
    /// let result = service.delete_block(&request).await?;
    /// if result.success.unwrap_or(false) {
    ///     println!("删除成功");
    /// }
    /// ```
    pub async fn delete_block(
        &self,
        req: &DeleteDocumentBlockRequest,
    ) -> SDKResult<DeleteDocumentBlockResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!(
            "开始删除文档块: document_id={}, block_id={}",
            req.document_id,
            req.block_id
        );

        // 构建动态端点路径
        let endpoint = format!(
            "/open-apis/docx/v1/documents/{}/blocks/{}",
            req.document_id, req.block_id
        );

        let api_req = ApiRequest {
            method: openlark_core::api::Delete,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Some(openlark_core::api::RequestData::Json(req))?,
            
        };

        let resp =
            Transport::<DeleteDocumentBlockResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档块删除完成: document_id={}, block_id={}, success={:?}",
            req.document_id,
            req.block_id,
            response.success
        );

        Ok(response)
    }

    /// 查询文档列表
    ///
    /// 根据多种条件查询文档列表，支持分页、排序、搜索等功能。
    ///
    /// # 参数
    /// - `req`: 文档列表查询请求
    ///
    /// # 返回
    /// 返回符合条件的文档列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, ListDocumentsRequest};
    ///
    /// let service = DocumentService::new(config);
    /// let request = ListDocumentsRequest::default()
    ///     .page_size(20)
    ///     .keyword("项目计划");
    ///
    /// let result = service.list_documents(&request).await?;
    /// if let Some(documents) = result.items {
    ///     println!("找到 {} 个文档", documents.len());
    /// }
    /// ```
    pub async fn list_documents(
        &self,
        req: &ListDocumentsRequest,
    ) -> SDKResult<DocumentListResponse> {
        log::debug!(
            "开始查询文档列表: folder_id={:?}, keyword={:?}",
            req.folder_id,
            req.keyword
        );

        // 构建查询参数

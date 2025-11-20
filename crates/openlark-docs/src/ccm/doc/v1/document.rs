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
#[derive(Clone, Debug)]
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
    ///     .build()
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
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
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
            .build()
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
#[derive(Clone, Debug)]
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
            method: openlark_core::api::HttpMethod::Post,
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
            method: openlark_core::api::HttpMethod::Get,
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
            method: openlark_core::api::HttpMethod::Get,
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
            method: openlark_core::api::HttpMethod::Get,
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
            method: openlark_core::api::HttpMethod::Get,
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
            method: openlark_core::api::HttpMethod::Post,
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
            method: openlark_core::api::HttpMethod::Patch,
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
            method: openlark_core::api::HttpMethod::Delete,
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
        let mut query_params = vec![];

        if let Some(folder_id) = &req.folder_id {
            query_params.push(("folder_id".to_string(), folder_id.clone()));
        }

        if let Some(document_type) = &req.document_type {
            query_params.push(("document_type".to_string(), format!("{:?}", document_type)));
        }

        if let Some(owner_id) = &req.owner_id {
            query_params.push(("owner_id".to_string(), owner_id.clone()));
        }

        if let Some(status) = &req.status {
            query_params.push(("status".to_string(), format!("{:?}", status)));
        }

        if let Some(keyword) = &req.keyword {
            query_params.push(("keyword".to_string(), keyword.clone()));
        }

        if let Some(sort_field) = &req.sort_field {
            query_params.push(("sort_field".to_string(), sort_field.clone()));
        }

        if let Some(sort_order) = &req.sort_order {
            query_params.push(("sort_order".to_string(), sort_order.clone()));
        }

        if let Some(page_size) = req.page_size {
            query_params.push(("page_size".to_string(), page_size.to_string()));
        }

        if let Some(page_token) = &req.page_token {
            query_params.push(("page_token".to_string(), page_token.clone()));
        }

        // 构建查询字符串
        let query_string = if !query_params.is_empty() {
            format!(
                "?{}",
                query_params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                    .collect::<Vec<_>>()
                    .join("&")
            )
        } else {
            String::new()
        };

        let endpoint = format!("/open-apis/docx/v1/documents{}", query_string);

        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Get,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None, // GET请求无body
            
        };

        let resp = Transport::<DocumentListResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档列表查询完成: found_count={:?}, has_more={:?}",
            response.items.as_ref().map(|docs| docs.len()),
            response.has_more
        );

        Ok(response)
    }

    /// 删除文档
    ///
    /// 删除指定的文档，删除操作不可恢复，请谨慎使用。
    ///
    /// # 参数
    /// * `req` - 删除文档请求
    ///
    /// # 返回值
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, DeleteDocumentRequest};
    ///
    /// let service = DocumentService::new(config);
    /// let request = DeleteDocumentRequest::new("doc_123");
    ///
    /// let result = service.delete(&request).await?;
    /// if result.success.unwrap_or(false) {
    ///     println!("文档删除成功");
    /// }
    /// ```
    pub async fn delete(&self, req: &DeleteDocumentRequest) -> SDKResult<DeleteDocumentResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始删除文档: document_id={}", req.document_id);

        // 构建动态端点路径
        let endpoint = openlark_core::endpoints::Endpoints::DOCX_V1_DOCUMENT_GET
            .replace("{}", &req.document_id);

        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Delete,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None, // DELETE请求无body
            
        };

        let resp =
            Transport::<DeleteDocumentResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档删除完成: document_id={}, success={:?}",
            req.document_id,
            response.success
        );

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 创建文档构建器
#[derive(Clone, Debug)]
pub struct CreateDocumentBuilder {
    request: CreateDocumentRequest,
}

impl Default for CreateDocumentBuilder {
    fn default() -> Self {
        Self {
            request: CreateDocumentRequest {
                title: String::new(),
                folder_token: None,
            },
        }
    }
}

impl CreateDocumentBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文档标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = title.into();
        self
    }

    /// 设置所在文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = Some(folder_token.into());
        self
    }

    /// 执行创建文档操作
    pub async fn execute(self, service: &DocumentService) -> SDKResult<CreateDocumentResponse> {
        service.create(&self.request).await
    }
}

/// 获取文档构建器
#[derive(Clone, Debug)]
pub struct GetDocumentBuilder {
    request: GetDocumentRequest,
}

impl Default for GetDocumentBuilder {
    fn default() -> Self {
        Self {
            request: GetDocumentRequest {
                document_id: String::new(),
            },
        }
    }
}

impl GetDocumentBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文档ID
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::GetDocumentBuilder;
    ///
    /// let builder = GetDocumentBuilder::new().document_id("doc_123");
    /// ```
    pub fn document_id(mut self, document_id: impl Into<String>) -> Self {
        self.request.document_id = document_id.into();
        self
    }

    /// 执行获取文档操作
    ///
    /// # 参数
    /// - `service`: 文档管理服务实例
    ///
    /// # 返回值
    /// 返回文档的详细信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, GetDocumentBuilder};
    ///
    /// let service = DocumentService::new(config);
    ///
    /// let result = GetDocumentBuilder::new()
    ///     .document_id("doc_123")
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(self, service: &DocumentService) -> SDKResult<GetDocumentResponse> {
        service.get(&self.request).await
    }
}

/// 删除文档构建器
#[derive(Clone, Debug)]
pub struct DeleteDocumentBuilder {
    request: DeleteDocumentRequest,
}

impl DeleteDocumentBuilder {
    /// 创建新的构建器
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::DeleteDocumentBuilder;
    ///
    /// let builder = DeleteDocumentBuilder::new("doc_123");
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            request: DeleteDocumentRequest::new(document_id),
        }
    }

    /// 执行删除文档操作
    ///
    /// # 参数
    /// - `service`: 文档管理服务实例
    ///
    /// # 返回值
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, DeleteDocumentBuilder};
    ///
    /// let service = DocumentService::new(config);
    ///
    /// let result = DeleteDocumentBuilder::new("doc_123")
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(self, service: &DocumentService) -> SDKResult<DeleteDocumentResponse> {
        service.delete(&self.request).await
    }
}

/// 获取文档块构建器
#[derive(Clone, Debug)]
pub struct GetDocumentBlocksBuilder {
    request: GetDocumentBlocksRequest,
}

impl GetDocumentBlocksBuilder {
    /// 创建新的构建器
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::GetDocumentBlocksBuilder;
    ///
    /// let builder = GetDocumentBlocksBuilder::new("doc_123");
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            request: GetDocumentBlocksRequest::new(document_id),
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置页面标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 设置文档版本号
    pub fn document_revision_id(mut self, document_revision_id: i32) -> Self {
        self.request = self.request.document_revision_id(document_revision_id);
        self
    }

    /// 执行获取文档块操作
    ///
    /// # 参数
    /// - `service`: 文档管理服务实例
    ///
    /// # 返回值
    /// 返回文档块列表
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, GetDocumentBlocksBuilder};
    ///
    /// let service = DocumentService::new(config);
    ///
    /// let result = GetDocumentBlocksBuilder::new("doc_123")
    ///     .page_size(20)
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(self, service: &DocumentService) -> SDKResult<GetDocumentBlocksResponse> {
        service.get_blocks(&self.request).await
    }
}

/// 创建文档块构建器
#[derive(Clone, Debug)]
pub struct CreateDocumentBlockBuilder {
    request: CreateDocumentBlockRequest,
}

impl CreateDocumentBlockBuilder {
    /// 创建新的构建器
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    /// - `block_id`: 父块ID
    /// - `index`: 块索引位置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::CreateDocumentBlockBuilder;
    ///
    /// let builder = CreateDocumentBlockBuilder::new("doc_123", 123456, 0);
    /// ```
    pub fn new(document_id: impl Into<String>, block_id: i64, index: i32) -> Self {
        Self {
            request: CreateDocumentBlockRequest::new(document_id, block_id, index),
        }
    }

    /// 添加子块
    pub fn add_child(mut self, child: DocumentBlock) -> Self {
        self.request = self.request.add_child(child);
        self
    }

    /// 执行创建文档块操作
    ///
    /// # 参数
    /// - `service`: 文档管理服务实例
    ///
    /// # 返回值
    /// 返回创建的块信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, CreateDocumentBuilder, DocumentBlock};
    ///
    /// let service = DocumentService::new(config);
    /// let child_block = DocumentBlock { /* 设置块内容 */ };
    ///
    /// let result = CreateDocumentBlockBuilder::new("doc_123", 123456, 0)
    ///     .add_child(child_block)
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(
        self,
        service: &DocumentService,
    ) -> SDKResult<CreateDocumentBlockResponse> {
        service.create_block(&self.request).await
    }
}

/// 更新文档块构建器
#[derive(Clone, Debug)]
pub struct UpdateDocumentBlockBuilder {
    request: UpdateDocumentBlockRequest,
}

impl UpdateDocumentBlockBuilder {
    /// 创建新的构建器
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    /// - `block_id`: 块ID
    /// - `block`: 块内容
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{UpdateDocumentBlockBuilder, DocumentBlock};
    ///
    /// let block = DocumentBlock::new();
    /// let builder = UpdateDocumentBlockBuilder::new("doc_123", 123456, block);
    /// ```
    pub fn new(document_id: impl Into<String>, block_id: i64, block: DocumentBlock) -> Self {
        Self {
            request: UpdateDocumentBlockRequest::new(document_id, block_id, block),
        }
    }

    /// 设置是否为增量更新
    pub fn is_insecure(mut self, is_insecure: bool) -> Self {
        self.request = self.request.is_insecure(is_insecure);
        self
    }

    /// 执行更新文档块操作
    ///
    /// # 参数
    /// - `service`: 文档管理服务实例
    ///
    /// # 返回值
    /// 返回更新后的块信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, UpdateDocumentBlockBuilder, DocumentBlock};
    ///
    /// let service = DocumentService::new(config);
    /// let block = DocumentBlock { /* 设置块内容 */ };
    ///
    /// let result = UpdateDocumentBlockBuilder::new("doc_123", 123456, block)
    ///     .is_insecure(true)
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(
        self,
        service: &DocumentService,
    ) -> SDKResult<UpdateDocumentBlockResponse> {
        service.update_block(&self.request).await
    }
}

/// 删除文档块构建器
#[derive(Clone, Debug)]
pub struct DeleteDocumentBlockBuilder {
    request: DeleteDocumentBlockRequest,
}

/// 文档列表查询构建器
#[derive(Clone, Debug)]
pub struct ListDocumentsBuilder {
    request: ListDocumentsRequest,
}

impl DeleteDocumentBlockBuilder {
    /// 创建新的构建器
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    /// - `block_id`: 块ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::DeleteDocumentBlockBuilder;
    ///
    /// let builder = DeleteDocumentBlockBuilder::new("doc_123", 123456);
    /// ```
    pub fn new(document_id: impl Into<String>, block_id: i64) -> Self {
        Self {
            request: DeleteDocumentBlockRequest::new(document_id, block_id),
        }
    }

    /// 设置要删除的子元素ID列表
    pub fn block_id_list(mut self, block_id_list: Vec<i64>) -> Self {
        self.request = self.request.block_id_list(block_id_list);
        self
    }

    /// 执行删除文档块操作
    ///
    /// # 参数
    /// - `service`: 文档管理服务实例
    ///
    /// # 返回值
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, DeleteDocumentBlockBuilder};
    ///
    /// let service = DocumentService::new(config);
    ///
    /// let result = DeleteDocumentBlockBuilder::new("doc_123", 123456)
    ///     .block_id_list(vec![789, 1011])
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(
        self,
        service: &DocumentService,
    ) -> SDKResult<DeleteDocumentBlockResponse> {
        service.delete_block(&self.request).await
    }
}

impl ListDocumentsBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            request: ListDocumentsRequest::new(),
        }
    }

    /// 设置文件夹ID
    pub fn folder_id(mut self, folder_id: impl Into<String>) -> Self {
        self.request = self.request.folder_id(folder_id);
        self
    }

    /// 设置文档类型
    pub fn document_type(mut self, document_type: DocumentType) -> Self {
        self.request = self.request.document_type(document_type);
        self
    }

    /// 设置所有者ID
    pub fn owner_id(mut self, owner_id: impl Into<String>) -> Self {
        self.request = self.request.owner_id(owner_id);
        self
    }

    /// 设置文档状态
    pub fn status(mut self, status: DocumentStatus) -> Self {
        self.request = self.request.status(status);
        self
    }

    /// 设置搜索关键词
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.request = self.request.keyword(keyword);
        self
    }

    /// 设置标签筛选
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.request = self.request.tags(tags);
        self
    }

    /// 设置排序字段
    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.request = self.request.sort_field(sort_field);
        self
    }

    /// 设置排序方向
    pub fn sort_order(mut self, sort_order: impl Into<String>) -> Self {
        self.request = self.request.sort_order(sort_order);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置页面标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 执行文档列表查询操作
    ///
    /// # 参数
    /// - `service`: 文档管理服务实例
    ///
    /// # 返回值
    /// 返回符合条件的文档列表
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, ListDocumentsBuilder};
    ///
    /// let service = DocumentService::new(config);
    ///
    /// let result = ListDocumentsBuilder::new()
    ///     .keyword("项目计划")
    ///     .page_size(20)
    ///     .sort_field("update_time")
    ///     .sort_order("desc")
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(self, service: &DocumentService) -> SDKResult<DocumentListResponse> {
        service.list_documents(&self.request).await
    }
}

impl DocumentService {
    /// 创建文档构建器
    pub fn create_document_builder(&self) -> CreateDocumentBuilder {
        CreateDocumentBuilder::new()
    }

    /// 创建获取文档构建器
    ///
    /// # 返回值
    /// 返回获取文档构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::DocumentService;
    ///
    /// let service = DocumentService::new(config);
    /// let builder = service.get_document_builder();
    /// ```
    pub fn get_document_builder(&self) -> GetDocumentBuilder {
        GetDocumentBuilder::new()
    }

    /// 创建删除文档构建器
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 返回值
    /// 返回删除文档构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::DocumentService;
    ///
    /// let service = DocumentService::new(config);
    /// let builder = service.delete_document_builder("doc_123");
    /// ```
    pub fn delete_document_builder(&self, document_id: impl Into<String>) -> DeleteDocumentBuilder {
        DeleteDocumentBuilder::new(document_id)
    }

    /// 创建获取文档块构建器
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 返回值
    /// 返回获取文档块构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::DocumentService;
    ///
    /// let service = DocumentService::new(config);
    /// let builder = service.get_blocks_builder("doc_123")
    ///     .page_size(20);
    /// ```
    pub fn get_blocks_builder(&self, document_id: impl Into<String>) -> GetDocumentBlocksBuilder {
        GetDocumentBlocksBuilder::new(document_id)
    }

    /// 创建文档块构建器
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    /// - `block_id`: 父块ID
    /// - `index`: 块索引位置
    ///
    /// # 返回值
    /// 返回创建文档块构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::DocumentService;
    ///
    /// let service = DocumentService::new(config);
    /// let builder = service.create_block_builder("doc_123", 123456, 0);
    /// ```
    pub fn create_block_builder(
        &self,
        document_id: impl Into<String>,
        block_id: i64,
        index: i32,
    ) -> CreateDocumentBlockBuilder {
        CreateDocumentBlockBuilder::new(document_id, block_id, index)
    }

    /// 更新文档块构建器
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    /// - `block_id`: 块ID
    /// - `block`: 块内容
    ///
    /// # 返回值
    /// 返回更新文档块构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::{DocumentService, DocumentBlock};
    ///
    /// let service = DocumentService::new(config);
    /// let block = DocumentBlock::new();
    /// let builder = service.update_block_builder("doc_123", 123456, block);
    /// ```
    pub fn update_block_builder(
        &self,
        document_id: impl Into<String>,
        block_id: i64,
        block: DocumentBlock,
    ) -> UpdateDocumentBlockBuilder {
        UpdateDocumentBlockBuilder::new(document_id, block_id, block)
    }

    /// 删除文档块构建器
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    /// - `block_id`: 块ID
    ///
    /// # 返回值
    /// 返回删除文档块构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::DocumentService;
    ///
    /// let service = DocumentService::new(config);
    /// let builder = service.delete_block_builder("doc_123", 123456);
    /// ```
    pub fn delete_block_builder(
        &self,
        document_id: impl Into<String>,
        block_id: i64,
    ) -> DeleteDocumentBlockBuilder {
        DeleteDocumentBlockBuilder::new(document_id, block_id)
    }

    /// 文档列表查询构建器
    ///
    /// # 返回值
    /// 返回文档列表查询构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docs::v1::document::DocumentService;
    ///
    /// let service = DocumentService::new(config);
    /// let builder = service.list_documents_builder();
    /// ```
    pub fn list_documents_builder(&self) -> ListDocumentsBuilder {
        ListDocumentsBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = DocumentService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_document_default_creation() {
        let document = Document::default();
        assert_eq!(document.document_id, None);
        assert_eq!(document.title, None);
        assert_eq!(document.url, None);
        assert_eq!(document.version, None);
        assert_eq!(document.create_time, None);
        assert_eq!(document.update_time, None);
        assert_eq!(document.creator, None);
        assert_eq!(document.folder_token, None);
        assert_eq!(document.status, None);
    }

    #[test]
    fn test_document_with_data() {
        let creator = Creator {
            user_id: Some("user_123".to_string()),
            name: Some("张三".to_string()),
            avatar: Some("avatar_url".to_string()),
        };

        let document = Document {
            document_id: Some("doc_456".to_string()),
            title: Some("项目计划".to_string()),
            url: Some("https://example.com/doc".to_string()),
            version: Some(1),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
            creator: Some(creator),
            folder_token: Some("folder_789".to_string()),
            status: Some("active".to_string()),
        };

        assert_eq!(document.document_id, Some("doc_456".to_string()));
        assert_eq!(document.title, Some("项目计划".to_string()));
        assert_eq!(document.url, Some("https://example.com/doc".to_string()));
        assert_eq!(document.version, Some(1));
        assert_eq!(
            document.creator.as_ref().unwrap().user_id,
            Some("user_123".to_string())
        );
        assert_eq!(
            document.creator.as_ref().unwrap().name,
            Some("张三".to_string())
        );
        assert_eq!(document.folder_token, Some("folder_789".to_string()));
        assert_eq!(document.status, Some("active".to_string()));
    }

    #[test]
    fn test_creator_default_creation() {
        let creator = Creator::default();
        assert_eq!(creator.user_id, None);
        assert_eq!(creator.name, None);
        assert_eq!(creator.avatar, None);
    }

    #[test]
    fn test_create_document_request() {
        let request = CreateDocumentRequest::new("测试文档").folder_token("folder_token");

        assert_eq!(request.title, "测试文档");
        assert_eq!(request.folder_token, Some("folder_token".to_string()));
    }

    #[test]
    fn test_create_document_request_validation() {
        // 测试正常情况
        let valid_request = CreateDocumentRequest::new("有效标题");
        assert!(valid_request.validate().is_ok());

        // 测试空标题
        let empty_title_request = CreateDocumentRequest::new("");
        assert!(empty_title_request.validate().is_err());

        // 测试空白标题
        let whitespace_title_request = CreateDocumentRequest::new("   ");
        assert!(whitespace_title_request.validate().is_err());

        // 测试标题过长
        let long_title_request = CreateDocumentRequest::new(&"a".repeat(201));
        assert!(long_title_request.validate().is_err());

        // 测试标题长度边界
        let boundary_title_request = CreateDocumentRequest::new(&"a".repeat(200));
        assert!(boundary_title_request.validate().is_ok());
    }

    #[test]
    fn test_create_document_builder() {
        let builder = CreateDocumentBuilder::new()
            .title("构建器测试")
            .folder_token("test_folder");

        assert_eq!(builder.request.title, "构建器测试");
        assert_eq!(
            builder.request.folder_token,
            Some("test_folder".to_string())
        );
    }

    #[test]
    fn test_create_document_builder_default() {
        let builder = CreateDocumentBuilder::default();
        assert_eq!(builder.request.title, "");
        assert_eq!(builder.request.folder_token, None);
    }

    #[test]
    fn test_response_default_creation() {
        let response = CreateDocumentResponse::default();
        assert_eq!(response.document.document_id, None);
        assert_eq!(response.document.title, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut response = CreateDocumentResponse::default();
        response.document = Document {
            document_id: Some("doc_abc".to_string()),
            title: Some("响应测试".to_string()),
            
        };

        assert_eq!(response.document.document_id, Some("doc_abc".to_string()));
        assert_eq!(response.document.title, Some("响应测试".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(CreateDocumentResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = CreateDocumentRequest::new("序列化测试").folder_token("test_folder");

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateDocumentRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.title, deserialized.title);
        assert_eq!(request.folder_token, deserialized.folder_token);
    }

    #[test]
    fn test_endpoint_constant() {
        // Test that the endpoint constant is properly defined
        assert_eq!(
            openlark_core::endpoints::Endpoints::DOCX_V1_DOCUMENTS,
            "/open-apis/docx/v1/documents"
        );
    }

    #[test]
    fn test_document_title_variations() {
        // Test different document titles
        let project_doc = Document {
            title: Some("项目计划文档".to_string()),
            
        };

        let meeting_doc = Document {
            title: Some("会议纪要".to_string()),
            
        };

        let report_doc = Document {
            title: Some("月度报告".to_string()),
            
        };

        assert_eq!(project_doc.title, Some("项目计划文档".to_string()));
        assert_eq!(meeting_doc.title, Some("会议纪要".to_string()));
        assert_eq!(report_doc.title, Some("月度报告".to_string()));
    }

    #[test]
    fn test_comprehensive_document_data() {
        // Test comprehensive document data with all fields
        let comprehensive_creator = Creator {
            user_id: Some("creator_001".to_string()),
            name: Some("李四".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
        };

        let comprehensive_document = Document {
            document_id: Some("comprehensive_doc_001".to_string()),
            title: Some("2023年度工作总结".to_string()),
            url: Some("https://docs.example.com/comprehensive_doc_001".to_string()),
            version: Some(3),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
            creator: Some(comprehensive_creator),
            folder_token: Some("reports_folder_2023".to_string()),
            status: Some("published".to_string()),
        };

        assert_eq!(
            comprehensive_document.document_id,
            Some("comprehensive_doc_001".to_string())
        );
        assert_eq!(
            comprehensive_document.title,
            Some("2023年度工作总结".to_string())
        );
        assert_eq!(
            comprehensive_document.url,
            Some("https://docs.example.com/comprehensive_doc_001".to_string())
        );
        assert_eq!(comprehensive_document.version, Some(3));
        assert_eq!(
            comprehensive_document.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_document.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_document.creator.as_ref().unwrap().user_id,
            Some("creator_001".to_string())
        );
        assert_eq!(
            comprehensive_document.creator.as_ref().unwrap().name,
            Some("李四".to_string())
        );
        assert_eq!(
            comprehensive_document.folder_token,
            Some("reports_folder_2023".to_string())
        );
        assert_eq!(comprehensive_document.status, Some("published".to_string()));
    }

    #[test]
    fn test_request_validation_edge_cases() {
        // Test with whitespace-only title
        let whitespace_request = CreateDocumentRequest::new("  \t\n  ");
        assert!(whitespace_request.validate().is_err());

        // Test with special characters in title
        let special_chars_request = CreateDocumentRequest::new("项目计划-Q1_2023.docx");
        assert!(special_chars_request.validate().is_ok());

        // Test with Unicode characters
        let unicode_request = CreateDocumentRequest::new("📊 项目数据 📈");
        assert!(unicode_request.validate().is_ok());
    }

    #[test]
    fn test_document_version_handling() {
        // Test document version
        let versioned_doc = Document {
            document_id: Some("doc_versioned".to_string()),
            title: Some("版本化文档".to_string()),
            version: Some(5),
            
        };

        assert_eq!(versioned_doc.version, Some(5));

        let unversioned_doc = Document {
            document_id: Some("doc_unversioned".to_string()),
            title: Some("无版本文档".to_string()),
            version: None,
            
        };

        assert_eq!(unversioned_doc.version, None);
    }

    #[test]
    fn test_get_document_request() {
        let request = GetDocumentRequest::new("doc_123");
        assert_eq!(request.document_id, "doc_123");
    }

    #[test]
    fn test_get_document_request_validation() {
        // 测试正常情况
        let valid_request = GetDocumentRequest::new("doc_123");
        assert!(valid_request.validate().is_ok());

        // 测试空document_id
        let empty_request = GetDocumentRequest::new("");
        assert!(empty_request.validate().is_err());

        // 测试空白字符
        let whitespace_request = GetDocumentRequest::new("   ");
        assert!(whitespace_request.validate().is_err());

        // 测试长度超限
        let long_request = GetDocumentRequest::new(&"a".repeat(201));
        assert!(long_request.validate().is_err());

        // 测试长度边界
        let boundary_request = GetDocumentRequest::new(&"a".repeat(200));
        assert!(boundary_request.validate().is_ok());
    }

    #[test]
    fn test_get_document_builder() {
        let builder = GetDocumentBuilder::new().document_id("doc_123");
        assert_eq!(builder.request.document_id, "doc_123");
    }

    #[test]
    fn test_get_document_builder_default() {
        let builder = GetDocumentBuilder::default();
        assert_eq!(builder.request.document_id, "");
    }

    #[test]
    fn test_get_document_response_default_creation() {
        let response = GetDocumentResponse::default();
        assert_eq!(response.document.document_id, None);
        assert_eq!(response.document.title, None);
    }

    #[test]
    fn test_get_document_response_with_data() {
        let mut response = GetDocumentResponse::default();
        response.document = Document {
            document_id: Some("doc_abc".to_string()),
            title: Some("获取测试文档".to_string()),
            
        };

        assert_eq!(response.document.document_id, Some("doc_abc".to_string()));
        assert_eq!(response.document.title, Some("获取测试文档".to_string()));
    }

    #[test]
    fn test_get_document_response_api_trait() {
        assert_eq!(GetDocumentResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_get_document_request_serialization() {
        let request = GetDocumentRequest::new("doc_123");
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetDocumentRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.document_id, deserialized.document_id);
    }

    #[test]
    fn test_get_document_response_serialization() {
        let mut response = GetDocumentResponse::default();
        response.document = Document {
            document_id: Some("doc_xyz".to_string()),
            title: Some("序列化测试".to_string()),
            version: Some(2),
            
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetDocumentResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            response.document.document_id,
            deserialized.document.document_id
        );
        assert_eq!(response.document.title, deserialized.document.title);
        assert_eq!(response.document.version, deserialized.document.version);
    }

    #[test]
    fn test_get_document_builder_chain_calls() {
        let builder = GetDocumentBuilder::new()
            .document_id("doc_123")
            .document_id("doc_456"); // 覆盖之前的值

        assert_eq!(builder.request.document_id, "doc_456");
    }

    #[test]
    fn test_get_document_request_validation_edge_cases() {
        // 测试仅包含空白字符的document_id
        let whitespace_request = GetDocumentRequest::new("  \t\n  ");
        assert!(whitespace_request.validate().is_err());

        // 测试中文字符（虽然可能不常见，但应该能处理）
        let chinese_request = GetDocumentRequest::new("文档_123");
        assert!(chinese_request.validate().is_ok());

        // 测试包含特殊字符的document_id
        let special_chars_request = GetDocumentRequest::new("doc_abc-123_xyz");
        assert!(special_chars_request.validate().is_ok());
    }

    #[test]
    fn test_get_document_endpoint_constant() {
        // 测试端点常量是否正确定义
        assert_eq!(
            openlark_core::endpoints::Endpoints::DOCX_V1_DOCUMENT_GET,
            "/open-apis/docx/v1/documents/{}"
        );
    }

    #[test]
    fn test_get_document_response_comprehensive_data() {
        // 测试包含完整数据的文档响应
        let comprehensive_creator = Creator {
            user_id: Some("user_001".to_string()),
            name: Some("测试用户".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
        };

        let comprehensive_response = GetDocumentResponse {
            document: Document {
                document_id: Some("doc_comprehensive".to_string()),
                title: Some("综合测试文档".to_string()),
                url: Some("https://docs.example.com/doc_comprehensive".to_string()),
                version: Some(5),
                create_time: Some("2023-01-01T08:00:00Z".to_string()),
                update_time: Some("2023-12-31T16:00:00Z".to_string()),
                creator: Some(comprehensive_creator),
                folder_token: Some("folder_123".to_string()),
                status: Some("published".to_string()),
            },
        };

        assert_eq!(
            comprehensive_response.document.document_id,
            Some("doc_comprehensive".to_string())
        );
        assert_eq!(
            comprehensive_response.document.title,
            Some("综合测试文档".to_string())
        );
        assert_eq!(
            comprehensive_response.document.url,
            Some("https://docs.example.com/doc_comprehensive".to_string())
        );
        assert_eq!(comprehensive_response.document.version, Some(5));
        assert_eq!(
            comprehensive_response.document.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_response.document.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_response
                .document
                .creator
                .as_ref()
                .unwrap()
                .user_id,
            Some("user_001".to_string())
        );
        assert_eq!(
            comprehensive_response
                .document
                .creator
                .as_ref()
                .unwrap()
                .name,
            Some("测试用户".to_string())
        );
        assert_eq!(
            comprehensive_response.document.folder_token,
            Some("folder_123".to_string())
        );
        assert_eq!(
            comprehensive_response.document.status,
            Some("published".to_string())
        );
    }

    // ==================== 群公告块内容获取 API 测试 ====================

    #[test]
    fn test_get_announcement_block_content_request_creation() {
        let request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456");
        assert_eq!(request.chat_id, "chat_123");
        assert_eq!(request.block_id, "block_456");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_set_user_id_type() {
        let mut request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456");
        request.set_user_id_type("user_id");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_request_validation_success() {
        let request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_request_validation_with_user_id_type() {
        let mut request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456");
        request.set_user_id_type("open_id");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_request_validation_empty_chat_id() {
        let request = GetAnnouncementBlockContentRequest::new("", "block_456");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "群聊ID不能为空");
    }

    #[test]
    fn test_request_validation_empty_block_id() {
        let request = GetAnnouncementBlockContentRequest::new("chat_123", "");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "公告块ID不能为空");
    }

    #[test]
    fn test_request_validation_empty_user_id_type() {
        let mut request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456");
        request.set_user_id_type("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_request_validation_invalid_user_id_type() {
        let mut request = GetAnnouncementBlockContentRequest::new("chat_123", "block_456");
        request.set_user_id_type("invalid_type");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "用户ID类型必须是 open_id、user_id 或 union_id"
        );
    }

    #[test]
    fn test_get_announcement_block_content_builder_creation() {
        let builder = GetAnnouncementBlockContentBuilder::new("chat_123", "block_456");
        assert_eq!(builder.request.chat_id, "chat_123");
        assert_eq!(builder.request.block_id, "block_456");
        assert!(builder.request.user_id_type.is_none());
    }

    #[test]
    fn test_builder_user_id_type() {
        let builder = GetAnnouncementBlockContentBuilder::new("chat_123", "block_456")
            .user_id_type("union_id");
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_builder_build_success() {
        let result = GetAnnouncementBlockContentBuilder::new("chat_123", "block_456")
            .user_id_type("open_id")
            .build();
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.chat_id, "chat_123");
        assert_eq!(request.block_id, "block_456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_builder_build_failure_empty_chat_id() {
        let result = GetAnnouncementBlockContentBuilder::new("", "block_456").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "群聊ID不能为空");
    }

    #[test]
    fn test_builder_build_failure_empty_block_id() {
        let result = GetAnnouncementBlockContentBuilder::new("chat_123", "").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "公告块ID不能为空");
    }

    #[test]
    fn test_announcement_block_content_default() {
        let content = AnnouncementBlockContent::default();
        assert_eq!(content.block_id, None);
        assert_eq!(content.block_type, None);
        assert_eq!(content.content, None);
        assert_eq!(content.title, None);
        assert_eq!(content.create_time, None);
        assert_eq!(content.update_time, None);
        assert_eq!(content.creator, None);
    }

    #[test]
    fn test_get_announcement_block_content_response_default() {
        let response = GetAnnouncementBlockContentResponse::default();
        assert_eq!(response.block_content, None);
        assert_eq!(response.success, None);
        assert_eq!(response.message, None);
    }

    #[test]
    fn test_builder_default() {
        let builder = GetAnnouncementBlockContentBuilder::default();
        assert_eq!(builder.request.chat_id, "");
        assert_eq!(builder.request.block_id, "");
        assert!(builder.request.user_id_type.is_none());
    }

    #[test]
    fn test_request_with_whitespace_chat_id() {
        let request = GetAnnouncementBlockContentRequest::new("   ", "block_456");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "群聊ID不能为空");
    }

    #[test]
    fn test_request_with_whitespace_block_id() {
        let request = GetAnnouncementBlockContentRequest::new("chat_123", "   ");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "公告块ID不能为空");
    }

    #[test]
    fn test_serialization_request() {
        let request = GetAnnouncementBlockContentRequest {
            chat_id: "chat_123".to_string(),
            block_id: "block_456".to_string(),
            user_id_type: Some("open_id".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("chat_123"));
        assert!(json.contains("block_456"));
        assert!(json.contains("open_id"));
    }

    #[test]
    fn test_serialization_request_without_optional_fields() {
        let request = GetAnnouncementBlockContentRequest {
            chat_id: "chat_123".to_string(),
            block_id: "block_456".to_string(),
            user_id_type: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("chat_123"));
        assert!(json.contains("block_456"));
        assert!(!json.contains("user_id_type"));
    }

    #[test]
    fn test_deserialization_response() {
        let json = r#"
        {
            "block_content": {
                "block_id": "block_456",
                "block_type": "text",
                "content": "这是一条公告内容",
                "title": "重要公告",
                "create_time": "2024-01-01T00:00:00Z",
                "update_time": "2024-01-02T00:00:00Z"
            },
            "success": true,
            "message": "获取成功"
        }
        "#;
        let response: GetAnnouncementBlockContentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.success, Some(true));
        assert!(response.block_content.is_some());
        assert_eq!(
            response.block_content.unwrap().block_id,
            Some("block_456".to_string())
        );
        assert_eq!(response.message, Some("获取成功".to_string()));
    }

    #[test]
    fn test_fluent_api_chain() {
        let result = GetAnnouncementBlockContentBuilder::new("chat_123", "block_456")
            .user_id_type("user_id")
            .build();
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.chat_id, "chat_123");
        assert_eq!(request.block_id, "block_456");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_validation_edge_cases() {
        // Test with very long IDs
        let long_id = "a".repeat(1000);
        let request = GetAnnouncementBlockContentRequest::new(&long_id, &long_id);
        assert!(request.validate().is_ok());

        // Test with Unicode characters
        let unicode_request = GetAnnouncementBlockContentRequest::new("群聊_123", "公告块_456");
        assert!(unicode_request.validate().is_ok());
    }

    #[test]
    fn test_builder_fluent_with_all_options() {
        let builder = GetAnnouncementBlockContentBuilder::new("chat_123", "block_456")
            .user_id_type("union_id");

        assert_eq!(builder.request.chat_id, "chat_123");
        assert_eq!(builder.request.block_id, "block_456");
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));

        let request = builder.build().unwrap();
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_announcement_block_content_with_creator() {
        let creator = Creator {
            user_id: Some("user_123".to_string()),
            name: Some("张三".to_string()),
        };

        let content = AnnouncementBlockContent {
            block_id: Some("block_456".to_string()),
            block_type: Some("rich_text".to_string()),
            content: Some("富文本公告内容".to_string()),
            title: Some("系统公告".to_string()),
            create_time: Some("2024-01-01T08:00:00Z".to_string()),
            update_time: Some("2024-01-01T09:00:00Z".to_string()),
            creator: Some(creator),
        };

        assert_eq!(content.block_id, Some("block_456".to_string()));
        assert_eq!(content.block_type, Some("rich_text".to_string()));
        assert_eq!(content.content, Some("富文本公告内容".to_string()));
        assert_eq!(content.title, Some("系统公告".to_string()));
        assert_eq!(
            content.creator.as_ref().unwrap().name,
            Some("张三".to_string())
        );
    }
}

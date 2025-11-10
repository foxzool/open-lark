//! 创建嵌套块API v1
//!
//! 提供飞书云文档嵌套块创建功能，包括：
//! - 在指定块下创建嵌套子块
//! - 支持多种块类型和内容格式
//! - 自动处理块层级关系
//! - 完整的错误处理和参数验证

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{Document, Creator};

/// 创建嵌套块请求
///
/// 用于在文档的指定块下创建嵌套子块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDescendantBlockRequest {
    /// 块内容
    pub block_content: BlockContent,
    /// 块类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<i32>,
    /// 块索引
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 父块ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 子块ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
}

impl CreateDescendantBlockRequest {
    /// 创建新的嵌套块请求实例
    ///
    /// # 参数
    /// - `block_content` - 块内容
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::docx::v1::document_block_descendant::CreateDescendantBlockRequest;
    /// use open_lark::service::cloud_docs::docx::v1::document_block_descendant::BlockContent;
    ///
    /// let content = BlockContent::new("这是嵌套块内容");
    /// let request = CreateDescendantBlockRequest::new(content);
    /// ```
    pub fn new(block_content: BlockContent) -> Self {
        Self {
            block_content,
            block_type: None,
            index: None,
            parent_id: None,
            children: None,
        }
    }

    /// 设置块类型
    ///
    /// # 参数
    /// - `block_type` - 块类型
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::CreateDescendantBlockRequest;
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::BlockContent;
    ///
    /// let mut request = CreateDescendantBlockRequest::new(BlockContent::new("内容"));
    /// request.set_block_type(1);
    /// ```
    pub fn set_block_type(&mut self, block_type: i32) -> &mut Self {
        self.block_type = Some(block_type);
        self
    }

    /// 设置块索引
    ///
    /// # 参数
    /// - `index` - 块索引
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::CreateDescendantBlockRequest;
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::BlockContent;
    ///
    /// let mut request = CreateDescendantBlockRequest::new(BlockContent::new("内容"));
    /// request.set_index(5);
    /// ```
    pub fn set_index(&mut self, index: i32) -> &mut Self {
        self.index = Some(index);
        self
    }

    /// 设置父块ID
    ///
    /// # 参数
    /// - `parent_id` - 父块ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::CreateDescendantBlockRequest;
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::BlockContent;
    ///
    /// let mut request = CreateDescendantBlockRequest::new(BlockContent::new("内容"));
    /// request.set_parent_id("parent_block_123".to_string());
    /// ```
    pub fn set_parent_id(&mut self, parent_id: impl Into<String>) -> &mut Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// 设置子块ID列表
    ///
    /// # 参数
    /// - `children` - 子块ID列表
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::CreateDescendantBlockRequest;
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::BlockContent;
    ///
    /// let mut request = CreateDescendantBlockRequest::new(BlockContent::new("内容"));
    /// request.set_children(vec!["child_1".to_string(), "child_2".to_string()]);
    /// ```
    pub fn set_children(&mut self, children: Vec<String>) -> &mut Self {
        self.children = Some(children);
        self
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 验证通过
    /// - `Err(String)`: 验证失败，返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref content) = self.block_content.text {
            if content.trim().is_empty() {
                return Err("块内容不能为空".to_string());
            }
            if content.len() > 10000 {
                return Err("块内容长度不能超过10000个字符".to_string());
            }
        }

        if let Some(index) = self.index {
            if index < 0 {
                return Err("块索引不能为负数".to_string());
            }
            if index > 10000 {
                return Err("块索引不能超过10000".to_string());
            }
        }

        if let Some(ref parent_id) = self.parent_id {
            if parent_id.trim().is_empty() {
                return Err("父块ID不能为空".to_string());
            }
        }

        if let Some(ref children) = self.children {
            if children.is_empty() {
                return Err("子块列表不能为空".to_string());
            }
            if children.len() > 1000 {
                return Err("子块数量不能超过1000个".to_string());
            }
            for child_id in children {
                if child_id.trim().is_empty() {
                    return Err("子块ID不能为空".to_string());
                }
            }
        }

        Ok(())
    }
}

/// 块内容
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockContent {
    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 块元素列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<BlockElement>>,
}

impl BlockContent {
    /// 创建新的块内容
    ///
    /// # 参数
    /// - `text` - 文本内容
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::docx::v1::document_block_descendant::BlockContent;
    ///
    /// let content = BlockContent::new("这是文本内容");
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            elements: None,
        }
    }

    /// 设置块元素
    ///
    /// # 参数
    /// - `elements` - 块元素列表
    pub fn set_elements(&mut self, elements: Vec<BlockElement>) -> &mut Self {
        self.elements = Some(elements);
        self
    }
}

impl Default for BlockContent {
    fn default() -> Self {
        Self {
            text: None,
            elements: None,
        }
    }
}

/// 块元素
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockElement {
    /// 元素类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
    /// 元素内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 元素属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

impl Default for BlockElement {
    fn default() -> Self {
        Self {
            type_field: None,
            content: None,
            attributes: None,
        }
    }
}

/// 创建嵌套块响应数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDescendantBlockResponseData {
    /// 创建的块信息
    pub block: Block,
}

/// 创建嵌套块响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDescendantBlockResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateDescendantBlockResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for CreateDescendantBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 块信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    /// 块ID
    pub block_id: String,
    /// 父块ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 子块ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
    /// 块类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<i32>,
    /// 块索引
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 块内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<BlockContent>,
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

impl Default for Block {
    fn default() -> Self {
        Self {
            block_id: String::new(),
            parent_id: None,
            children: None,
            block_type: None,
            index: None,
            content: None,
            create_time: None,
            update_time: None,
            creator: None,
        }
    }
}

/// DocumentService扩展 - 嵌套块管理
impl super::DocumentService {
    /// 创建嵌套块
    ///
    /// 在文档的指定块下创建嵌套子块，支持丰富的内容格式和层级管理
    ///
    /// # 参数
    /// * `document_id` - 文档ID
    /// * `block_id` - 父块ID
    /// * `req` - 创建嵌套块请求
    ///
    /// # 返回值
    /// 返回创建的嵌套块信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::document::{DocumentService};
    /// use open_lark::service::cloud_docs::docx::v1::document_block_descendant::{
    ///     CreateDescendantBlockRequest, BlockContent
    /// };
    ///
    /// let service = DocumentService::new(config);
    /// let content = BlockContent::new("这是嵌套块内容");
    /// let request = CreateDescendantBlockRequest::new(content)
    ///     .set_block_type(1)
    ///     .set_index(5)
    ///     .set_parent_id("parent_block_123".to_string());
    ///
    /// let result = service.create_descendant_block(
    ///     "doc_123",
    ///     "parent_block_456",
    ///     &request
    /// ).await?;
    /// println!("嵌套块创建成功: {}", result.data.unwrap().block.block_id);
    /// ```
    pub async fn create_descendant_block(
        &self,
        document_id: impl Into<String>,
        block_id: impl Into<String>,
        req: &CreateDescendantBlockRequest,
    ) -> SDKResult<CreateDescendantBlockResponse> {
        let document_id = document_id.into();
        let block_id = block_id.into();

        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始创建嵌套块: document_id={}, block_id={}", document_id, block_id);

        let endpoint = crate::core::endpoints_original::Endpoints::DOCX_V1_DOCUMENT_BLOCK_DESCENDANT_CREATE
            .replace("{document_id}", &document_id)
            .replace("{block_id}", &block_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: endpoint,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<CreateDescendantBlockResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            if let Some(ref data) = response.data {
                log::info!(
                    "嵌套块创建成功: document_id={}, block_id={}, new_block_id={}",
                    document_id,
                    block_id,
                    data.block.block_id
                );
            }
        } else {
            log::warn!(
                "嵌套块创建失败: document_id={}, block_id={}, error={:?}",
                document_id,
                block_id,
                response.error_message
            );
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 创建嵌套块构建器
#[derive(Debug, Clone)]
pub struct CreateDescendantBlockBuilder {
    document_id: String,
    block_id: String,
    request: CreateDescendantBlockRequest,
}

impl Default for CreateDescendantBlockBuilder {
    fn default() -> Self {
        Self {
            document_id: String::new(),
            block_id: String::new(),
            request: CreateDescendantBlockRequest {
                block_content: BlockContent::default(),
                block_type: None,
                index: None,
                parent_id: None,
                children: None,
            },
        }
    }
}

impl CreateDescendantBlockBuilder {
    /// 创建新的构建器
    ///
    /// # 参数
    /// - `document_id` - 文档ID
    /// - `block_id` - 父块ID
    /// - `block_content` - 块内容
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::docx::v1::document_block_descendant::{
    ///     CreateDescendantBlockBuilder, BlockContent
    /// };
    ///
    /// let builder = CreateDescendantBlockBuilder::new(
    ///     "doc_123",
    ///     "parent_block_456",
    ///     BlockContent::new("嵌套块内容")
    /// );
    /// ```
    pub fn new(
        document_id: impl Into<String>,
        block_id: impl Into<String>,
        block_content: BlockContent,
    ) -> Self {
        Self {
            document_id: document_id.into(),
            block_id: block_id.into(),
            request: CreateDescendantBlockRequest::new(block_content),
        }
    }

    /// 设置块类型
    ///
    /// # 参数
    /// - `block_type` - 块类型
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::{CreateDescendantBlockBuilder, BlockContent};
    ///
    /// let builder = CreateDescendantBlockBuilder::new("doc_123", "parent_456", BlockContent::new("内容"))
    ///     .block_type(1);
    /// ```
    pub fn block_type(mut self, block_type: i32) -> Self {
        self.request.set_block_type(block_type);
        self
    }

    /// 设置块索引
    ///
    /// # 参数
    /// - `index` - 块索引
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::{CreateDescendantBlockBuilder, BlockContent};
    ///
    /// let builder = CreateDescendantBlockBuilder::new("doc_123", "parent_456", BlockContent::new("内容"))
    ///     .index(5);
    /// ```
    pub fn index(mut self, index: i32) -> Self {
        self.request.set_index(index);
        self
    }

    /// 设置父块ID
    ///
    /// # 参数
    /// - `parent_id` - 父块ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::{CreateDescendantBlockBuilder, BlockContent};
    ///
    /// let builder = CreateDescendantBlockBuilder::new("doc_123", "parent_456", BlockContent::new("内容"))
    ///     .parent_id("new_parent_789");
    /// ```
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.request.set_parent_id(parent_id);
        self
    }

    /// 设置子块ID列表
    ///
    /// # 参数
    /// - `children` - 子块ID列表
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document_block_descendant::{CreateDescendantBlockBuilder, BlockContent};
    ///
    /// let builder = CreateDescendantBlockBuilder::new("doc_123", "parent_456", BlockContent::new("内容"))
    ///     .children(vec!["child_1".to_string(), "child_2".to_string()]);
    /// ```
    pub fn children(mut self, children: Vec<String>) -> Self {
        self.request.set_children(children);
        self
    }

    /// 执行创建嵌套块操作
    ///
    /// # 参数
    /// - `service` - 文档服务实例
    ///
    /// # 返回值
    /// 返回创建的嵌套块信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::document::{DocumentService};
    /// use open_lark::service::cloud_docs::docx::v1::document_block_descendant::{
    ///     CreateDescendantBlockBuilder, BlockContent
    /// };
    ///
    /// let service = DocumentService::new(config);
    ///
    /// let result = CreateDescendantBlockBuilder::new(
    ///     "doc_123",
    ///     "parent_block_456",
    ///     BlockContent::new("嵌套块内容")
    /// )
    /// .block_type(1)
    /// .index(5)
    /// .execute(&service)
    /// .await?;
    /// ```
    pub async fn execute(self, service: &super::DocumentService) -> SDKResult<CreateDescendantBlockResponse> {
        service.create_descendant_block(&self.document_id, &self.block_id, &self.request).await
    }
}

/// DocumentService扩展 - 构建器模式
impl super::DocumentService {
    /// 创建嵌套块构建器
    ///
    /// # 参数
    /// - `document_id` - 文档ID
    /// - `block_id` - 父块ID
    /// - `block_content` - 块内容
    ///
    /// # 返回值
    /// 返回嵌套块构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::document::{DocumentService};
    /// use open_lark::service::cloud_docs::docx::v1::document_block_descendant::{
    ///     CreateDescendantBlockBuilder, BlockContent
    /// };
    ///
    /// let service = DocumentService::new(config);
    /// let builder = service.create_descendant_block_builder(
    ///     "doc_123",
    ///     "parent_block_456",
    ///     BlockContent::new("嵌套块内容")
    /// );
    /// ```
    pub fn create_descendant_block_builder(
        &self,
        document_id: impl Into<String>,
        block_id: impl Into<String>,
        block_content: BlockContent,
    ) -> CreateDescendantBlockBuilder {
        CreateDescendantBlockBuilder::new(document_id, block_id, block_content)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_content_creation() {
        let content = BlockContent::new("测试内容");
        assert_eq!(content.text, Some("测试内容".to_string()));
        assert_eq!(content.elements, None);
    }

    #[test]
    fn test_block_content_with_elements() {
        let element = BlockElement {
            type_field: Some("text".to_string()),
            content: Some("元素内容".to_string()),
            attributes: None,
        };

        let mut content = BlockContent::new("文本内容");
        content.set_elements(vec![element]);

        assert_eq!(content.text, Some("文本内容".to_string()));
        assert_eq!(content.elements.as_ref().unwrap().len(), 1);
        assert_eq!(content.elements.as_ref().unwrap()[0].type_field, Some("text".to_string()));
    }

    #[test]
    fn test_create_descendant_block_request_creation() {
        let content = BlockContent::new("测试内容");
        let request = CreateDescendantBlockRequest::new(content);

        assert_eq!(request.block_content.text, Some("测试内容".to_string()));
        assert_eq!(request.block_type, None);
        assert_eq!(request.index, None);
        assert_eq!(request.parent_id, None);
        assert_eq!(request.children, None);
    }

    #[test]
    fn test_create_descendant_block_request_with_fields() {
        let content = BlockContent::new("测试内容");
        let mut request = CreateDescendantBlockRequest::new(content);

        request.set_block_type(1)
               .set_index(5)
               .set_parent_id("parent_block_123".to_string())
               .set_children(vec!["child_1".to_string(), "child_2".to_string()]);

        assert_eq!(request.block_type, Some(1));
        assert_eq!(request.index, Some(5));
        assert_eq!(request.parent_id, Some("parent_block_123".to_string()));
        assert_eq!(request.children.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_request_validation_success() {
        let content = BlockContent::new("有效内容");
        let request = CreateDescendantBlockRequest::new(content);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_request_validation_empty_content() {
        let content = BlockContent::new("");
        let request = CreateDescendantBlockRequest::new(content);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "块内容不能为空");
    }

    #[test]
    fn test_request_validation_content_too_long() {
        let content = BlockContent::new(&"a".repeat(10001));
        let request = CreateDescendantBlockRequest::new(content);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "块内容长度不能超过10000个字符");
    }

    #[test]
    fn test_request_validation_negative_index() {
        let content = BlockContent::new("内容");
        let mut request = CreateDescendantBlockRequest::new(content);
        request.set_index(-1);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "块索引不能为负数");
    }

    #[test]
    fn test_request_validation_empty_parent_id() {
        let content = BlockContent::new("内容");
        let mut request = CreateDescendantBlockRequest::new(content);
        request.set_parent_id("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "父块ID不能为空");
    }

    #[test]
    fn test_request_validation_empty_children() {
        let content = BlockContent::new("内容");
        let mut request = CreateDescendantBlockRequest::new(content);
        request.set_children(vec![]);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "子块列表不能为空");
    }

    #[test]
    fn test_request_validation_too_many_children() {
        let content = BlockContent::new("内容");
        let children: Vec<String> = (0..1001).map(|i| format!("child_{}", i)).collect();
        let mut request = CreateDescendantBlockRequest::new(content);
        request.set_children(children);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "子块数量不能超过1000个");
    }

    #[test]
    fn test_block_default_creation() {
        let block = Block::default();
        assert_eq!(block.block_id, "");
        assert_eq!(block.parent_id, None);
        assert_eq!(block.children, None);
        assert_eq!(block.block_type, None);
        assert_eq!(block.index, None);
        assert_eq!(block.content, None);
        assert_eq!(block.create_time, None);
        assert_eq!(block.update_time, None);
        assert_eq!(block.creator, None);
    }

    #[test]
    fn test_block_with_data() {
        let creator = Creator {
            user_id: Some("user_123".to_string()),
            name: Some("张三".to_string()),
            avatar: Some("avatar_url".to_string()),
        };

        let block_content = BlockContent::new("块内容");
        let block = Block {
            block_id: "block_456".to_string(),
            parent_id: Some("parent_789".to_string()),
            children: Some(vec!["child_1".to_string(), "child_2".to_string()]),
            block_type: Some(1),
            index: Some(5),
            content: Some(block_content),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
            creator: Some(creator),
        };

        assert_eq!(block.block_id, "block_456");
        assert_eq!(block.parent_id, Some("parent_789".to_string()));
        assert_eq!(block.children.as_ref().unwrap().len(), 2);
        assert_eq!(block.block_type, Some(1));
        assert_eq!(block.index, Some(5));
        assert_eq!(block.content.as_ref().unwrap().text, Some("块内容".to_string()));
        assert_eq!(block.creator.as_ref().unwrap().user_id, Some("user_123".to_string()));
    }

    #[test]
    fn test_create_descendant_block_builder() {
        let builder = CreateDescendantBlockBuilder::new(
            "doc_123",
            "parent_block_456",
            BlockContent::new("嵌套块内容")
        )
        .block_type(1)
        .index(5)
        .parent_id("new_parent_789")
        .children(vec!["child_1".to_string()]);

        assert_eq!(builder.document_id, "doc_123");
        assert_eq!(builder.block_id, "parent_block_456");
        assert_eq!(builder.request.block_type, Some(1));
        assert_eq!(builder.request.index, Some(5));
        assert_eq!(builder.request.parent_id, Some("new_parent_789".to_string()));
        assert_eq!(builder.request.children.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_builder_default() {
        let builder = CreateDescendantBlockBuilder::default();
        assert_eq!(builder.document_id, "");
        assert_eq!(builder.block_id, "");
        assert_eq!(builder.request.block_content.text, None);
        assert_eq!(builder.request.block_type, None);
    }

    #[test]
    fn test_response_default_creation() {
        let response = CreateDescendantBlockResponse::default();
        assert_eq!(response.data, None);
        assert_eq!(response.success, false);
        assert_eq!(response.error_message, None);
        assert_eq!(response.error_code, None);
    }

    #[test]
    fn test_response_with_data() {
        let block = Block {
            block_id: "block_abc".to_string(),
            parent_id: Some("parent_xyz".to_string()),
            ..Default::default()
        };

        let data = CreateDescendantBlockResponseData {
            block,
        };

        let mut response = CreateDescendantBlockResponse::default();
        response.data = Some(data);
        response.success = true;

        assert!(response.success);
        assert_eq!(response.data.unwrap().block.block_id, "block_abc");
        assert_eq!(response.data.unwrap().block.parent_id, Some("parent_xyz".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            CreateDescendantBlockResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_request_serialization() {
        let content = BlockContent::new("序列化测试");
        let mut request = CreateDescendantBlockRequest::new(content);
        request.set_block_type(2)
               .set_index(10)
               .set_parent_id("parent_test".to_string())
               .set_children(vec!["child_test".to_string()]);

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateDescendantBlockRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.block_type, deserialized.block_type);
        assert_eq!(request.index, deserialized.index);
        assert_eq!(request.parent_id, deserialized.parent_id);
        assert_eq!(request.children, deserialized.children);
    }

    #[test]
    fn test_response_serialization() {
        let block = Block {
            block_id: "test_block".to_string(),
            content: Some(BlockContent::new("测试内容".to_string())),
            ..Default::default()
        };

        let data = CreateDescendantBlockResponseData {
            block,
        };

        let mut response = CreateDescendantBlockResponse::default();
        response.data = Some(data);
        response.success = true;

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: CreateDescendantBlockResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.success, deserialized.success);
        assert_eq!(response.data.unwrap().block.block_id, deserialized.data.unwrap().block.block_id);
        assert_eq!(
            response.data.unwrap().block.content.as_ref().unwrap().text,
            deserialized.data.unwrap().block.content.as_ref().unwrap().text
        );
    }

    #[test]
    fn test_block_element_creation() {
        let element = BlockElement {
            type_field: Some("text".to_string()),
            content: Some("元素内容".to_string()),
            attributes: Some(serde_json::json!({"style": "bold"})),
        };

        assert_eq!(element.type_field, Some("text".to_string()));
        assert_eq!(element.content, Some("元素内容".to_string()));
        assert_eq!(element.attributes.as_ref().unwrap()["style"], "bold");
    }

    #[test]
    fn test_block_element_default() {
        let element = BlockElement::default();
        assert_eq!(element.type_field, None);
        assert_eq!(element.content, None);
        assert_eq!(element.attributes, None);
    }

    #[test]
    fn test_complex_block_content() {
        let element1 = BlockElement {
            type_field: Some("text".to_string()),
            content: Some("第一段".to_string()),
            attributes: Some(serde_json::json!({"size": 16})),
        };

        let element2 = BlockElement {
            type_field: Some("image".to_string()),
            content: Some("图片URL".to_string()),
            attributes: None,
        };

        let mut content = BlockContent::new("主内容");
        content.set_elements(vec![element1, element2]);

        assert_eq!(content.text, Some("主内容".to_string()));
        assert_eq!(content.elements.as_ref().unwrap().len(), 2);
        assert_eq!(content.elements.as_ref().unwrap()[0].type_field, Some("text".to_string()));
        assert_eq!(content.elements.as_ref().unwrap()[1].type_field, Some("image".to_string()));
    }

    #[test]
    fn test_comprehensive_scenario() {
        // 测试完整的业务场景
        let creator = Creator {
            user_id: Some("user_001".to_string()),
            name: Some("李四".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
        };

        let mut content = BlockContent::new("这是一个复杂的嵌套块");
        content.set_elements(vec![
            BlockElement {
                type_field: Some("title".to_string()),
                content: Some("标题".to_string()),
                attributes: Some(serde_json::json!({"level": 1})),
            },
            BlockElement {
                type_field: Some("paragraph".to_string()),
                content: Some("段落内容".to_string()),
                attributes: None,
            }
        ]);

        let request = CreateDescendantBlockRequest::new(content.clone())
            .set_block_type(1)
            .set_index(3)
            .set_parent_id("parent_block_001".to_string())
            .set_children(vec!["child_001".to_string(), "child_002".to_string()]);

        // 验证请求参数
        assert!(request.validate().is_ok());
        assert_eq!(request.block_type, Some(1));
        assert_eq!(request.index, Some(3));
        assert_eq!(request.parent_id, Some("parent_block_001".to_string()));
        assert_eq!(request.children.as_ref().unwrap().len(), 2);

        // 验证内容
        assert_eq!(request.block_content.text, Some("这是一个复杂的嵌套块".to_string()));
        assert_eq!(request.block_content.elements.as_ref().unwrap().len(), 2);

        // 创建对应的块对象
        let block = Block {
            block_id: "nested_block_001".to_string(),
            parent_id: Some("parent_block_001".to_string()),
            children: Some(vec!["child_001".to_string(), "child_002".to_string()]),
            block_type: Some(1),
            index: Some(3),
            content: Some(content),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-01-15T16:00:00Z".to_string()),
            creator: Some(creator),
        };

        // 验证块对象
        assert_eq!(block.block_id, "nested_block_001");
        assert_eq!(block.parent_id, Some("parent_block_001".to_string()));
        assert_eq!(block.children.as_ref().unwrap().len(), 2);
        assert_eq!(block.block_type, Some(1));
        assert_eq!(block.index, Some(3));
        assert_eq!(block.content.as_ref().unwrap().text, Some("这是一个复杂的嵌套块".to_string()));
        assert_eq!(block.creator.as_ref().unwrap().user_id, Some("user_001".to_string()));
    }

    #[test]
    fn test_edge_cases_validation() {
        // 测试边界条件

        // 测试最小有效索引
        let content = BlockContent::new("内容");
        let mut request = CreateDescendantBlockRequest::new(content);
        request.set_index(0);
        assert!(request.validate().is_ok());

        // 测试最大有效索引
        request.set_index(10000);
        assert!(request.validate().is_ok());

        // 测试超过最大索引
        request.set_index(10001);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "块索引不能超过10000");

        // 测试最小有效子块数量
        request.set_children(vec!["child_1".to_string()]);
        assert!(request.validate().is_ok());

        // 测试最大有效子块数量
        let children: Vec<String> = (0..1000).map(|i| format!("child_{}", i)).collect();
        request.set_children(children);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_unicode_content_handling() {
        // 测试Unicode字符处理
        let content = BlockContent::new("🚀 嵌套块内容 - 测试Unicode支持 🎉");
        let request = CreateDescendantBlockRequest::new(content);
        assert!(request.validate().is_ok());

        // 测试包含Unicode的父块ID
        let mut request_with_unicode = CreateDescendantBlockRequest::new(BlockContent::new("内容"));
        request_with_unicode.set_parent_id("父块📝_123");
        assert!(request_with_unicode.validate().is_ok());

        // 测试包含Unicode的子块ID
        request_with_unicode.set_children(vec!["子块🔗_1".to_string()]);
        assert!(request_with_unicode.validate().is_ok());
    }
}
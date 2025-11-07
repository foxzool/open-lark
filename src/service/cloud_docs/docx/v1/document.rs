//! 文档管理服务 v1
//!
//! 提供飞书云文档的完整管理功能，包括：
//! - 创建新文档
//! - 获取文档基本信息
//! - 获取文档纯文本内容
//! - 获取文档所有块
//! - 文档格式转换

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 文档信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    /// 文档ID
    pub document_id: String,
    /// 文档版本ID
    pub document_revision_id: i64,
    /// 文档标题
    pub title: String,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    /// 最后编辑者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_editor: Option<Creator>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            document_id: String::new(),
            document_revision_id: 0,
            title: String::new(),
            create_time: None,
            update_time: None,
            creator: None,
            last_editor: None,
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

/// 文档块信息
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
}

impl Default for Block {
    fn default() -> Self {
        Self {
            block_id: String::new(),
            parent_id: None,
            children: None,
            block_type: None,
            index: None,
        }
    }
}

/// DocumentService v1版本服务
///
/// 提供飞书云文档v1版本的统一入口，支持现代化的文档管理。
/// 包括创建、查询、转换等企业级功能。
#[derive(Debug, Clone)]
pub struct DocumentService {
    config: Config,
}

impl DocumentService {
    /// 创建Document v1服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::document::DocumentService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = DocumentService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl crate::core::trait_system::Service for DocumentService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "DocumentService"
    }
}

// ==================== 创建文档 ====================

/// 创建文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRequest {
    /// 文档标题
    pub title: String,
    /// 文档内容，JSON格式的块内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

impl CreateDocumentRequest {
    /// 创建新的文档请求实例
    ///
    /// # 参数
    /// - `title`: 文档标题
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::docx::v1::document::CreateDocumentRequest;
    ///
    /// let request = CreateDocumentRequest::new("新文档标题");
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: None,
            folder_token: None,
        }
    }

    /// 设置文档内容
    ///
    /// # 参数
    /// - `content`: JSON格式的块内容
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document::CreateDocumentRequest;
    ///
    /// let mut request = CreateDocumentRequest::new("标题");
    /// request.set_content(r#"[{"type":"text","text":"内容"}]"#);
    /// ```
    pub fn set_content(&mut self, content: impl Into<String>) -> &mut Self {
        self.content = Some(content.into());
        self
    }

    /// 设置文件夹token
    ///
    /// # 参数
    /// - `folder_token`: 文件夹token
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document::CreateDocumentRequest;
    ///
    /// let mut request = CreateDocumentRequest::new("标题");
    /// request.set_folder_token("folder_token_123");
    /// ```
    pub fn set_folder_token(&mut self, folder_token: impl Into<String>) -> &mut Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 验证通过
    /// - `Err(String)`: 验证失败，返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("文档标题不能为空".to_string());
        }

        if self.title.len() > 200 {
            return Err("文档标题长度不能超过200个字符".to_string());
        }

        Ok(())
    }
}

/// 创建文档响应数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDocumentResponseData {
    /// 文档ID
    pub document_id: String,
    /// 文档版本ID
    pub document_revision_id: i64,
    /// 文档标题
    pub title: String,
}

/// 创建文档响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDocumentResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateDocumentResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 获取文档基本信息 ====================

/// 获取文档基本信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRequest {
    /// 文档ID
    pub document_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetDocumentRequest {
    /// 创建新的获取文档请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::docx::v1::document::GetDocumentRequest;
    ///
    /// let request = GetDocumentRequest::new("doc_123");
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型（open_id、user_id、union_id）
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document::GetDocumentRequest;
    ///
    /// let mut request = GetDocumentRequest::new("doc_123");
    /// request.set_user_id_type("open_id");
    /// ```
    pub fn set_user_id_type(&mut self, user_id_type: impl Into<String>) -> &mut Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 验证通过
    /// - `Err(String)`: 验证失败，返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }

        if let Some(ref user_id_type) = user_id_type.trim() {
            if user_id_type.is_empty() {
                return Err("用户ID类型不能为空".to_string());
            }
        }

        Ok(())
    }
}

/// 获取文档基本信息响应数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDocumentResponseData {
    /// 文档信息
    pub document: Document,
}

/// 获取文档基本信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDocumentResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetDocumentResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for GetDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 获取文档纯文本内容 ====================

/// 获取文档纯文本内容请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRawContentRequest {
    /// 文档ID
    pub document_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetRawContentRequest {
    /// 创建新的获取纯文本内容请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::docx::v1::document::GetRawContentRequest;
    ///
    /// let request = GetRawContentRequest::new("doc_123");
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
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
    /// # use open_lark::service::cloud_docs::docx::v1::document::GetRawContentRequest;
    ///
    /// let mut request = GetRawContentRequest::new("doc_123");
    /// request.set_user_id_type("open_id");
    /// ```
    pub fn set_user_id_type(&mut self, user_id_type: impl Into<String>) -> &mut Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 验证通过
    /// - `Err(String)`: 验证失败，返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }

        if let Some(ref user_id_type) = user_id_type.trim() {
            if user_id_type.is_empty() {
                return Err("用户ID类型不能为空".to_string());
            }
        }

        Ok(())
    }
}

/// 获取文档纯文本内容响应数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRawContentResponseData {
    /// 纯文本内容
    pub content: String,
}

/// 获取文档纯文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRawContentResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetRawContentResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for GetRawContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 获取文档所有块 ====================

/// 获取文档所有块请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDocumentBlocksRequest {
    /// 文档ID
    pub document_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl ListDocumentBlocksRequest {
    /// 创建新的获取文档所有块请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::docx::v1::document::ListDocumentBlocksRequest;
    ///
    /// let request = ListDocumentBlocksRequest::new("doc_123");
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置分页大小
    ///
    /// # 参数
    /// - `page_size`: 分页大小，范围1-100
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document::ListDocumentBlocksRequest;
    ///
    /// let mut request = ListDocumentBlocksRequest::new("doc_123");
    /// request.set_page_size(50);
    /// ```
    pub fn set_page_size(&mut self, page_size: i32) -> &mut Self {
        if page_size > 0 && page_size <= 100 {
            self.page_size = Some(page_size);
        }
        self
    }

    /// 设置分页标记
    ///
    /// # 参数
    /// - `page_token`: 分页标记
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document::ListDocumentBlocksRequest;
    ///
    /// let mut request = ListDocumentBlocksRequest::new("doc_123");
    /// request.set_page_token("next_page_token");
    /// ```
    pub fn set_page_token(&mut self, page_token: impl Into<String>) -> &mut Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::cloud_docs::docx::v1::document::ListDocumentBlocksRequest;
    ///
    /// let mut request = ListDocumentBlocksRequest::new("doc_123");
    /// request.set_user_id_type("open_id");
    /// ```
    pub fn set_user_id_type(&mut self, user_id_type: impl Into<String>) -> &mut Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 验证通过
    /// - `Err(String)`: 验证失败，返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }

        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err("分页大小必须在1-100之间".to_string());
            }
        }

        if let Some(ref user_id_type) = user_id_type.trim() {
            if user_id_type.is_empty() {
                return Err("用户ID类型不能为空".to_string());
            }
        }

        Ok(())
    }
}

/// 获取文档所有块响应数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListDocumentBlocksResponseData {
    /// 块列表
    pub items: Vec<Block>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 获取文档所有块响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListDocumentBlocksResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListDocumentBlocksResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for ListDocumentBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 转换为文档块 ====================

/// 转换为文档块请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertToDocxRequest {
    /// 文档ID
    pub document_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl ConvertToDocxRequest {
    /// 创建新的转换请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::docx::v1::document::ConvertToDocxRequest;
    ///
    /// let request = ConvertToDocxRequest::new("doc_123");
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
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
    /// # use open_lark::service::cloud_docs::docx::v1::document::ConvertToDocxRequest;
    ///
    /// let mut request = ConvertToDocxRequest::new("doc_123");
    /// request.set_user_id_type("open_id");
    /// ```
    pub fn set_user_id_type(&mut self, user_id_type: impl Into<String>) -> &mut Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 验证通过
    /// - `Err(String)`: 验证失败，返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }

        if let Some(ref user_id_type) = user_id_type.trim() {
            if user_id_type.is_empty() {
                return Err("用户ID类型不能为空".to_string());
            }
        }

        Ok(())
    }
}

/// 转换为文档块响应数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConvertToDocxResponseData {
    /// 新文档ID
    pub document_id: String,
    /// 文档版本ID
    pub document_revision_id: i64,
}

/// 转换为文档块响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConvertToDocxResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ConvertToDocxResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for ConvertToDocxResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 服务实现 ====================

impl DocumentService {
    /// 创建文档
    ///
    /// 创建一个新的云文档，支持指定标题、内容和文件夹位置
    ///
    /// # 参数
    /// * `req` - 创建文档请求
    ///
    /// # 返回值
    /// 返回创建的文档信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::document::{
    ///     DocumentService, CreateDocumentRequest
    /// };
    ///
    /// let service = DocumentService::new(config);
    /// let request = CreateDocumentRequest::new("新文档标题")
    ///     .set_content(r#"[{"type":"text","text":"文档内容"}]"#)
    ///     .set_folder_token("folder_token_123");
    ///
    /// let result = service.create(&request).await?;
    /// println!("创建成功，文档ID: {}", result.data.unwrap().document_id);
    /// ```
    pub async fn create(
        &self,
        req: &CreateDocumentRequest,
    ) -> SDKResult<CreateDocumentResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始创建文档: title={}", req.title);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::DOCX_V1_DOCUMENTS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<CreateDocumentResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            if let Some(ref data) = response.data {
                log::info!(
                    "文档创建成功: title={}, document_id={}",
                    req.title,
                    data.document_id
                );
            }
        } else {
            log::warn!(
                "文档创建失败: title={}, error={:?}",
                req.title,
                response.error_message
            );
        }

        Ok(response)
    }

    /// 获取文档基本信息
    ///
    /// 根据文档ID查询文档的基本信息，包括标题、创建时间、更新时间等
    ///
    /// # 参数
    /// * `req` - 获取文档基本信息请求
    ///
    /// # 返回值
    /// 返回文档的基本信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::document::{
    ///     DocumentService, GetDocumentRequest
    /// };
    ///
    /// let service = DocumentService::new(config);
    /// let request = GetDocumentRequest::new("doc_123")
    ///     .set_user_id_type("open_id");
    ///
    /// let result = service.get(&request).await?;
    /// println!("文档标题: {}", result.data.unwrap().document.title);
    /// ```
    pub async fn get(
        &self,
        req: &GetDocumentRequest,
    ) -> SDKResult<GetDocumentResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始获取文档基本信息: document_id={}", req.document_id);

        // 构建API路径
        let endpoint = crate::core::endpoints_original::Endpoints::DOCX_V1_DOCUMENT_GET
            .replace("{}", &req.document_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: endpoint,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(), // GET请求无body
            ..Default::default()
        };

        let resp = Transport::<GetDocumentResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            if let Some(ref data) = response.data {
                log::info!(
                    "文档信息获取完成: document_id={}, title={}",
                    req.document_id,
                    data.document.title
                );
            }
        } else {
            log::warn!(
                "文档信息获取失败: document_id={}, error={:?}",
                req.document_id,
                response.error_message
            );
        }

        Ok(response)
    }

    /// 获取文档纯文本内容
    ///
    /// 获取文档的纯文本格式内容，适用于文本处理和分析
    ///
    /// # 参数
    /// * `req` - 获取纯文本内容请求
    ///
    /// # 返回值
    /// 返回文档的纯文本内容
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::document::{
    ///     DocumentService, GetRawContentRequest
    /// };
    ///
    /// let service = DocumentService::new(config);
    /// let request = GetRawContentRequest::new("doc_123");
    ///
    /// let result = service.get_raw_content(&request).await?;
    /// println!("文档纯文本内容: {}", result.data.unwrap().content);
    /// ```
    pub async fn get_raw_content(
        &self,
        req: &GetRawContentRequest,
    ) -> SDKResult<GetRawContentResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始获取文档纯文本内容: document_id={}", req.document_id);

        // 构建API路径
        let endpoint = crate::core::endpoints_original::Endpoints::DOCX_V1_DOCUMENT_RAW_CONTENT
            .replace("{}", &req.document_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: endpoint,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(), // GET请求无body
            ..Default::default()
        };

        let resp = Transport::<GetRawContentResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            if let Some(ref data) = response.data {
                log::info!(
                    "文档纯文本内容获取完成: document_id={}, length={}",
                    req.document_id,
                    data.content.len()
                );
            }
        } else {
            log::warn!(
                "文档纯文本内容获取失败: document_id={}, error={:?}",
                req.document_id,
                response.error_message
            );
        }

        Ok(response)
    }

    /// 获取文档所有块
    ///
    /// 获取文档中的所有块信息，支持分页查询
    ///
    /// # 参数
    /// * `req` - 获取文档所有块请求
    ///
    /// # 返回值
    /// 返回文档块列表信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::document::{
    ///     DocumentService, ListDocumentBlocksRequest
    /// };
    ///
    /// let service = DocumentService::new(config);
    /// let request = ListDocumentBlocksRequest::new("doc_123")
    ///     .set_page_size(50)
    ///     .set_user_id_type("open_id");
    ///
    /// let result = service.list_blocks(&request).await?;
    /// for block in &result.data.as_ref().unwrap().items {
    ///     println!("块ID: {}, 类型: {}", block.block_id, block.block_type);
    /// }
    /// ```
    pub async fn list_blocks(
        &self,
        req: &ListDocumentBlocksRequest,
    ) -> SDKResult<ListDocumentBlocksResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始获取文档所有块: document_id={}", req.document_id);

        // 构建查询参数
        let mut query_params = Vec::new();
        if let Some(ref page_size) = req.page_size {
            query_params.push(("page_size", page_size.to_string()));
        }
        if let Some(ref page_token) = req.page_token {
            query_params.push(("page_token", page_token.clone()));
        }
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.push(("user_id_type", user_id_type.clone()));
        }

        // 构建API路径
        let endpoint = crate::core::endpoints_original::Endpoints::DOCX_V1_DOCUMENT_BLOCKS
            .replace("{}", &req.document_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: endpoint,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: Vec::new(), // GET请求无body
            ..Default::default()
        };

        let resp = Transport::<ListBlocksResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            if let Some(ref data) = response.data {
                log::info!(
                    "文档块列表获取完成: document_id={}, count={}, has_more={:?}",
                    req.document_id,
                    data.items.len(),
                    data.has_more
                );
            }
        } else {
            log::warn!(
                "文档块列表获取失败: document_id={}, error={:?}",
                req.document_id,
                response.error_message
            );
        }

        Ok(response)
    }

    /// 转换为文档块格式
    ///
    /// 将旧版文档转换为新版文档块格式
    ///
    /// # 参数
    /// * `req` - 转换请求
    ///
    /// # 返回值
    /// 返回转换后的文档信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::document::{
    ///     DocumentService, ConvertToDocxRequest
    /// };
    ///
    /// let service = DocumentService::new(config);
    /// let request = ConvertToDocxRequest::new("doc_123");
    ///
    /// let result = service.convert_to_docx(&request).await?;
    /// println!("转换成功，新文档ID: {}", result.data.unwrap().document_id);
    /// ```
    pub async fn convert_to_docx(
        &self,
        req: &ConvertToDocxRequest,
    ) -> SDKResult<ConvertToDocxResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始转换为文档块格式: document_id={}", req.document_id);

        // 构建API路径
        let endpoint = crate::core::endpoints_original::Endpoints::DOCX_V1_DOCUMENT_CONVERT
            .replace("{}", &req.document_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: endpoint,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(), // POST请求无特定body
            ..Default::default()
        };

        let resp = Transport::<ConvertToDocxResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            if let Some(ref data) = response.data {
                log::info!(
                    "文档转换完成: document_id={}, new_document_id={}",
                    req.document_id,
                    data.document_id
                );
            }
        } else {
            log::warn!(
                "文档转换失败: document_id={}, error={:?}",
                req.document_id,
                response.error_message
            );
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 获取文档基本信息构建器
#[derive(Debug, Clone)]
pub struct GetDocumentBuilder {
    request: GetDocumentRequest,
}

impl Default for GetDocumentBuilder {
    fn default() -> Self {
        Self {
            request: GetDocumentRequest {
                document_id: String::new(),
                user_id_type: None,
            },
        }
    }
}

impl GetDocumentBuilder {
    /// 创建新的构建器
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::cloud_docs::docx::v1::document::GetDocumentBuilder;
    ///
    /// let builder = GetDocumentBuilder::new("doc_token_123");
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            request: GetDocumentRequest::new(document_id),
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
    /// # use open_lark::service::cloud_docs::docx::v1::document::GetDocumentBuilder;
    ///
    /// let builder = GetDocumentBuilder::new("doc_token_123")
    ///     .user_id_type("open_id");
    /// ```
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.set_user_id_type(user_id_type);
        self
    }

    /// 执行获取文档基本信息操作
    ///
    /// # 参数
    /// - `service`: 文档服务实例
    ///
    /// # 返回值
    /// 返回文档基本信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::document::{
    ///     DocumentService, GetDocumentBuilder
    /// };
    ///
    /// let service = DocumentService::new(config);
    ///
    /// let result = GetDocumentBuilder::new("doc_token_123")
    ///     .user_id_type("open_id")
    ///     .execute(&service)
    ///     .await?;
    /// println!("文档标题: {}", result.data.unwrap().document.title);
    /// ```
    pub async fn execute(self, service: &DocumentService) -> SDKResult<GetDocumentResponse> {
        service.get(&self.request).await
    }
}

impl DocumentService {
    /// 创建获取文档基本信息构建器
    ///
    /// # 返回值
    /// 返回获取构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::docx::v1::document::DocumentService;
    ///
    /// let service = DocumentService::new(config);
    /// let builder = service.get_document_builder("doc_token_123");
    /// ```
    pub fn get_document_builder(&self, document_id: impl Into<String>) -> GetDocumentBuilder {
        GetDocumentBuilder::new(document_id)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::trait_system::Service;

    #[test]
    fn test_document_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocumentService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_trait_implementation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DocumentService::new(config);

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
        assert_eq!(DocumentService::service_name(), "DocumentService");
    }

    #[test]
    fn test_create_document_request_creation() {
        let request = CreateDocumentRequest::new("测试文档");
        assert_eq!(request.title, "测试文档");
        assert_eq!(request.content, None);
        assert_eq!(request.folder_token, None);
    }

    #[test]
    fn test_create_document_request_with_fields() {
        let mut request = CreateDocumentRequest::new("测试文档");
        request.set_content(r#"[{"type":"text","text":"内容"}]"#);
        request.set_folder_token("folder_token_123");

        assert_eq!(request.title, "测试文档");
        assert_eq!(request.content, Some(r#"[{"type":"text","text":"内容"}]"#));
        assert_eq!(request.folder_token, Some("folder_token_123"));
    }

    #[test]
    fn test_create_document_request_validation() {
        let request = CreateDocumentRequest::new("测试文档");
        assert!(request.validate().is_ok());

        let empty_title_request = CreateDocumentRequest::new("");
        let result = empty_title_request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "文档标题不能为空");

        let long_title_request = CreateDocumentRequest::new(&"a".repeat(201));
        let result = long_title_request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "文档标题长度不能超过200个字符");
    }

    #[test]
    fn test_get_document_request_creation() {
        let request = GetDocumentRequest::new("doc_token_123");
        assert_eq!(request.document_id, "doc_token_123");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_get_document_request_with_fields() {
        let mut request = GetDocumentRequest::new("doc_token_123");
        request.set_user_id_type("open_id");

        assert_eq!(request.document_id, "doc_token_123");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_document_request_validation() {
        let request = GetDocumentRequest::new("doc_token_123");
        assert!(request.validate().is_ok());

        let empty_doc_request = GetDocumentRequest::new("");
        let result = empty_doc_request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "文档ID不能为空");

        let empty_type_request = GetDocumentRequest::new("doc_token_123");
        let mut request = empty_type_request;
        request.set_user_id_type("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_get_raw_content_request_creation() {
        let request = GetRawContentRequest::new("doc_token_123");
        assert_eq!(request.document_id, "doc_token_123");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_get_raw_content_request_with_fields() {
        let mut request = GetRawContentRequest::new("doc_token_123");
        request.set_user_id_type("union_id");

        assert_eq!(request.document_id, "doc_token_123");
        assert_eq!(request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_get_raw_content_request_validation() {
        let request = GetRawContentRequest::new("doc_token_123");
        assert!(request.validate().is_ok());

        let empty_doc_request = GetRawContentRequest::new("");
        let result = empty_doc_request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "文档ID不能为空");

        let empty_type_request = GetRawContentRequest::new("doc_token_123");
        let mut request = empty_type_request;
        request.set_user_id_type("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_list_document_blocks_request_creation() {
        let request = ListDocumentBlocksRequest::new("doc_token_123");
        assert_eq!(request.document_id, "doc_token_123");
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_list_document_blocks_request_with_fields() {
        let mut request = ListDocumentBlocksRequest::new("doc_token_123");
        request.set_page_size(50);
        request.set_page_token("next_page_token");
        request.set_user_id_type("union_id");

        assert_eq!(request.document_id, "doc_token_123");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
        assert_eq!(request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_list_document_blocks_request_validation() {
        let request = ListDocumentBlocksRequest::new("doc_token_123");
        assert!(request.validate().is_ok());

        let empty_doc_request = ListDocumentBlocksRequest::new("");
        let result = empty_doc_request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "文档ID不能为空");

        let small_page_size_request = ListDocumentRequest::new("doc_token_123");
        let mut request = small_page_size_request;
        request.set_page_size(0);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "分页大小必须在1-100之间");

        let large_page_size_request = ListDocumentRequest::new("doc_token_123");
        let mut request = large_page_size_request;
        request.set_page_size(101);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "分页大小必须在1-100之间");

        let empty_type_request = ListDocumentRequest::new("doc_token_123");
        let mut request = empty_type_request;
        request.set_user_id_type("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_convert_to_docx_request_creation() {
        let request = ConvertToDocxRequest::new("doc_token_123");
        assert_eq!(request.document_id, "doc_token_123");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_convert_to_docx_request_with_fields() {
        let mut request = ConvertToDocxRequest::new("doc_token_123");
        request.set_user_id_type("union_id");

        assert_eq!(request.document_id, "doc_token_123");
        assert_eq!(request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_convert_to_docx_request_validation() {
        let request = ConvertToDocxRequest::new("doc_token_123");
        assert!(request.validate().is_ok());

        let empty_doc_request = ConvertToDocxRequest::new("");
        let result = empty_doc_request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "文档ID不能为空");

        let empty_type_request = ConvertToDocxRequest::new("doc_token_123");
        let mut request = empty_type_request;
        request.set_user_id_type("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_document_default_creation() {
        let document = Document::default();
        assert_eq!(document.document_id, "");
        assert_eq!(document.document_revision_id, 0);
        assert_eq!(document.title, "");
        assert_eq!(document.create_time, None);
        assert_eq!(document.update_time, None);
        assert_eq!(document.creator, None);
        assert_eq!(document.last_editor, None);
    }

    #[test]
    fn test_creator_default_creation() {
        let creator = Creator::default();
        assert_eq!(creator.user_id, None);
        assert_eq!(creator.name, None);
        assert_eq!(creator.avatar, None);
    }

    #[test]
    fn test_block_default_creation() {
        let block = Block::default();
        assert_eq!(block.block_id, "");
        assert_eq!(block.parent_id, None);
        assert_eq!(block.children, None);
        assert_eq!(block.block_type, None);
        assert_eq!(block.index, None);
    }

    #[test]
    fn test_create_document_response_creation() {
        let document = Document {
            document_id: "doc_123".to_string(),
            document_revision_id: 456,
            title: "测试文档".to_string(),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        let data = CreateDocumentResponseData {
            document_id: "doc_123".to_string(),
            document_revision_id: 456,
            title: "测试文档".to_string(),
        };

        let response = CreateDocumentResponse {
            data: Some(data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert_eq!(response.data.unwrap().document_id, "doc_123");
        assert_eq!(response.data.unwrap().title, "测试文档");
    }

    #[test]
    fn test_get_document_response_creation() {
        let document = Document {
            document_id: "doc_456".to_string(),
            document_revision_id: 789,
            title: "测试文档".to_string(),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-15T00:00:00Z".to_string()),
        };

        let data = GetDocumentResponseData {
            document: document,
        };

        let response = GetDocumentResponse {
            data: Some(data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert_eq!(response.data.unwrap().document.document_id, "doc_456");
        assert_eq!(response.data.unwrap().document.title, "测试文档");
    }

    #[test]
    fn test_get_raw_content_response_creation() {
        let data = GetRawContentResponseData {
            content: "这是文档的纯文本内容".to_string(),
        };

        let response = GetRawContentResponse {
            data: Some(data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert_eq!(response.data.unwrap().content, "这是文档的纯文本内容");
    }

    #[test]
    fn test_list_document_blocks_response_creation() {
        let block1 = Block {
            block_id: "block_123".to_string(),
            block_type: Some(1),
            index: Some(0),
            ..Default::default()
        };

        let block2 = Block {
            block_id: "block_456".to_string(),
            parent_id: Some("block_123".to_string()),
            children: Some(vec!["child_123".to_string()]),
            block_type: Some(2),
            index: Some(1),
            ..Default::default()
        };

        let data = ListDocumentBlocksResponseData {
            items: vec![block1, block2],
            has_more: Some(true),
            page_token: Some("next_token".to_string()),
        };

        let response = ListDocumentBlocksResponse {
            data: Some(data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert_eq!(response.data.unwrap().items.len(), 2);
        assert_eq!(response.data.unwrap().items[0].block_id, "block_123");
        assert_eq!(response.data.unwrap().items[1].parent_id, Some("block_123".to_string()));
    }

    #[test]
    fn test_convert_to_docx_response_creation() {
        let data = ConvertToDocxResponseData {
            document_id: "new_doc_789".to_string(),
            document_revision_id: 1000,
        };

        let response = ConvertToDocxResponse {
            data: Some(data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert_eq!(response.data.unwrap().document_id, "new_doc_789");
        assert_eq!(response.data.unwrap().document_revision_id, 1000);
    }

    #[test]
    fn test_get_document_builder() {
        let builder = GetDocumentBuilder::new("doc_token_123")
            .user_id_type("union_id");

        assert_eq!(builder.request.document_id, "doc_token_123");
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_get_document_builder_default() {
        let builder = GetDocumentBuilder::default();
        assert_eq!(builder.request.document_id, "");
        assert_eq!(builder.request.user_id_type, None);
    }

    #[test]
    fn test_create_document_builder() {
        let builder = CreateDocumentRequest::new("测试文档")
            .content(r#"[{"type":"text","text":"内容"}]"#)
            .folder_token("folder_token_123");

        assert_eq!(builder.request.title, "测试文档");
        assert_eq!(builder.request.content, Some(r#"[{"type":"text","text":"内容"}]"#));
        assert_eq!(builder.request.folder_token, Some("folder_token_123"));
    }

    #[test]
    fn test_response_trait_implementation() {
        assert_eq!(CreateDocumentResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetDocumentResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetRawContentResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ListDocumentBlocksResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ConvertToDocxResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = CreateDocumentRequest {
            title: "序列化测试".to_string(),
            content: Some(r#"[{"type":"text","text":"内容"}]"#),
            folder_token: Some("folder_token".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateDocumentRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.title, deserialized.title);
        assert_eq!(request.content, deserialized.content);
        assert_eq!(request.folder_token, deserialized.folder_token);
    }

    #[test]
    fn test_response_serialization() {
        let mut response = GetDocumentResponse::default();
        response.data = Some(GetDocumentResponseData {
            document: Document {
                document_id: "doc_456".to_string(),
                document_revision_id: 789,
                title: "序列化测试".to_string(),
                create_time: Some("2023-01-01T00:00:00Z".to_string()),
                update_time: Some("2023-01-02T00:00:00Z".to_string()),
            },
        });
        response.success = true;

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetDocumentResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.success, deserialized.success);
        assert_eq!(response.data.unwrap().document.document_id, "doc_456");
        assert_eq!(response.data.unwrap().document.title, "序列化测试");
    }

    #[test]
    fn test_comprehensive_scenario() {
        // 测试完整的文档生命周期
        let create_request = CreateDocumentRequest::new("完整文档测试")
            .content(r#"[{"type":"text","text":"第一段"},{"type":"heading","text":"标题"}]"#)
            .folder_token("test_folder");

        assert!(create_request.validate().is_ok());
        assert_eq!(create_request.title, "完整文档测试");
        assert_eq!(create_request.content.unwrap(), r#"[{"type":"text","text":"第一段"},{"type":"heading","text":"标题"}]"#);

        // 验证文档信息字段
        let document = Document {
            document_id: "doc_complete".to_string(),
            document_revision_id: 123,
            title: "完整文档测试".to_string(),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
            creator: Some(Creator {
                user_id: Some("user_123".to_string()),
                name: Some("张三".to_string()),
                avatar: Some("avatar_url".to_string()),
            }),
            last_editor: Some(Creator {
                user_id: Some("user_456".to_string()),
                name: Some("李四".to_string()),
                avatar: Some("new_avatar_url".to_string()),
            }),
        };

        // 验证所有字段都正确设置
        assert_eq!(document.document_id, "doc_complete");
        assert_eq!(document.document_revision_id, 123);
        assert_eq!(document.title, "完整文档测试");
        assert_eq!(document.create_time, Some("2023-01-01T00:00:00Z"));
        assert_eq!(document.update_time, Some("2023-01-02T00:00:00"));
        assert_eq!(document.creator.as_ref().unwrap().user_id, Some("user_123"));
        assert_eq!(document.creator.as_ref().unwrap().name, Some("张三"));
        assert_eq!(document.last_editor.as_ref().unwrap().user_id, Some("user_456"));
    }
}

#[cfg(test)]
mod builder_tests {
    use super::*;

    #[test]
    fn test_create_document_builder() {
        let builder = CreateDocumentRequestBuilder::new("builder测试")
            .content(r#"[{"type":"text","text":"内容"}]"#)
            .folder_token("folder_token");

        assert_eq!(builder.request.title, "builder测试");
        assert_eq!(builder.request.content.unwrap(), r#"[{"type":"text","text":"内容"}]"#);
        assert_eq!(builder.request.folder_token, Some("folder_token"));
    }

    #[test]
    fn test_get_document_builder() {
        let builder = GetDocumentBuilder::new("doc_token_123")
            .user_id_type("union_id");

        assert_eq!(builder.request.document_id, "doc_token_123");
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_list_document_blocks_builder() {
        let builder = ListDocumentBlocksRequestBuilder::new("doc_token_123")
            .page_size(50)
            .page_token("next_token")
            .user_id_type("union_id");

        assert_eq!(builder.request.document_id, "doc_token_123");
        assert_eq!(builder.request.page_size, Some(50));
        assert_eq!(builder.request.page_token, Some("next_token".to_string()));
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_builder_default() {
        let create_builder = CreateDocumentRequestBuilder::default();
        let get_builder = GetDocumentBuilder::default();
        let list_blocks_builder = ListDocumentBlocksRequestBuilder::default();

        assert_eq!(create_builder.request.title, "");
        assert_eq!(create_builder.request.content, None);
        assert_eq!(create_builder.request.folder_token, None);
        assert_eq!(get_builder.request.document_id, "");
        assert_eq!(get_builder.request.user_id_type, None);
        assert_eq!(list_blocks_builder.request.document_id, "");
        assert_eq!(list_blocks_builder.request.page_size, None);
        assert_eq!(list_blocks_builder.request.page_token, None);
        assert_eq!(list_blocks_builder.request.user_id_type, None);
    }
}
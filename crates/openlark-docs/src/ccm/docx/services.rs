//! ccm_docx API 服务实现
//!
//! 提供完整的文档(DocX)功能实现，包括：
//! - ccm_docs: 云文档搜索和元数据 (已实现)
//! - docx: 新版文档操作、块操作、群公告等 (新增)

use crate::prelude::*;
use openlark_core::{api::ApiRequest, constants::AccessTokenType, http::Transport, SDKResult};

// 导入ccm_docs API (已实现)
use super::models::{GetDocMetaRequest, GetDocMetaResponse, SearchDocsRequest, SearchDocsResponse};

// 导入docx相关的模型 (待创建)
use super::models_docx::{
    BatchUpdateBlocksRequest, BatchUpdateBlocksResponse, BatchUpdateChatAnnouncementBlocksRequest,
    BatchUpdateChatAnnouncementBlocksResponse, ChatAnnouncementRequest, ChatAnnouncementResponse,
    ConvertContentRequest, ConvertContentResponse, CreateBlockChildrenRequest,
    CreateBlockChildrenResponse, CreateBlockDescendantRequest, CreateBlockDescendantResponse,
    CreateChatAnnouncementBlockChildrenRequest, CreateChatAnnouncementBlockChildrenResponse,
    CreateDocumentRequest, CreateDocumentResponse, DeleteBlockChildrenRequest,
    DeleteBlockChildrenResponse, DeleteChatAnnouncementBlockChildrenRequest,
    DeleteChatAnnouncementBlockChildrenResponse, GetBlockChildrenRequest, GetBlockChildrenResponse,
    GetBlockRequest, GetBlockResponse, GetChatAnnouncementBlockChildrenRequest,
    GetChatAnnouncementBlockChildrenResponse, GetChatAnnouncementBlockRequest,
    GetChatAnnouncementBlockResponse, GetDocumentRawContentRequest, GetDocumentRawContentResponse,
    GetDocumentRequest, GetDocumentResponse, ListChatAnnouncementBlocksRequest,
    ListChatAnnouncementBlocksResponse, ListDocumentBlocksRequest, ListDocumentBlocksResponse,
    UpdateBlockRequest, UpdateBlockResponse,
};

/// ccm_docs API 服务
#[derive(Clone, Debug)]
pub struct CcmDocsService {
    config: Config,
}

impl CcmDocsService {
    /// 创建ccm_docs服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 搜索云文档
    ///
    /// 根据搜索条件在用户的云文档中进行搜索。
    /// 支持按文档类型、搜索范围、排序规则等进行筛选和排序。
    ///
    /// # 参数
    /// - `req`: 搜索文档请求
    ///
    /// # 返回
    /// - `SDKResult<SearchDocsResponse>`: 搜索结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ccm::docx::{CcmDocsService, SearchDocsRequest, SearchScope, SortRule, SortDirection};
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret");
    ///     let service = CcmDocsService::new(config);
    ///
    ///     let request = SearchDocsRequest {
    ///         search_key: "项目文档".to_string(),
    ///         doc_types: Some(vec!["doc".to_string(), "sheet".to_string()]),
    ///         search_scope: Some(SearchScope::CreatedByMe),
    ///         sort: Some(SortRule {
    ///             sort_field: "modify_time".to_string(),
    ///             sort_direction: SortDirection::Desc,
    ///         }),
    ///         page_size: Some(20),
    ///         page_token: None,
    ///     };
    ///
    ///     let response = service.search_documents(&request).await?;
    ///     if let Some(items) = response.items {
    ///         println!("找到 {} 个文档:", items.len());
    ///         for item in items {
    ///             println!("- {} ({})", item.title.unwrap_or_default(), item.doc_type.unwrap_or_default());
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn search_documents(&self, req: &SearchDocsRequest) -> SDKResult<SearchDocsResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始搜索云文档: search_key={}, doc_types={:?}, search_scope={:?}",
            req.search_key,
            req.doc_types,
            req.search_scope
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 文档类型参数
        if let Some(ref doc_types) = req.doc_types {
            query_params.insert("doc_types", doc_types.join(","));
        }

        // 搜索范围参数
        if let Some(ref search_scope) = req.search_scope {
            match search_scope {
                SearchScope::Folder(folder_token) => {
                    query_params.insert("search_scope", format!("folder:{}", folder_token));
                }
                _ => {
                    query_params
                        .insert("search_scope", format!("{:?}", search_scope).to_lowercase());
                }
            }
        }

        // 排序参数
        if let Some(ref sort) = req.sort {
            query_params.insert("sort_field", sort.sort_field.clone());
            query_params.insert(
                "sort_direction",
                format!("{:?}", sort.sort_direction).to_lowercase(),
            );
        }

        // 分页参数
        if let Some(page_size) = req.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = req.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("search_key".to_string(), req.search_key.clone());

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/suite/docs-api/search/object".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp = Transport::<SearchDocsResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "云文档搜索完成: search_key={}, result_count={}, has_more={}",
            req.search_key,
            response
                .items
                .as_ref()
                .map(|items| items.len())
                .unwrap_or(0),
            response.has_more.unwrap_or(false)
        );

        Ok(response)
    }

    /// 获取文档元数据
    ///
    /// 批量获取指定文档的详细元数据信息，包括基础信息、
    /// 统计信息、权限信息等。
    ///
    /// # 参数
    /// - `req`: 获取元数据请求
    ///
    /// # 返回
    /// - `SDKResult<GetDocMetaResponse>`: 文档元数据
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ccm::docx::{CcmDocsService, GetDocMetaRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret");
    ///     let service = CcmDocsService::new(config);
    ///
    ///     let request = GetDocMetaRequest {
    ///         tokens: vec!["doc_token_1".to_string(), "doc_token_2".to_string()],
    ///         extra_fields: Some(vec!["statistics".to_string()]),
    ///     };
    ///
    ///     let response = service.get_documents_meta(&request).await?;
    ///     if let Some(items) = response.items {
    ///         println!("获取到 {} 个文档的元数据:", items.len());
    ///         for item in items {
    ///             if let Some(ref basic_info) = item.basic_info {
    ///                 println!("- {} ({})",
    ///                    basic_info.title.as_ref().unwrap_or(&"无标题".to_string()),
    ///                    basic_info.doc_type.as_ref().unwrap_or(&"unknown".to_string())
    ///                 );
    ///             }
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_documents_meta(
        &self,
        req: &GetDocMetaRequest,
    ) -> SDKResult<GetDocMetaResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!("开始获取文档元数据: token_count={}", req.tokens.len());

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 扩展字段参数
        if let Some(ref extra_fields) = req.extra_fields {
            query_params.insert("extra_fields", extra_fields.join(","));
        }

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("tokens".to_string(), req.tokens.clone());

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/suite/docs-api/meta".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp = Transport::<GetDocMetaResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档元数据获取完成: requested_count={}, result_count={}",
            req.tokens.len(),
            response
                .items
                .as_ref()
                .map(|items| items.len())
                .unwrap_or(0)
        );

        Ok(response)
    }
}

/// 搜索文档构建器
///
/// 提供构建器模式搜索云文档，支持链式调用。
#[derive(Debug, Clone)]
pub struct SearchDocumentsBuilder {
    search_key: Option<String>,
    doc_types: Option<Vec<String>>,
    search_scope: Option<SearchScope>,
    sort: Option<SortRule>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

use super::models::{SearchScope, SortDirection, SortRule};

impl SearchDocumentsBuilder {
    /// 创建新的搜索构建器实例
    pub fn new() -> Self {
        Self {
            search_key: None,
            doc_types: None,
            search_scope: None,
            sort: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置搜索关键字
    pub fn search_key(mut self, search_key: impl Into<String>) -> Self {
        self.search_key = Some(search_key.into());
        self
    }

    /// 设置文档类型筛选
    pub fn doc_types(mut self, doc_types: Vec<String>) -> Self {
        self.doc_types = Some(doc_types);
        self
    }

    /// 添加文档类型
    pub fn add_doc_type(mut self, doc_type: impl Into<String>) -> Self {
        let mut types = self.doc_types.unwrap_or_default();
        types.push(doc_type.into());
        self.doc_types = Some(types);
        self
    }

    /// 设置搜索范围
    pub fn search_scope(mut self, search_scope: SearchScope) -> Self {
        self.search_scope = Some(search_scope);
        self
    }

    /// 设置排序规则
    pub fn sort(mut self, sort_field: impl Into<String>, sort_direction: SortDirection) -> Self {
        self.sort = Some(SortRule {
            sort_field: sort_field.into(),
            sort_direction,
        });
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页令牌
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行搜索操作
    pub async fn execute(self, service: &CcmDocsService) -> SDKResult<SearchDocsResponse> {
        let search_key = self.search_key.ok_or_else(|| {
            openlark_core::error::LarkAPIError::illegal_param("搜索关键字是必需的")
        })?;

        let request = SearchDocsRequest {
            search_key,
            doc_types: self.doc_types,
            search_scope: self.search_scope,
            sort: self.sort,
            page_size: self.page_size,
            page_token: self.page_token,
        };

        service.search_documents(&request).await
    }
}

impl Default for SearchDocumentsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 获取文档元数据构建器
///
/// 提供构建器模式获取文档元数据，支持链式调用。
#[derive(Debug, Clone)]
pub struct GetDocumentsMetaBuilder {
    tokens: Vec<String>,
    extra_fields: Option<Vec<String>>,
}

impl GetDocumentsMetaBuilder {
    /// 创建新的元数据构建器实例
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            extra_fields: None,
        }
    }

    /// 设置文档token列表
    pub fn tokens(mut self, tokens: Vec<String>) -> Self {
        self.tokens = tokens;
        self
    }

    /// 添加文档token
    pub fn add_token(mut self, token: impl Into<String>) -> Self {
        self.tokens.push(token.into());
        self
    }

    /// 设置扩展字段
    pub fn extra_fields(mut self, extra_fields: Vec<String>) -> Self {
        self.extra_fields = Some(extra_fields);
        self
    }

    /// 添加扩展字段
    pub fn add_extra_field(mut self, field: impl Into<String>) -> Self {
        let mut fields = self.extra_fields.unwrap_or_default();
        fields.push(field.into());
        self.extra_fields = Some(fields);
        self
    }

    /// 执行获取元数据操作
    pub async fn execute(self, service: &CcmDocsService) -> SDKResult<GetDocMetaResponse> {
        if self.tokens.is_empty() {
            return Err(openlark_core::error::LarkAPIError::illegal_param(
                "至少需要指定一个文档token",
            ));
        }

        let request = GetDocMetaRequest {
            tokens: self.tokens,
            extra_fields: self.extra_fields,
        };

        service.get_documents_meta(&request).await
    }
}

impl Default for GetDocumentsMetaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// docx API 服务
#[derive(Clone, Debug)]
pub struct DocxService {
    config: Config,
}

impl DocxService {
    /// 创建docx服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建文档
    ///
    /// 创建一个新的云文档，支持指定文档标题、文件夹等参数。
    ///
    /// # 参数
    /// - `req`: 创建文档请求
    ///
    /// # 返回
    /// - `SDKResult<CreateDocumentResponse>`: 创建结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ccm::docx::{DocxService, CreateDocumentRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret");
    ///     let service = DocxService::new(config);
    ///
    ///     let request = CreateDocumentRequest {
    ///         title: "新文档".to_string(),
    ///         folder_token: Some("folder_token".to_string()),
    ///         parent_type: Some("explorer".to_string()),
    ///     };
    ///
    ///     let response = service.create_document(&request).await?;
    ///     println!("文档创建成功: token={:?}", response.document_token);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_document(
        &self,
        req: &CreateDocumentRequest,
    ) -> SDKResult<CreateDocumentResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始创建文档: title={}, folder_token={:?}",
            req.title,
            req.folder_token
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 文件夹token参数
        if let Some(ref folder_token) = req.folder_token {
            query_params.insert("folder_token", folder_token.clone());
        }

        // 父类型参数
        if let Some(ref parent_type) = req.parent_type {
            query_params.insert("parent_type", parent_type.clone());
        }

        // 构建请求体
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert(
            "title".to_string(),
            serde_json::Value::String(req.title.clone()),
        );
        if let Some(ref cover_key) = req.cover_key {
            body.insert(
                "cover_key".to_string(),
                serde_json::Value::String(cover_key.clone()),
            );
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/docx/v1/documents".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<CreateDocumentResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档创建完成: title={}, document_token={:?}",
            req.title,
            response.document_token
        );

        Ok(response)
    }

    /// 获取文档信息
    ///
    /// 获取指定文档的详细信息，包括基础信息、权限等。
    ///
    /// # 参数
    /// - `req`: 获取文档请求
    ///
    /// # 返回
    /// - `SDKResult<GetDocumentResponse>`: 文档信息
    pub async fn get_document(&self, req: &GetDocumentRequest) -> SDKResult<GetDocumentResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!("开始获取文档信息: document_token={}", req.document_token);

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 是否需要权限信息
        if let Some(need_permission) = req.need_permission {
            query_params.insert("need_permission", need_permission.to_string());
        }

        // 是否需要统计信息
        if let Some(need_statistics) = req.need_statistics {
            query_params.insert("need_statistics", need_statistics.to_string());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/docx/v1/documents/{}", req.document_token),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp = Transport::<GetDocumentResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("文档信息获取完成: document_token={}", req.document_token);

        Ok(response)
    }

    /// 获取文档原始内容
    ///
    /// 获取文档的原始内容，用于下载或导出。
    ///
    /// # 参数
    /// - `req`: 获取原始内容请求
    ///
    /// # 返回
    /// - `SDKResult<GetDocumentRawContentResponse>`: 原始内容
    pub async fn get_document_raw_content(
        &self,
        req: &GetDocumentRawContentRequest,
    ) -> SDKResult<GetDocumentRawContentResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始获取文档原始内容: document_token={}",
            req.document_token
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/docx/v1/documents/{}/raw_content",
                req.document_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp = Transport::<GetDocumentRawContentResponse>::request(api_req, &self.config, None)
            .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档原始内容获取完成: document_token={}",
            req.document_token
        );

        Ok(response)
    }

    /// 列出文档块
    ///
    /// 获取文档中的所有块信息，支持分页和筛选。
    ///
    /// # 参数
    /// - `req`: 列出文档块请求
    ///
    /// # 返回
    /// - `SDKResult<ListDocumentBlocksResponse>`: 文档块列表
    pub async fn list_document_blocks(
        &self,
        req: &ListDocumentBlocksRequest,
    ) -> SDKResult<ListDocumentBlocksResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!("开始列出文档块: document_token={}", req.document_token);

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 分页参数
        if let Some(page_size) = req.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = req.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 块ID筛选
        if let Some(ref block_id) = req.block_id {
            query_params.insert("block_id", block_id.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/docx/v1/documents/{}/blocks", req.document_token),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<ListDocumentBlocksResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档块列表获取完成: document_token={}, block_count={:?}",
            req.document_token,
            response.items.as_ref().map(|items| items.len())
        );

        Ok(response)
    }

    /// 创建块子级
    ///
    /// 在指定块下创建子块，支持多种块类型。
    ///
    /// # 参数
    /// - `req`: 创建块子级请求
    ///
    /// # 返回
    /// - `SDKResult<CreateBlockChildrenResponse>`: 创建结果
    pub async fn create_block_children(
        &self,
        req: &CreateBlockChildrenRequest,
    ) -> SDKResult<CreateBlockChildrenResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始创建块子级: document_token={}, block_id={}",
            req.document_token,
            req.block_id
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建请求体
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert(
            "block_id".to_string(),
            serde_json::Value::String(req.block_id.clone()),
        );
        body.insert("children".to_string(), serde_json::to_value(&req.children)?);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/docx/v1/documents/{}/blocks/batch_create",
                req.document_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<CreateBlockChildrenResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "块子级创建完成: document_token={}, block_id={}, created_count={:?}",
            req.document_token,
            req.block_id,
            response.block_ids.as_ref().map(|ids| ids.len())
        );

        Ok(response)
    }

    /// 创建块后代
    ///
    /// 递归创建块及其所有后代，适用于复杂的文档结构。
    ///
    /// # 参数
    /// - `req`: 创建块后代请求
    ///
    /// # 返回
    /// - `SDKResult<CreateBlockDescendantResponse>`: 创建结果
    pub async fn create_block_descendant(
        &self,
        req: &CreateBlockDescendantRequest,
    ) -> SDKResult<CreateBlockDescendantResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始创建块后代: document_token={}, block_id={}",
            req.document_token,
            req.block_id
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建请求体
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert("blocks".to_string(), serde_json::to_value(&req.blocks)?);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/docx/v1/documents/{}/blocks/create",
                req.document_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp = Transport::<CreateBlockDescendantResponse>::request(api_req, &self.config, None)
            .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "块后代创建完成: document_token={}, block_count={:?}",
            req.document_token,
            response.block_ids.as_ref().map(|ids| ids.len())
        );

        Ok(response)
    }

    /// 更新块
    ///
    /// 更新指定块的内容和属性。
    ///
    /// # 参数
    /// - `req`: 更新块请求
    ///
    /// # 返回
    /// - `SDKResult<UpdateBlockResponse>`: 更新结果
    pub async fn update_block(&self, req: &UpdateBlockRequest) -> SDKResult<UpdateBlockResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始更新块: document_token={}, block_id={}",
            req.document_token,
            req.block_id
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建请求体
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert(
            "block_id".to_string(),
            serde_json::Value::String(req.block_id.clone()),
        );

        if let Some(ref block_type) = req.block_type {
            body.insert(
                "block_type".to_string(),
                serde_json::Value::String(block_type.clone()),
            );
        }

        if let Some(ref children) = req.children {
            body.insert("children".to_string(), serde_json::to_value(children)?);
        }

        if let Some(ref text_elements) = req.text_elements {
            body.insert(
                "text_elements".to_string(),
                serde_json::to_value(text_elements)?,
            );
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path: format!(
                "/open-apis/docx/v1/documents/{}/blocks/{}",
                req.document_token, req.block_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp = Transport::<UpdateBlockResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "块更新完成: document_token={}, block_id={}",
            req.document_token,
            req.block_id
        );

        Ok(response)
    }

    /// 获取块信息
    ///
    /// 获取指定块的详细信息，包括内容和子块。
    ///
    /// # 参数
    /// - `req`: 获取块请求
    ///
    /// # 返回
    /// - `SDKResult<GetBlockResponse>`: 块信息
    pub async fn get_block(&self, req: &GetBlockRequest) -> SDKResult<GetBlockResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始获取块信息: document_token={}, block_id={}",
            req.document_token,
            req.block_id
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/docx/v1/documents/{}/blocks/{}",
                req.document_token, req.block_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp = Transport::<GetBlockResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "块信息获取完成: document_token={}, block_id={}",
            req.document_token,
            req.block_id
        );

        Ok(response)
    }

    /// 批量更新块
    ///
    /// 批量更新多个块，提高操作效率。
    ///
    /// # 参数
    /// - `req`: 批量更新块请求
    ///
    /// # 返回
    /// - `SDKResult<BatchUpdateBlocksResponse>`: 批量更新结果
    pub async fn batch_update_blocks(
        &self,
        req: &BatchUpdateBlocksRequest,
    ) -> SDKResult<BatchUpdateBlocksResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始批量更新块: document_token={}, block_count={}",
            req.document_token,
            req.requests.len()
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建请求体
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert("requests".to_string(), serde_json::to_value(&req.requests)?);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/docx/v1/documents/{}/blocks/batch_update",
                req.document_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<BatchUpdateBlocksResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "块批量更新完成: document_token={}, success_count={:?}",
            req.document_token,
            response.responses.as_ref().map(|responses| responses.len())
        );

        Ok(response)
    }

    /// 获取块子级
    ///
    /// 获取指定块的所有子块信息。
    ///
    /// # 参数
    /// - `req`: 获取块子级请求
    ///
    /// # 返回
    /// - `SDKResult<GetBlockChildrenResponse>`: 块子级列表
    pub async fn get_block_children(
        &self,
        req: &GetBlockChildrenRequest,
    ) -> SDKResult<GetBlockChildrenResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始获取块子级: document_token={}, block_id={}",
            req.document_token,
            req.block_id
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/docx/v1/documents/{}/blocks/{}/children",
                req.document_token, req.block_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<GetBlockChildrenResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "块子级获取完成: document_token={}, block_id={}, child_count={:?}",
            req.document_token,
            req.block_id,
            response.items.as_ref().map(|items| items.len())
        );

        Ok(response)
    }

    /// 删除块子级
    ///
    /// 删除指定块的子块。
    ///
    /// # 参数
    /// - `req`: 删除块子级请求
    ///
    /// # 返回
    /// - `SDKResult<DeleteBlockChildrenResponse>`: 删除结果
    pub async fn delete_block_children(
        &self,
        req: &DeleteBlockChildrenRequest,
    ) -> SDKResult<DeleteBlockChildrenResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始删除块子级: document_token={}, block_id={}, child_count={}",
            req.document_token,
            req.block_id,
            req.block_ids.len()
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建请求体
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert(
            "block_id".to_string(),
            serde_json::Value::String(req.block_id.clone()),
        );
        body.insert(
            "block_ids".to_string(),
            serde_json::to_value(&req.block_ids)?,
        );

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: format!(
                "/open-apis/docx/v1/documents/{}/blocks/batch_delete",
                req.document_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<DeleteBlockChildrenResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "块子级删除完成: document_token={}, block_id={}, deleted_count={:?}",
            req.document_token,
            req.block_id,
            response.deleted_block_ids.as_ref().map(|ids| ids.len())
        );

        Ok(response)
    }

    /// 内容转换
    ///
    /// 将文档内容转换为其他格式，如HTML、Markdown等。
    ///
    /// # 参数
    /// - `req`: 内容转换请求
    ///
    /// # 返回
    /// - `SDKResult<ConvertContentResponse>`: 转换结果
    pub async fn convert_content(
        &self,
        req: &ConvertContentRequest,
    ) -> SDKResult<ConvertContentResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始内容转换: document_token={}, target_type={}",
            req.document_token,
            req.target_type
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建请求体
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert(
            "target_type".to_string(),
            serde_json::Value::String(req.target_type.clone()),
        );

        if let Some(ref block_id) = req.block_id {
            body.insert(
                "block_id".to_string(),
                serde_json::Value::String(block_id.clone()),
            );
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/docx/v1/documents/{}/content/convert",
                req.document_token
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<ConvertContentResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "内容转换完成: document_token={}, target_type={}",
            req.document_token,
            req.target_type
        );

        Ok(response)
    }

    /// 获取群公告
    ///
    /// 获取指定群的公告内容。
    ///
    /// # 参数
    /// - `req`: 获取群公告请求
    ///
    /// # 返回
    /// - `SDKResult<ChatAnnouncementResponse>`: 群公告信息
    pub async fn get_chat_announcement(
        &self,
        req: &ChatAnnouncementRequest,
    ) -> SDKResult<ChatAnnouncementResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!("开始获取群公告: chat_id={}", req.chat_id);

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/docx/v1/chats/{}/announcement", req.chat_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<ChatAnnouncementResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("群公告获取完成: chat_id={}", req.chat_id);

        Ok(response)
    }

    /// 列出群公告块
    ///
    /// 获取群公告的所有块信息。
    ///
    /// # 参数
    /// - `req`: 列出群公告块请求
    ///
    /// # 返回
    /// - `SDKResult<ListChatAnnouncementBlocksResponse>`: 群公告块列表
    pub async fn list_chat_announcement_blocks(
        &self,
        req: &ListChatAnnouncementBlocksRequest,
    ) -> SDKResult<ListChatAnnouncementBlocksResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!("开始列出群公告块: chat_id={}", req.chat_id);

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/docx/v1/chats/{}/announcement/blocks",
                req.chat_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<ListChatAnnouncementBlocksResponse>::request(api_req, &self.config, None)
                .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "群公告块列表获取完成: chat_id={}, block_count={:?}",
            req.chat_id,
            response.items.as_ref().map(|items| items.len())
        );

        Ok(response)
    }

    /// 创建群公告块子级
    ///
    /// 在群公告中创建子块。
    ///
    /// # 参数
    /// - `req`: 创建群公告块子级请求
    ///
    /// # 返回
    /// - `SDKResult<CreateChatAnnouncementBlockChildrenResponse>`: 创建结果
    pub async fn create_chat_announcement_block_children(
        &self,
        req: &CreateChatAnnouncementBlockChildrenRequest,
    ) -> SDKResult<CreateChatAnnouncementBlockChildrenResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始创建群公告块子级: chat_id={}, block_id={}",
            req.chat_id,
            req.block_id
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建请求体
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert(
            "block_id".to_string(),
            serde_json::Value::String(req.block_id.clone()),
        );
        body.insert("children".to_string(), serde_json::to_value(&req.children)?);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/docx/v1/chats/{}/announcement/blocks/batch_create",
                req.chat_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp = Transport::<CreateChatAnnouncementBlockChildrenResponse>::request(
            api_req,
            &self.config,
            None,
        )
        .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "群公告块子级创建完成: chat_id={}, block_id={}, created_count={:?}",
            req.chat_id,
            req.block_id,
            response.block_ids.as_ref().map(|ids| ids.len())
        );

        Ok(response)
    }

    /// 批量更新群公告块
    ///
    /// 批量更新群公告中的块。
    ///
    /// # 参数
    /// - `req`: 批量更新群公告块请求
    ///
    /// # 返回
    /// - `SDKResult<BatchUpdateChatAnnouncementBlocksResponse>`: 批量更新结果
    pub async fn batch_update_chat_announcement_blocks(
        &self,
        req: &BatchUpdateChatAnnouncementBlocksRequest,
    ) -> SDKResult<BatchUpdateChatAnnouncementBlocksResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始批量更新群公告块: chat_id={}, block_count={}",
            req.chat_id,
            req.requests.len()
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建请求体
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert("requests".to_string(), serde_json::to_value(&req.requests)?);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!(
                "/open-apis/docx/v1/chats/{}/announcement/blocks/batch_update",
                req.chat_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp = Transport::<BatchUpdateChatAnnouncementBlocksResponse>::request(
            api_req,
            &self.config,
            None,
        )
        .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "群公告块批量更新完成: chat_id={}, success_count={:?}",
            req.chat_id,
            response.responses.as_ref().map(|responses| responses.len())
        );

        Ok(response)
    }

    /// 获取群公告块
    ///
    /// 获取群公告中指定块的信息。
    ///
    /// # 参数
    /// - `req`: 获取群公告块请求
    ///
    /// # 返回
    /// - `SDKResult<GetChatAnnouncementBlockResponse>`: 群公告块信息
    pub async fn get_chat_announcement_block(
        &self,
        req: &GetChatAnnouncementBlockRequest,
    ) -> SDKResult<GetChatAnnouncementBlockResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始获取群公告块: chat_id={}, block_id={}",
            req.chat_id,
            req.block_id
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/docx/v1/chats/{}/announcement/blocks/{}",
                req.chat_id, req.block_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<GetChatAnnouncementBlockResponse>::request(api_req, &self.config, None)
                .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "群公告块获取完成: chat_id={}, block_id={}",
            req.chat_id,
            req.block_id
        );

        Ok(response)
    }

    /// 获取群公告块子级
    ///
    /// 获取群公告中指定块的所有子块。
    ///
    /// # 参数
    /// - `req`: 获取群公告块子级请求
    ///
    /// # 返回
    /// - `SDKResult<GetChatAnnouncementBlockChildrenResponse>`: 群公告块子级列表
    pub async fn get_chat_announcement_block_children(
        &self,
        req: &GetChatAnnouncementBlockChildrenRequest,
    ) -> SDKResult<GetChatAnnouncementBlockChildrenResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始获取群公告块子级: chat_id={}, block_id={}",
            req.chat_id,
            req.block_id
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/docx/v1/chats/{}/announcement/blocks/{}/children",
                req.chat_id, req.block_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp = Transport::<GetChatAnnouncementBlockChildrenResponse>::request(
            api_req,
            &self.config,
            None,
        )
        .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "群公告块子级获取完成: chat_id={}, block_id={}, child_count={:?}",
            req.chat_id,
            req.block_id,
            response.items.as_ref().map(|items| items.len())
        );

        Ok(response)
    }

    /// 删除群公告块子级
    ///
    /// 删除群公告中指定块的子块。
    ///
    /// # 参数
    /// - `req`: 删除群公告块子级请求
    ///
    /// # 返回
    /// - `SDKResult<DeleteChatAnnouncementBlockChildrenResponse>`: 删除结果
    pub async fn delete_chat_announcement_block_children(
        &self,
        req: &DeleteChatAnnouncementBlockChildrenRequest,
    ) -> SDKResult<DeleteChatAnnouncementBlockChildrenResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始删除群公告块子级: chat_id={}, block_id={}, child_count={}",
            req.chat_id,
            req.block_id,
            req.block_ids.len()
        );

        let mut query_params: std::collections::HashMap<&str, String> =
            std::collections::HashMap::new();

        // 用户ID类型参数
        if let Some(ref user_id_type) = req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建请求体
        let mut body: HashMap<String, serde_json::Value> = HashMap::new();
        body.insert(
            "block_id".to_string(),
            serde_json::Value::String(req.block_id.clone()),
        );
        body.insert(
            "block_ids".to_string(),
            serde_json::to_value(&req.block_ids)?,
        );

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: format!(
                "/open-apis/docx/v1/chats/{}/announcement/blocks/batch_delete",
                req.chat_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&body)?,
            query_params,
            ..Default::default()
        };

        let resp = Transport::<DeleteChatAnnouncementBlockChildrenResponse>::request(
            api_req,
            &self.config,
            None,
        )
        .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "群公告块子级删除完成: chat_id={}, block_id={}, deleted_count={:?}",
            req.chat_id,
            req.block_id,
            response.deleted_block_ids.as_ref().map(|ids| ids.len())
        );

        Ok(response)
    }
}

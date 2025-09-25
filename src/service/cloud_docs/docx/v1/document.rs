use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 文档服务
pub struct DocumentService {
    config: Config,
}

impl DocumentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建文档
    ///
    /// 该接口用于创建一个新的文档。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/create>
    pub async fn create(
        &self,
        request: CreateDocumentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateDocumentRespData>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DOCX_V1_DOCUMENTS.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取文档基本信息
    ///
    /// 该接口用于获取文档的基本信息。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/get>
    pub async fn get(
        &self,
        document_id: impl Into<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetDocumentRespData>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DOCX_V1_DOCUMENT_GET.replace("{}", &document_id.into()),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取文档纯文本内容
    ///
    /// 该接口用于获取文档的纯文本内容。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/raw_content>
    pub async fn get_raw_content(
        &self,
        document_id: impl Into<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetRawContentRespData>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DOCX_V1_DOCUMENT_RAW_CONTENT.replace("{}", &document_id.into()),
            ..Default::default()
        };
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取文档所有块
    ///
    /// 该接口用于获取文档的所有块。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/list>
    pub async fn list_blocks(
        &self,
        request: ListDocumentBlocksRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListDocumentBlocksRespData>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DOCX_V1_DOCUMENT_BLOCKS.replace("{}", &request.document_id),
            ..Default::default()
        };
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 转换为文档块
    ///
    /// 该接口用于将旧版文档转换为新版文档块格式。
    ///
    /// <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert>
    pub async fn convert_to_docx(
        &self,
        document_id: impl Into<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ConvertToDocxRespData>> {
        let mut api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DOCX_V1_DOCUMENT_CONVERT.replace("{}", &document_id.into()),
            ..Default::default()
        };
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// === 数据结构定义 ===

/// 创建文档请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDocumentRequest {
    /// 文档标题
    pub title: String,
    /// 文档内容，JSON格式的块内容
    pub content: Option<String>,
    /// 文件夹token
    pub folder_token: Option<String>,
}

impl CreateDocumentRequest {
    pub fn builder() -> CreateDocumentRequestBuilder {
        CreateDocumentRequestBuilder::default()
    }

    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: None,
            folder_token: None,
        }
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn with_folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }
}

/// 创建文档请求构建器
#[derive(Default)]
pub struct CreateDocumentRequestBuilder {
    request: CreateDocumentRequest,
}

impl CreateDocumentRequestBuilder {
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = title.into();
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.request.content = Some(content.into());
        self
    }

    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = Some(folder_token.into());
        self
    }

    pub fn build(self) -> CreateDocumentRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    CreateDocumentRequestBuilder,
    DocumentService,
    CreateDocumentRequest,
    BaseResponse<CreateDocumentRespData>,
    create
);

/// 创建文档响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRespData {
    /// 文档ID
    pub document_id: String,
    /// 文档版本ID
    pub document_revision_id: i64,
    /// 文档标题
    pub title: String,
}

impl ApiResponseTrait for CreateDocumentRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档信息响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRespData {
    /// 文档信息
    pub document: DocumentInfo,
}

impl ApiResponseTrait for GetDocumentRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文档信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentInfo {
    /// 文档ID
    pub document_id: String,
    /// 文档版本ID
    pub document_revision_id: i64,
    /// 文档标题
    pub title: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 创建者ID
    pub creator_id: String,
    /// 最后编辑者ID
    pub last_editor_id: String,
}

/// 获取纯文本内容响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRawContentRespData {
    /// 纯文本内容
    pub content: String,
}

impl ApiResponseTrait for GetRawContentRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档所有块请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListDocumentBlocksRequest {
    /// 文档ID
    pub document_id: String,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ListDocumentBlocksRequest {
    pub fn builder() -> ListDocumentBlocksRequestBuilder {
        ListDocumentBlocksRequestBuilder::default()
    }

    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
            page_size: None,
            page_token: None,
        }
    }

    pub fn with_page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn with_page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }
}

/// 获取文档所有块请求构建器
#[derive(Default)]
pub struct ListDocumentBlocksRequestBuilder {
    request: ListDocumentBlocksRequest,
}

impl ListDocumentBlocksRequestBuilder {
    pub fn document_id(mut self, document_id: impl Into<String>) -> Self {
        self.request.document_id = document_id.into();
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

    pub fn build(self) -> ListDocumentBlocksRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    ListDocumentBlocksRequestBuilder,
    DocumentService,
    ListDocumentBlocksRequest,
    BaseResponse<ListDocumentBlocksRespData>,
    list_blocks
);

/// 获取文档所有块响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDocumentBlocksRespData {
    /// 块列表
    pub items: Vec<Block>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListDocumentBlocksRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 块信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// 块ID
    pub block_id: String,
    /// 父块ID
    pub parent_id: String,
    /// 子块ID列表
    pub children: Vec<String>,
    /// 块类型
    pub block_type: i32,
    /// 块索引
    pub index: i32,
}

/// 转换为文档块响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertToDocxRespData {
    /// 新文档ID
    pub document_id: String,
    /// 文档版本ID
    pub document_revision_id: i64,
}

impl ApiResponseTrait for ConvertToDocxRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

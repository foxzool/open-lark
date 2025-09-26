use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder},
    http::Transport,
    req_option::RequestOption,
    trait_system::Service,
    SDKResult,
};

/// 文档块服务
pub struct DocumentBlockService {
    config: Config,
}

impl DocumentBlockService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建块
    ///
    /// 该接口用于在文档中创建一个新的块。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/create>
    pub async fn create(
        &self,
        document_id: impl Into<String>,
        request: CreateBlockRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateBlockRespData>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DOCX_V1_DOCUMENT_BLOCKS.replace("{}", &document_id.into()),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取块的内容
    ///
    /// 该接口用于获取块的详细内容。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/get>
    pub async fn get(
        &self,
        document_id: impl Into<String>,
        block_id: impl Into<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetBlockRespData>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DOCX_V1_DOCUMENT_BLOCK_GET
                .replace("{document_id}", &document_id.into())
                .replace("{block_id}", &block_id.into()),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 更新块的内容
    ///
    /// 该接口用于更新块的内容。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/patch>
    pub async fn patch(
        &self,
        document_id: impl Into<String>,
        block_id: impl Into<String>,
        request: PatchBlockRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PatchBlockRespData>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: DOCX_V1_DOCUMENT_BLOCK_GET
                .replace("{document_id}", &document_id.into())
                .replace("{block_id}", &block_id.into()),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 批量更新块的内容
    ///
    /// 该接口用于批量更新多个块的内容。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/batch_update>
    pub async fn batch_update(
        &self,
        document_id: impl Into<String>,
        request: BatchUpdateBlockRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchUpdateBlockRespData>> {
        let mut api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: DOCX_V1_DOCUMENT_BLOCKS_BATCH_UPDATE
                .replace("{document_id}", &document_id.into()),
            ..Default::default()
        };
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = serde_json::to_vec(&request)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 删除块
    ///
    /// 该接口用于批量删除块。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/batch_delete>
    pub async fn batch_delete(
        &self,
        document_id: impl Into<String>,
        request: BatchDeleteBlockRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchDeleteBlockRespData>> {
        let mut api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                DOCX_V1_DOCUMENT_BLOCKS_BATCH_DELETE,
                "document_id",
                &document_id.into(),
            ),
            ..Default::default()
        };
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = serde_json::to_vec(&request)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取所有子块
    ///
    /// 该接口用于获取指定块的所有子块。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/get-2>
    pub async fn list_children(
        &self,
        document_id: impl Into<String>,
        block_id: impl Into<String>,
        request: ListChildrenRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListChildrenRespData>> {
        let document_id_str = document_id.into();
        let block_id_str = block_id.into();
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                DOCX_V1_DOCUMENT_BLOCK_CHILDREN,
                &[
                    ("document_id", &document_id_str),
                    ("block_id", &block_id_str),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

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
}

// === 数据结构定义 ===

/// 创建块请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBlockRequest {
    /// 父块ID，如果创建在文档根级，传document_id
    pub parent_id: String,
    /// 块索引位置
    pub index: Option<i32>,
    /// 块数据列表
    pub blocks: Vec<BlockData>,
}

impl CreateBlockRequest {
    pub fn builder() -> CreateBlockRequestBuilder {
        CreateBlockRequestBuilder::default()
    }

    pub fn new(parent_id: impl Into<String>, blocks: Vec<BlockData>) -> Self {
        Self {
            parent_id: parent_id.into(),
            index: None,
            blocks,
        }
    }

    pub fn with_index(mut self, index: i32) -> Self {
        self.index = Some(index);
        self
    }
}

/// 创建块请求构建器
#[derive(Default)]
pub struct CreateBlockRequestBuilder {
    request: CreateBlockRequest,
    document_id: String,
}

impl CreateBlockRequestBuilder {
    pub fn document_id(mut self, document_id: impl Into<String>) -> Self {
        self.document_id = document_id.into();
        self
    }

    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.request.parent_id = parent_id.into();
        self
    }

    pub fn index(mut self, index: i32) -> Self {
        self.request.index = Some(index);
        self
    }

    pub fn blocks(mut self, blocks: Vec<BlockData>) -> Self {
        self.request.blocks = blocks;
        self
    }

    pub fn add_block(mut self, block: BlockData) -> Self {
        self.request.blocks.push(block);
        self
    }

    pub fn build(self) -> (String, CreateBlockRequest) {
        (self.document_id, self.request)
    }
}

/// 块数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockData {
    /// 块类型
    pub block_type: i32,
    /// 块内容（根据不同类型有不同结构）
    pub block: Value,
}

/// 创建块响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBlockRespData {
    /// 创建的块列表
    pub blocks: Vec<BlockInfo>,
    /// 文档版本ID
    pub document_revision_id: i64,
}

impl ApiResponseTrait for CreateBlockRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 块信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
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

/// 获取块响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlockRespData {
    /// 块信息
    pub block: DetailedBlock,
}

impl ApiResponseTrait for GetBlockRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 详细块信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedBlock {
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
    /// 块内容
    pub block: Value,
}

/// 更新块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchBlockRequest {
    /// 要更新的块内容
    pub block: Value,
}

impl PatchBlockRequest {
    pub fn new(block: Value) -> Self {
        Self { block }
    }
}

/// 更新块响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchBlockRespData {
    /// 更新后的块信息
    pub block: DetailedBlock,
    /// 文档版本ID
    pub document_revision_id: i64,
}

impl ApiResponseTrait for PatchBlockRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新块请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchUpdateBlockRequest {
    /// 要更新的块列表
    pub requests: Vec<UpdateBlockItem>,
}

/// 更新块项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBlockItem {
    /// 块ID
    pub block_id: String,
    /// 要更新的块内容
    pub block: Value,
}

impl BatchUpdateBlockRequest {
    pub fn builder() -> BatchUpdateBlockRequestBuilder {
        BatchUpdateBlockRequestBuilder::default()
    }

    pub fn new(requests: Vec<UpdateBlockItem>) -> Self {
        Self { requests }
    }
}

/// 批量更新块请求构建器
#[derive(Default)]
pub struct BatchUpdateBlockRequestBuilder {
    request: BatchUpdateBlockRequest,
    document_id: String,
}

impl BatchUpdateBlockRequestBuilder {
    pub fn document_id(mut self, document_id: impl Into<String>) -> Self {
        self.document_id = document_id.into();
        self
    }

    pub fn requests(mut self, requests: Vec<UpdateBlockItem>) -> Self {
        self.request.requests = requests;
        self
    }

    pub fn add_request(mut self, block_id: impl Into<String>, block: Value) -> Self {
        self.request.requests.push(UpdateBlockItem {
            block_id: block_id.into(),
            block,
        });
        self
    }

    pub fn build(self) -> (String, BatchUpdateBlockRequest) {
        (self.document_id, self.request)
    }
}

/// 批量更新块响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateBlockRespData {
    /// 更新的块列表
    pub blocks: Vec<DetailedBlock>,
    /// 文档版本ID
    pub document_revision_id: i64,
}

impl ApiResponseTrait for BatchUpdateBlockRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除块请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteBlockRequest {
    /// 要删除的块ID列表
    pub block_ids: Vec<String>,
}

impl BatchDeleteBlockRequest {
    pub fn builder() -> BatchDeleteBlockRequestBuilder {
        BatchDeleteBlockRequestBuilder::default()
    }

    pub fn new(block_ids: Vec<String>) -> Self {
        Self { block_ids }
    }
}

/// 批量删除块请求构建器
#[derive(Default)]
pub struct BatchDeleteBlockRequestBuilder {
    request: BatchDeleteBlockRequest,
    document_id: String,
}

impl BatchDeleteBlockRequestBuilder {
    pub fn document_id(mut self, document_id: impl Into<String>) -> Self {
        self.document_id = document_id.into();
        self
    }

    pub fn block_ids(mut self, block_ids: Vec<String>) -> Self {
        self.request.block_ids = block_ids;
        self
    }

    pub fn add_block_id(mut self, block_id: impl Into<String>) -> Self {
        self.request.block_ids.push(block_id.into());
        self
    }

    pub fn build(self) -> (String, BatchDeleteBlockRequest) {
        (self.document_id, self.request)
    }
}

/// 批量删除块响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteBlockRespData {
    /// 文档版本ID
    pub document_revision_id: i64,
}

impl ApiResponseTrait for BatchDeleteBlockRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取子块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListChildrenRequest {
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ListChildrenRequest {
    pub fn new() -> Self {
        Self {
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

impl Default for ListChildrenRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// 获取子块响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListChildrenRespData {
    /// 子块列表
    pub items: Vec<DetailedBlock>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListChildrenRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// === Builder execute方法实现 ===
// 为需要路径参数的Builder提供统一的execute方法

macro_rules! impl_execute_with_path {
    ($builder:ty, $response:ty, $method:ident) => {
        impl $builder {
            /// 执行请求
            pub async fn execute(
                self,
                service: &DocumentBlockService,
                option: Option<RequestOption>,
            ) -> SDKResult<$response> {
                let (document_id, request) = self.build();
                service.$method(document_id, request, option).await
            }

            /// 执行请求（带选项）
            pub async fn execute_with_options(
                self,
                service: &DocumentBlockService,
                option: RequestOption,
            ) -> SDKResult<$response> {
                self.execute(service, Some(option)).await
            }
        }
    };
}

impl_execute_with_path!(
    CreateBlockRequestBuilder,
    BaseResponse<CreateBlockRespData>,
    create
);

impl_execute_with_path!(
    BatchUpdateBlockRequestBuilder,
    BaseResponse<BatchUpdateBlockRespData>,
    batch_update
);

impl_execute_with_path!(
    BatchDeleteBlockRequestBuilder,
    BaseResponse<BatchDeleteBlockRespData>,
    batch_delete
);

// === Service trait 实现 ===

impl Service for DocumentBlockService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "document_block"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

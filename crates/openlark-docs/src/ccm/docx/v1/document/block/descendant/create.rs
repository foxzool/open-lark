//! 创建嵌套块
//!
//! 在指定块的子块列表中，新创建一批有父子关系的子块，并放置到指定位置。如果操作成功，接口将返回新创建子块的富文本内容。当创建的子块中含有 GridColumn、TableCell、Callout 时其中至少需要包含一个子块 ，即内容为空时也需要填入一个空 Text Block 作为子块。
//! API文档: https://open.feishu.cn/document/docs/docs/document-block/create-2

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::DocxApiV1;
use crate::ccm::docx::common_types::BlockContent;

/// 创建嵌套块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentBlockDescendantParams {
    /// 文档ID
    pub document_id: String,
    /// 父块ID
    pub block_id: String,
    /// 新建的嵌套子块列表
    pub children: Vec<NewNestedBlock>,
    /// 插入位置
    pub location: Option<BlockLocation>,
}

/// 新建的嵌套块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewNestedBlock {
    /// 块类型
    pub block_type: i32,
    /// 块内容
    pub content: Option<BlockContent>,
    /// 嵌套子块
    pub children: Option<Vec<NewNestedBlock>>,
}


/// 块位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockLocation {
    /// 插入位置索引
    pub index: Option<i32>,
}

/// 创建嵌套块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentBlockDescendantResponse {
    /// 创建结果
    pub data: Option<CreateResult>,
}

/// 创建结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResult {
    /// 创建成功的块列表
    pub blocks: Option<Vec<CreatedNestedBlock>>,
}

/// 创建的嵌套块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedNestedBlock {
    /// 块ID
    pub block_id: String,
    /// 块类型
    pub block_type: i32,
    /// 块内容
    pub content: Option<BlockContent>,
    /// 子块
    pub children: Option<Vec<CreatedNestedBlock>>,
}

impl ApiResponseTrait for CreateDocumentBlockDescendantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建嵌套块请求
pub struct CreateDocumentBlockDescendantRequest {
    config: Config,
}

impl CreateDocumentBlockDescendantRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self, params: CreateDocumentBlockDescendantParams) -> SDKResult<CreateDocumentBlockDescendantResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");
        validate_required!(params.children, "嵌套子块列表不能为空");

        let api_endpoint = DocxApiV1::DocumentBlockDescendantCreate(params.document_id.clone(), params.block_id.clone());
        let mut api_request: ApiRequest<CreateDocumentBlockDescendantResponse> = ApiRequest::post(&api_endpoint.to_url());
        api_request = api_request.json_body(&params);

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
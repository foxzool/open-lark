/// 创建块
///
/// 在指定块的子块列表中，新创建一批子块，并放置到指定位置。如果操作成功，接口将返回新创建子块的富文本内容。
/// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/create

use crate::ccm::docx::common_types::BlockContent;
use crate::common::api_endpoints::DocxApiV1;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentBlockChildrenParams {
    /// 文档ID
    pub document_id: String,
    /// 父块ID
    pub block_id: String,
    /// 新建的子块列表
    pub children: Vec<NewBlock>,
    /// 插入位置
    pub location: Option<BlockLocation>,
}

/// 新建的块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBlock {
    /// 块类型
    pub block_type: i32,
    /// 块内容
    pub content: Option<BlockContent>,
}

/// 块位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockLocation {
    /// 插入位置索引
    pub index: Option<i32>,
}

/// 创建块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentBlockChildrenResponse {
    /// 创建结果
    pub data: Option<CreateResult>,
}

/// 创建结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResult {
    /// 创建成功的块列表
    pub blocks: Option<Vec<CreatedBlock>>,
}

/// 创建的块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedBlock {
    /// 块ID
    pub block_id: String,
    /// 块类型
    pub block_type: i32,
}

impl ApiResponseTrait for CreateDocumentBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建块请求
pub struct CreateDocumentBlockChildrenRequest {
    config: Config,
}

impl CreateDocumentBlockChildrenRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/create
    /// 对应CSV记录: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/create
    pub async fn execute(
        self,
        params: CreateDocumentBlockChildrenParams,
    ) -> SDKResult<CreateDocumentBlockChildrenResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");
        validate_required!(params.children, "子块列表不能为空");

        let api_endpoint = DocxApiV1::DocumentBlockChildrenCreate(
            params.document_id.clone(),
            params.block_id.clone(),
        );
        let mut api_request: ApiRequest<CreateDocumentBlockChildrenResponse> =
            ApiRequest::post(&api_endpoint.to_url());
        api_request = api_request.json_body(&params);

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

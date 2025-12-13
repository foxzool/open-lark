/// 获取块的内容
///
/// 指定块的 block id 获取指定块的富文本内容数据。
/// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::ccm::docx::common_types::BlockContent;
use crate::common::api_endpoints::DocxApiV1;

/// 获取块内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlockParams {
    /// 文档ID
    pub document_id: String,
    /// 块ID
    pub block_id: String,
}

/// 获取块内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlockResponse {
    /// 块信息
    pub data: Option<BlockData>,
}

/// 块数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockData {
    /// 块ID
    pub block_id: String,
    /// 块类型
    pub block_type: String,
    /// 块内容
    pub content: Option<BlockContent>,
    /// 子块数量
    pub children_count: Option<u32>,
    /// 父块ID
    pub parent_block_id: Option<String>,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 更新时间
    pub update_time: Option<i64>,
}

impl ApiResponseTrait for GetDocumentBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取块内容请求
pub struct GetDocumentBlockRequest {
    config: Config,
}

impl GetDocumentBlockRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(
        self,
        params: GetDocumentBlockParams,
    ) -> SDKResult<GetDocumentBlockResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        let api_endpoint =
            DocxApiV1::DocumentBlockGet(params.document_id.clone(), params.block_id.clone());
        let api_request: ApiRequest<GetDocumentBlockResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

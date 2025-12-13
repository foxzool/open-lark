/// 删除块
///
/// 指定需要操作的块，删除其指定范围的子块。如果操作成功，接口将返回应用删除操作后的文档版本号。
/// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/batch_delete

use crate::common::api_endpoints::DocxApiV1;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteDocumentBlockChildrenParams {
    /// 文档ID
    pub document_id: String,
    /// 父块ID
    pub block_id: String,
    /// 要删除的子块ID列表
    pub block_ids: Vec<String>,
}

/// 删除块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteDocumentBlockChildrenResponse {
    /// 删除结果
    pub data: Option<BatchDeleteResult>,
}

/// 批量删除结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteResult {
    /// 删除成功的块数量
    pub deleted_count: Option<u32>,
    /// 文档版本号
    pub version: Option<u64>,
}

impl ApiResponseTrait for BatchDeleteDocumentBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除块请求
pub struct BatchDeleteDocumentBlockChildrenRequest {
    config: Config,
}

impl BatchDeleteDocumentBlockChildrenRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(
        self,
        params: BatchDeleteDocumentBlockChildrenParams,
    ) -> SDKResult<BatchDeleteDocumentBlockChildrenResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");
        validate_required!(params.block_ids, "子块ID列表不能为空");

        let api_endpoint = DocxApiV1::DocumentBlockChildrenBatchDelete(
            params.document_id.clone(),
            params.block_id.clone(),
        );
        let mut api_request: ApiRequest<BatchDeleteDocumentBlockChildrenResponse> =
            ApiRequest::delete(&api_endpoint.to_url());
        api_request = api_request.json_body(&params);

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

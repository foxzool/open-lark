//! 更新块的内容
//!
//! 更新指定块的内容。如果操作成功，接口将返回更新后的块的富文本内容。
//! API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/patch

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 更新块内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentBlockParams {
    /// 文档ID
    pub document_id: String,
    /// 块ID
    pub block_id: String,
    /// 块内容
    pub content: Option<BlockContent>,
}

/// 块内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockContent {
    /// 文本内容
    pub text: Option<String>,
}

/// 更新块内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentBlockResponse {
    /// 更新结果
    pub data: Option<UpdatedBlockData>,
}

/// 更新的块数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedBlockData {
    /// 块ID
    pub block_id: String,
    /// 更新后的内容
    pub content: Option<BlockContent>,
}

impl ApiResponseTrait for UpdateDocumentBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新块内容请求
pub struct UpdateDocumentBlockRequest {
    config: Config,
}

impl UpdateDocumentBlockRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self, params: UpdateDocumentBlockParams) -> SDKResult<UpdateDocumentBlockResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        let url = format!("/open-apis/docx/v1/documents/{}/blocks/{}", params.document_id, params.block_id);
        let mut api_request: ApiRequest<UpdateDocumentBlockResponse> = ApiRequest::patch(&url);
        api_request = api_request.body(&params)?;

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
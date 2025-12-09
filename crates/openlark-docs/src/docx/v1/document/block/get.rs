//! 获取块的信息
//!
//! API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/get-document-block

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::docx::v1::models::DocumentBlockInfo;
use crate::common::api_endpoints::DocxApiV1;

/// 获取块的信息请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetDocumentBlockParams {
    /// 文档ID
    pub document_id: String,
    /// 块ID
    pub block_id: String,
}

/// 获取块的信息响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GetDocumentBlockResponse {
    /// 块信息
    pub data: DocumentBlockInfo,
}

impl ApiResponseTrait for GetDocumentBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取块的信息请求
pub struct GetDocumentBlockRequest {
    config: Config,
}

impl GetDocumentBlockRequest {
    /// 创建新的获取块的信息请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/get-document-block
    pub async fn execute(self, params: GetDocumentBlockParams) -> SDKResult<GetDocumentBlockResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::DocumentBlockGet(params.document_id.clone(), params.block_id.clone());

        // 创建API请求
        let api_request: ApiRequest<GetDocumentBlockResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
//! 更新块的内容
//!
//! API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/patch-document-block

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

/// 更新块的内容请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct PatchDocumentBlockParams {
    /// 文档ID
    pub document_id: String,
    /// 块ID
    pub block_id: String,
    /// 块内容
    pub content: serde_json::Value,
}

/// 更新块的内容响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PatchDocumentBlockResponse {
    /// 更新后的块信息
    pub data: DocumentBlockInfo,
}

impl ApiResponseTrait for PatchDocumentBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新块的内容请求
pub struct PatchDocumentBlockRequest {
    config: Config,
}

impl PatchDocumentBlockRequest {
    /// 创建新的更新块的内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/patch-document-block
    pub async fn execute(self, params: PatchDocumentBlockParams) -> SDKResult<PatchDocumentBlockResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        // 验证content不为空对象
        if params.content == serde_json::Value::Null {
            return Err(openlark_core::error::validation_error("块内容不能为空", "content字段不能为null"));
        }

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::DocumentBlockPatch(params.document_id.clone(), params.block_id.clone());

        // 创建API请求
        let api_request: ApiRequest<PatchDocumentBlockResponse> =
            ApiRequest::put(&api_endpoint.to_url());

        // 设置请求体
        let body = serde_json::json!({
            "content": params.content
        });

        let request_with_body = api_request.body(body);

        // 发送请求
        let response = Transport::request(request_with_body, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
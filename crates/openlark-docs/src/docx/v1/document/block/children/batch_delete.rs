//! 删除块
//!
//! API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/delete-document-block-children

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::docx::v1::models::DeleteResult;
use crate::common::api_endpoints::DocxApiV1;

/// 删除块请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchDeleteDocumentBlockChildrenParams {
    /// 文档ID
    pub document_id: String,
    /// 块ID
    pub block_id: String,
    /// 要删除的子块ID列表
    pub block_ids: Vec<String>,
}

/// 删除块响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct BatchDeleteDocumentBlockChildrenResponse {
    /// 删除结果
    pub data: DeleteResult,
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
    /// 创建新的删除块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/delete-document-block-children
    pub async fn execute(self, params: BatchDeleteDocumentBlockChildrenParams) -> SDKResult<BatchDeleteDocumentBlockChildrenResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");
        validate_required!(params.block_ids, "子块ID列表不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::DocumentBlockChildrenBatchDelete(params.document_id.clone(), params.block_id.clone());

        // 创建API请求
        let api_request: ApiRequest<BatchDeleteDocumentBlockChildrenResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        // 设置请求体
        let body = serde_json::json!({
            "block_ids": params.block_ids
        });

        let request_with_body = api_request.body(body);

        // 发送请求
        let response = Transport::request(request_with_body, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
//! 创建块
//!
//! 在指定块的子块列表中，新创建一批子块，并放置到指定位置。如果操作成功，接口将返回新创建子块的富文本内容。
//! API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/create

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

/// 创建块请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateDocumentBlockChildrenParams {
    /// 文档ID
    pub document_id: String,
    /// 块ID
    pub block_id: String,
    /// 子块列表
    pub children: Vec<DocumentBlockChild>,
}

/// 子块信息
#[derive(Debug, Clone, Serialize, Default)]
pub struct DocumentBlockChild {
    /// 块类型
    pub block_type: i32,
    /// 块内容
    pub content: serde_json::Value,
}

/// 创建块响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateDocumentBlockChildrenResponse {
    /// 创建的块信息
    pub data: Vec<DocumentBlockInfo>,
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
    /// 创建新的创建块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/create-document-block-children
    pub async fn execute(self, params: CreateDocumentBlockChildrenParams) -> SDKResult<CreateDocumentBlockChildrenResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");
        validate_required!(params.children, "子块列表不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::DocumentBlockChildrenCreate(params.document_id.clone(), params.block_id.clone());

        // 创建API请求
        let api_request: ApiRequest<CreateDocumentBlockChildrenResponse> =
            ApiRequest::post(&api_endpoint.to_url());

        // 设置请求体
        let body = serde_json::json!({
            "children": params.children
        });

        let request_with_body = api_request.body(body);

        // 发送请求
        let response = Transport::request(request_with_body, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
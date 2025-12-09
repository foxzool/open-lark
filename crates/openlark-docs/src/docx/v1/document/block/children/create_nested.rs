//! 创建有父子关系的子块
//!
//! API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/create-nested-document-block-children

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

/// 创建有父子关系的子块请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateNestedDocumentBlockChildrenParams {
    /// 文档ID
    pub document_id: String,
    /// 块ID
    pub block_id: String,
    /// 有父子关系的子块列表
    pub children: Vec<NestedDocumentBlockChild>,
}

/// 有父子关系的子块信息
#[derive(Debug, Clone, Serialize, Default)]
pub struct NestedDocumentBlockChild {
    /// 块类型
    pub block_type: i32,
    /// 块内容
    pub content: serde_json::Value,
    /// 子块列表（用于创建嵌套结构）
    pub children: Option<Vec<NestedDocumentBlockChild>>,
}

/// 创建有父子关系的子块响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateNestedDocumentBlockChildrenResponse {
    /// 创建的块信息
    pub data: Vec<DocumentBlockInfo>,
}

impl ApiResponseTrait for CreateNestedDocumentBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建有父子关系的子块请求
pub struct CreateNestedDocumentBlockChildrenRequest {
    config: Config,
}

impl CreateNestedDocumentBlockChildrenRequest {
    /// 创建新的创建有父子关系的子块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/create-nested-document-block-children
    pub async fn execute(self, params: CreateNestedDocumentBlockChildrenParams) -> SDKResult<CreateNestedDocumentBlockChildrenResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");
        validate_required!(params.children, "子块列表不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::DocumentBlockDescendantCreate(params.document_id.clone(), params.block_id.clone());

        // 创建API请求
        let api_request: ApiRequest<CreateNestedDocumentBlockChildrenResponse> =
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
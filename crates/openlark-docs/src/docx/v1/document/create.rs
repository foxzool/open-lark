//! 创建文档
//!
//! 创建新版文档，文档标题和目录可选。
//! API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::docx::v1::models::DocumentInfo;
use crate::common::api_endpoints::DocxApiV1;

/// 创建文档请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateDocumentParams {
    /// 文档标题
    pub title: String,
    /// 文档内容（可选）
    pub content: Option<serde_json::Value>,
    /// 文档类型（可选）
    pub doc_type: Option<String>,
}

/// 创建文档响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateDocumentResponse {
    /// 文档信息
    pub data: DocumentInfo,
}

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建文档请求
pub struct CreateDocumentRequest {
    config: Config,
}

impl CreateDocumentRequest {
    /// 创建新的创建文档请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/create
    pub async fn execute(self, params: CreateDocumentParams) -> SDKResult<CreateDocumentResponse> {
        // 验证必填字段
        validate_required!(params.title, "文档标题不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::DocumentCreate;

        // 创建API请求
        let mut api_request: ApiRequest<CreateDocumentResponse> =
            ApiRequest::post(&api_endpoint.to_url());

        // 设置请求体
        let mut body = serde_json::json!({
            "title": params.title
        });

        if let Some(content) = params.content {
            body["content"] = content;
        }
        if let Some(doc_type) = params.doc_type {
            body["doc_type"] = serde_json::Value::String(doc_type);
        }

        api_request = api_request.body(body);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
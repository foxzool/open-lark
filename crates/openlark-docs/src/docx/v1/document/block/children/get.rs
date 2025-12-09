//! 获取块的所有子块
//!
//! API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/get-document-block-children

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

/// 获取块的所有子块请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetDocumentBlockChildrenParams {
    /// 文档ID
    pub document_id: String,
    /// 块ID
    pub block_id: String,
    /// 文档版本号（可选）
    pub document_revision_id: Option<String>,
    /// 分页大小
    pub page_size: Option<i64>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 分页响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DocumentBlockChildrenPage {
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 获取块的所有子块响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GetDocumentBlockChildrenResponse {
    /// 子块信息列表
    pub data: Vec<DocumentBlockInfo>,
    /// 分页信息
    pub page: DocumentBlockChildrenPage,
}

impl ApiResponseTrait for GetDocumentBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取块的所有子块请求
pub struct GetDocumentBlockChildrenRequest {
    config: Config,
}

impl GetDocumentBlockChildrenRequest {
    /// 创建新的获取块的所有子块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/get-document-block-children
    pub async fn execute(self, params: GetDocumentBlockChildrenParams) -> SDKResult<GetDocumentBlockChildrenResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::DocumentBlockChildrenGet(params.document_id.clone(), params.block_id.clone());

        // 创建API请求
        let api_request: ApiRequest<GetDocumentBlockChildrenResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 构建查询参数
        let mut query_params = Vec::new();

        if let Some(document_revision_id) = &params.document_revision_id {
            query_params.push(format!("document_revision_id={}", document_revision_id));
        }

        if let Some(page_size) = &params.page_size {
            query_params.push(format!("page_size={}", page_size));
        }

        if let Some(page_token) = &params.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        // 发送请求
        let response = if !query_params.is_empty() {
            let url_with_params = format!("{}?{}", api_endpoint.to_url(), query_params.join("&"));
            let api_request_with_params: ApiRequest<GetDocumentBlockChildrenResponse> = ApiRequest::get(&url_with_params);
            Transport::request(api_request_with_params, &self.config, None).await?
        } else {
            Transport::request(api_request, &self.config, None).await?
        };

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
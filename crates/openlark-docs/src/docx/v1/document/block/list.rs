//! 获取文档所有块
//!
//! 获取文档所有块的富文本内容并分页返回。
//! API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/list

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

/// 获取文档所有块请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListDocumentBlocksParams {
    /// 文档ID
    pub document_id: String,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页令牌
    pub page_token: Option<String>,
}

/// 获取文档所有块响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListDocumentBlocksResponse {
    /// 块列表
    pub data: Vec<DocumentBlockInfo>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页令牌
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListDocumentBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档所有块请求
pub struct ListDocumentBlocksRequest {
    config: Config,
}

impl ListDocumentBlocksRequest {
    /// 创建新的获取文档所有块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docx-v1/list-document-blocks
    pub async fn execute(self, params: ListDocumentBlocksParams) -> SDKResult<ListDocumentBlocksResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::DocumentBlockList(params.document_id.clone());

        // 创建API请求
        let mut api_request: ApiRequest<ListDocumentBlocksResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 设置查询参数
        if let Some(page_size) = params.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = &params.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
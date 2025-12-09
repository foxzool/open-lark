//! 获取所有子块
//!
//! 获取文档中指定块的所有子块的富文本内容并分页返回。文档版本号可选。
//! API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/get-2

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取所有子块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlockChildrenParams {
    /// 文档ID
    pub document_id: String,
    /// 父块ID
    pub block_id: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 获取所有子块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlockChildrenResponse {
    /// 子块列表
    pub data: Option<ChildrenListData>,
}

/// 子块列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildrenListData {
    /// 子块列表
    pub items: Vec<ChildBlockItem>,
    /// 分页信息
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

/// 子块项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildBlockItem {
    /// 块ID
    pub block_id: String,
    /// 块类型
    pub block_type: String,
}

impl ApiResponseTrait for GetDocumentBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取所有子块请求
pub struct GetDocumentBlockChildrenRequest {
    config: Config,
}

impl GetDocumentBlockChildrenRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self, params: GetDocumentBlockChildrenParams) -> SDKResult<GetDocumentBlockChildrenResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");

        let url = format!("/open-apis/docx/v1/documents/{}/blocks/{}/children", params.document_id, params.block_id);
        let mut api_request: ApiRequest<GetDocumentBlockChildrenResponse> = ApiRequest::get(&url);

        if let Some(page_size) = params.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = params.page_token {
            api_request = api_request.query("page_token", &page_token);
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
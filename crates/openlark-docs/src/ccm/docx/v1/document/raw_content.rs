/// 获取文档纯文本内容
///
/// 获取文档的纯文本内容。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/raw_content

use crate::common::api_endpoints::DocxApiV1;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取文档纯文本内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRawContentParams {
    /// 文档ID
    pub document_id: String,
}

/// 获取文档纯文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRawContentResponse {
    /// 文档内容
    pub data: Option<DocumentRawContentData>,
}

/// 文档原始内容数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRawContentData {
    /// 纯文本内容
    pub content: String,
    /// 字符数
    pub char_count: u64,
    /// 字节数
    pub byte_count: u64,
}

impl ApiResponseTrait for GetDocumentRawContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档纯文本内容请求
pub struct GetDocumentRawContentRequest {
    config: Config,
}

impl GetDocumentRawContentRequest {
    /// 创建获取文档纯文本内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/raw_content
    pub async fn execute(
        self,
        params: GetDocumentRawContentParams,
    ) -> SDKResult<GetDocumentRawContentResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");

        // 构建API端点
        let api_endpoint = DocxApiV1::DocumentRawContent(params.document_id.clone());

        // 创建API请求
        let api_request: ApiRequest<GetDocumentRawContentResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

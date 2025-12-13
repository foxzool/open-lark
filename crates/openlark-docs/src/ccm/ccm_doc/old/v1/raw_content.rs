/// 获取旧版文档纯文本内容
///
/// 获取文档的纯文本内容，不包含富文本格式信息。
/// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/obtain-document-content
/// 对应CSV记录: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/obtain-document-content
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

/// 获取旧版文档纯文本内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRawContentParams {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
}

/// 获取旧版文档纯文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRawContentResponse {
    /// 文档纯文本内容
    pub data: Option<DocumentRawContent>,
}

/// 文档纯文本内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRawContent {
    /// 纯文本内容
    pub content: String,
}

impl ApiResponseTrait for GetDocumentRawContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档纯文本内容请求
pub struct GetDocumentRawContentRequest {
    config: Config,
}

impl GetDocumentRawContentRequest {
    /// 创建获取旧版文档纯文本内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/obtain-document-content
    /// 对应CSV记录: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/obtain-document-content
    pub async fn execute(
        self,
        params: GetDocumentRawContentParams,
    ) -> SDKResult<GetDocumentRawContentResponse> {
        // 验证必填字段
        validate_required!(params.doc_token, "文档token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDocApiOld::RawContent(params.doc_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetDocumentRawContentResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

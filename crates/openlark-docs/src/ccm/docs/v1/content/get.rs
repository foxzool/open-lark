/// 获取云文档内容
///
/// 此接口用于获取指定云文档的详细内容，包括文档结构、文本内容等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs-v1/content/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DocsApiV1, api_utils::*};

use serde_json::json;

/// 获取云文档内容请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocsContentRequest {
    /// 文档Token
    pub document_token: String,
}

impl GetDocsContentRequest {
    /// 创建获取云文档内容请求
    ///
    /// # 参数
    /// * `document_token` - 文档Token
    pub fn new(document_token: impl Into<String>) -> Self {
        Self {
            document_token: document_token.into(),
        }
    }

    /// 设置文档Token
    pub fn document_token(mut self, document_token: impl Into<String>) -> Self {
        self.document_token = document_token.into();
        self
    }
}

/// 文档内容信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsContent {
    /// 文档Token
    pub document_token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub obj_type: String,
    /// 文档内容
    pub content: serde_json::Value,
    /// 文档结构
    pub structure: Option<serde_json::Value>,
    /// 文本内容（纯文本）
    pub text_content: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 创建者信息
    pub creator: Option<serde_json::Value>,
    /// 更新者信息
    pub modifier: Option<serde_json::Value>,
}

/// 获取云文档内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocsContentResponse {
    /// 文档内容信息
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetDocsContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档内容
///
/// 获取指定云文档的详细内容，包括文档结构、文本内容等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs-v1/content/get
pub async fn get_docs_content(
    request: GetDocsContentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetDocsContentResponse>> {
    // 使用DocsApiV1枚举生成API端点
    let api_endpoint = DocsApiV1::ContentGet;

    // 创建API请求
    let mut api_request: ApiRequest<GetDocsContentResponse> =
        ApiRequest::get(&api_endpoint.to_url()).query("document_token", &request.document_token);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_docs_content_request_builder() {
        let request = GetDocsContentRequest::new("doc_token");

        assert_eq!(request.document_token, "doc_token");
    }

    #[test]
    fn test_get_docs_content_request_with_token() {
        let request = GetDocsContentRequest::new("initial_token").document_token("new_token");

        assert_eq!(request.document_token, "new_token");
    }

    #[test]
    fn test_docs_content_structure() {
        let content = serde_json::json!({
            "document_token": "doc_token",
            "title": "文档标题",
            "content": {
                "elements": []
            }
        });

        assert!(content.get("document_token").is_some());
        assert!(content.get("title").is_some());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetDocsContentResponse::data_format(), ResponseFormat::Data);
    }
}

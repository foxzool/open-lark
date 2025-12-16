use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

/// 创建旧版文档请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDocRequest {
    /// 文件夹 token，获取方式见[文件夹概述](https://open.feishu.cn/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/folder-overview)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 文档内容，JSON 格式，支持的节点类型参考[文档数据结构](https://open.feishu.cn/document/ukTMukTMukTM/uAzM5YjLwMTO24CMzkjN)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl CreateDocRequest {
    /// 创建新的 CreateDocRequest
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 folder_token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 设置 content
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }
}

/// 创建旧版文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocResponse {
    /// 文档 token
    pub obj_token: String,
    /// 文档 URL
    pub url: String,
}

impl ApiResponseTrait for CreateDocResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建旧版文档
///
/// 创建并初始化文档。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/apiRef/create-document
pub async fn create(
    request: CreateDocRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CreateDocResponse>> {
    let api_endpoint = CcmDocApiOld::Create;
    let mut api_request: ApiRequest<CreateDocResponse> = ApiRequest::post(&api_endpoint.to_url())
        .body(serde_json::json!(&request));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_doc_request() {
        let request = CreateDocRequest::new()
            .folder_token("folder_token")
            .content("content");

        assert_eq!(request.folder_token, Some("folder_token".to_string()));
        assert_eq!(request.content, Some("content".to_string()));
    }
}

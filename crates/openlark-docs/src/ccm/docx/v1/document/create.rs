/// 创建文档
///
/// 此接口用于创建新版文档，支持设置文档标题、目录位置和模板。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docx-v1/document/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DocxApiV1, api_utils::*};
use serde_json::json;

/// 创建文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRequest {
    /// 文档标题（可选）
    pub title: Option<String>,
    /// 文档目录ID（可选）
    pub folder_token: Option<String>,
    /// 文档模板ID（可选）
    pub template_id: Option<String>,
    /// 文档类型（可选）
    pub doc_type: Option<String>,
}

impl CreateDocumentRequest {
    /// 创建创建文档请求
    pub fn new() -> Self {
        Self {
            title: None,
            folder_token: None,
            template_id: None,
            doc_type: None,
        }
    }

    /// 设置文档标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置文档目录ID
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 设置文档模板ID
    pub fn template_id(mut self, template_id: impl Into<String>) -> Self {
        self.template_id = Some(template_id.into());
        self
    }

    /// 设置文档类型
    pub fn doc_type(mut self, doc_type: impl Into<String>) -> Self {
        self.doc_type = Some(doc_type.into());
        self
    }
}

/// 文档数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentData {
    /// 文档ID
    pub document_id: String,
    /// 文档token
    pub document_token: String,
    /// 文档标题
    pub title: String,
    /// 文档URL
    pub url: String,
    /// 创建时间
    pub create_time: i64,
    /// 更新时间
    pub update_time: i64,
    /// 创建者信息
    pub creator: Option<UserInfo>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 创建文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentResponse {
    /// 文档信息
    pub data: Option<DocumentData>,
}

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建文档
///
/// 创建新版文档，支持设置文档标题、目录位置和模板。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docx-v1/document/create
pub async fn create_document(
    request: CreateDocumentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateDocumentResponse>> {
    // 创建请求体
    let mut request_body = json!({});

    if let Some(title) = &request.title {
        request_body["title"] = json!(title);
    }
    if let Some(folder_token) = &request.folder_token {
        request_body["folder_token"] = json!(folder_token);
    }
    if let Some(template_id) = &request.template_id {
        request_body["template_id"] = json!(template_id);
    }
    if let Some(doc_type) = &request.doc_type {
        request_body["doc_type"] = json!(doc_type);
    }

    // 使用DocxApiV1枚举生成API端点
    let api_endpoint = DocxApiV1::DocumentCreate;

    // 创建API请求
    let mut api_request: ApiRequest<CreateDocumentResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(request_body);

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
    fn test_create_document_request_builder() {
        let request = CreateDocumentRequest::new()
            .title("文档标题")
            .folder_token("folder_token")
            .template_id("template_id")
            .doc_type("docx");

        assert_eq!(request.title, Some("文档标题".to_string()));
        assert_eq!(request.folder_token, Some("folder_token".to_string()));
        assert_eq!(request.template_id, Some("template_id".to_string()));
        assert_eq!(request.doc_type, Some("docx".to_string()));
    }

    #[test]
    fn test_create_document_request_minimal() {
        let request = CreateDocumentRequest::new();

        assert_eq!(request.title, None);
        assert_eq!(request.folder_token, None);
        assert_eq!(request.template_id, None);
        assert_eq!(request.doc_type, None);
    }

    #[test]
    fn test_document_data_structure() {
        let user_info = UserInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
            email: Some("email@example.com".to_string()),
        };

        let document_data = DocumentData {
            document_id: "doc_id".to_string(),
            document_token: "doc_token".to_string(),
            title: "文档标题".to_string(),
            url: "https://example.com".to_string(),
            create_time: 1640995200,
            update_time: 1640995200,
            creator: Some(user_info),
        };

        assert_eq!(document_data.document_id, "doc_id");
        assert_eq!(document_data.creator.as_ref().unwrap().name, "用户名");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateDocumentResponse::data_format(), ResponseFormat::Data);
    }
}

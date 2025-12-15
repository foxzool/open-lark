/// 获取文档元数据
///
/// 获取指定云文档的元数据信息，包括标题、类型、创建者、大小等。
/// docPath: https://open.feishu.cn/open-apis/suite/docs-api/meta
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::CcmDocsApiOld, api_utils::*};

/// 获取文档元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocMetaRequest {
    /// 文档token
    pub token: String,
    /// 需要返回的字段
    pub fields: Option<Vec<String>>,
}

impl GetDocMetaRequest {
    /// 创建获取文档元数据请求
    ///
    /// # 参数
    /// * `token` - 文档token
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            fields: None,
        }
    }

    /// 设置需要返回的字段
    pub fn fields(mut self, fields: Vec<String>) -> Self {
        self.fields = Some(fields);
        self
    }
}

/// 获取文档元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocMetaResponse {
    /// 文档元数据
    pub data: Option<DocMeta>,
}

impl ApiResponseTrait for GetDocMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文档元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocMeta {
    /// 文档token
    pub token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub doc_type: String,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub modify_time: String,
    /// 创建者信息
    pub creator: CreatorInfo,
    /// 父目录token
    pub parent_token: Option<String>,
    /// 文档大小
    pub size: Option<i64>,
    /// 文档URL
    pub url: Option<String>,
    /// 额外属性
    pub extra: Option<serde_json::Value>,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
}

/// 获取文档元数据
///
/// 获取指定云文档的元数据信息，包括标题、类型、创建者、大小等。
/// docPath: https://open.feishu.cn/open-apis/suite/docs-api/meta
pub async fn get_doc_meta(
    request: GetDocMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetDocMetaResponse>> {
    // 使用CcmDocsApiOld枚举生成API端点
    let api_endpoint = CcmDocsApiOld::Meta;

    // 构建请求体
    let mut body = json!({
        "token": request.token
    });

    if let Some(fields) = &request.fields {
        body["fields"] = json!(fields);
    }

    // 创建API请求
    let mut api_request: ApiRequest<GetDocMetaResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(body);

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
    fn test_get_doc_meta_request_builder() {
        let request = GetDocMetaRequest::new("doc_token");

        assert_eq!(request.token, "doc_token");
    }

    #[test]
    fn test_get_doc_meta_request_with_fields() {
        let fields = vec!["title".to_string(), "doc_type".to_string()];
        let request = GetDocMetaRequest::new("doc_token").fields(fields.clone());

        assert_eq!(request.token, "doc_token");
        assert_eq!(request.fields, Some(fields));
    }

    #[test]
    fn test_doc_meta_structure() {
        let creator = CreatorInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
        };

        let doc_meta = DocMeta {
            token: "doc_token".to_string(),
            title: "文档标题".to_string(),
            doc_type: "docx".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            modify_time: "2023-01-02T00:00:00Z".to_string(),
            creator,
            parent_token: Some("parent_token".to_string()),
            size: Some(1024),
            url: Some("https://example.com".to_string()),
            extra: None,
        };

        assert_eq!(doc_meta.token, "doc_token");
        assert_eq!(doc_meta.title, "文档标题");
        assert_eq!(doc_meta.creator.name, "用户名");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetDocMetaResponse::data_format(), ResponseFormat::Data);
    }
}
use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取云文档内容请求
#[derive(Debug, Serialize, Default)]
pub struct GetDocsContentRequest {
    /// 文档token
    pub token: String,
    /// 文档版本ID
    pub version_id: Option<String>,
    /// 需要返回的字段
    pub fields: Option<Vec<String>>,
}

/// 获取云文档内容响应
#[derive(Debug, Deserialize, Default)]
pub struct GetDocsContentResponse {
    /// 文档内容
    pub content: DocContent,
    /// 操作结果
    pub result: String,
}

/// 文档内容
#[derive(Debug, Deserialize, Default)]
pub struct DocContent {
    /// 文档token
    pub token: String,
    /// 文档标题
    pub title: String,
    /// 文档版本ID
    pub version_id: String,
    /// 文档内容
    pub body: Option<Vec<DocElement>>,
    /// 文档元数据
    pub meta: Option<serde_json::Value>,
}

/// 文档元素
#[derive(Debug, Deserialize, Default)]
pub struct DocElement {
    /// 元素类型
    pub r#type: String,
    /// 元素内容
    pub content: Option<serde_json::Value>,
    /// 子元素列表
    pub elements: Option<Vec<DocElement>>,
}

/// 获取云文档内容
/// docPath: https://open.feishu.cn/open-apis/docs/v1/content
pub async fn get_docs_content(
    request: GetDocsContentRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetDocsContentResponse>> {
    let url = format!(
        "{}/open-apis/docs/v1/content",
        config.base_url
    );

    let mut query_params = vec![
        ("token".to_string(), request.token)
    ];

    if let Some(version_id) = &request.version_id {
        query_params.push(("version_id".to_string(), version_id.clone()));
    }

    if let Some(fields) = &request.fields {
        query_params.push(("fields".to_string(), fields.join(",")));
    }

    let req = OpenLarkRequest {
        url,
        method: http::Method::GET,
        headers: vec![],
        query_params,
        body: None,
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_docs_content() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetDocsContentRequest {
            token: "test_doc_token".to_string(),
            version_id: None,
            fields: Some(vec!["title".to_string(), "body".to_string()]),
        };

        let result = get_docs_content(request, &config, None).await;
        assert!(result.is_ok());
    }
}
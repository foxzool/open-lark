use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取文档元数据请求
#[derive(Debug, Serialize, Default)]
pub struct GetDocMetaRequest {
    /// 文档token
    pub token: String,
    /// 需要返回的字段
    pub fields: Option<Vec<String>>,
}

/// 获取文档元数据响应
#[derive(Debug, Deserialize, Default)]
pub struct GetDocMetaResponse {
    /// 文档元数据
    pub doc_meta: DocMeta,
    /// 操作结果
    pub result: String,
}

/// 文档元数据
#[derive(Debug, Deserialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
}

/// 获取文档元数据
/// docPath: https://open.feishu.cn/open-apis/suite/docs-api/meta
pub async fn get_doc_meta(
    request: GetDocMetaRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetDocMetaResponse>> {
    let url = format!(
        "{}/open-apis/suite/docs-api/meta",
        config.base_url
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::POST,
        headers: vec![],
        query_params: vec![],
        body: Some(serde_json::to_vec(&request)?),
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_doc_meta() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetDocMetaRequest {
            token: "test_doc_token".to_string(),
            fields: Some(vec!["title".to_string(), "doc_type".to_string(), "size".to_string()]),
        };

        let result = get_doc_meta(request, &config, None).await;
        assert!(result.is_ok());
    }
}
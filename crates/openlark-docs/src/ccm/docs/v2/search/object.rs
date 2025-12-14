use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 搜索云文档请求
#[derive(Debug, Serialize, Default)]
pub struct SearchDocsRequest {
    /// 搜索关键词
    pub query: String,
    /// 搜索类型
    pub search_type: Option<String>,
    /// 文件类型过滤
    pub file_type: Option<String>,
    /// 父目录token
    pub parent_token: Option<String>,
    /// 创建者ID
    pub creator_id: Option<String>,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 搜索云文档响应
#[derive(Debug, Deserialize, Default)]
pub struct SearchDocsResponse {
    /// 文档列表
    pub items: Vec<DocItem>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 操作结果
    pub result: String,
}

/// 文档项
#[derive(Debug, Deserialize, Default)]
pub struct DocItem {
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
}

/// 创建者信息
#[derive(Debug, Deserialize, Default)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
}

/// 搜索云文档
/// docPath: https://open.feishu.cn/open-apis/suite/docs-api/search/object
pub async fn search_docs(
    request: SearchDocsRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<SearchDocsResponse>> {
    let url = format!(
        "{}/open-apis/suite/docs-api/search/object",
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
    async fn test_search_docs() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = SearchDocsRequest {
            query: "测试文档".to_string(),
            search_type: Some("doc".to_string()),
            file_type: None,
            parent_token: None,
            creator_id: None,
            page_size: Some(20),
            page_token: None,
        };

        let result = search_docs(request, &config, None).await;
        assert!(result.is_ok());
    }
}
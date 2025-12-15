/// 搜索云文档
///
/// 根据关键词搜索云文档，支持多种过滤条件。
/// docPath: https://open.feishu.cn/open-apis/suite/docs-api/search/object
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::CcmDocsApiOld, api_utils::*};

/// 搜索云文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl SearchDocsRequest {
    /// 创建搜索云文档请求
    ///
    /// # 参数
    /// * `query` - 搜索关键词
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            search_type: None,
            file_type: None,
            parent_token: None,
            creator_id: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置搜索类型
    pub fn search_type(mut self, search_type: impl Into<String>) -> Self {
        self.search_type = Some(search_type.into());
        self
    }

    /// 设置文件类型过滤
    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = Some(file_type.into());
        self
    }

    /// 设置父目录token
    pub fn parent_token(mut self, parent_token: impl Into<String>) -> Self {
        self.parent_token = Some(parent_token.into());
        self
    }

    /// 设置创建者ID
    pub fn creator_id(mut self, creator_id: impl Into<String>) -> Self {
        self.creator_id = Some(creator_id.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }
}

/// 搜索云文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocsResponse {
    /// 文档列表
    pub data: Option<SearchData>,
}

impl ApiResponseTrait for SearchDocsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchData {
    /// 文档列表
    pub items: Vec<DocItem>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 文档项
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
}

/// 搜索云文档
///
/// 根据关键词搜索云文档，支持多种过滤条件。
/// docPath: https://open.feishu.cn/open-apis/suite/docs-api/search/object
pub async fn search_docs(
    request: SearchDocsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<SearchDocsResponse>> {
    // 使用CcmDocsApiOld枚举生成API端点
    let api_endpoint = CcmDocsApiOld::SearchObject;

    // 构建请求体
    let mut body = json!({
        "query": request.query
    });

    if let Some(search_type) = &request.search_type {
        body["search_type"] = json!(search_type);
    }
    if let Some(file_type) = &request.file_type {
        body["file_type"] = json!(file_type);
    }
    if let Some(parent_token) = &request.parent_token {
        body["parent_token"] = json!(parent_token);
    }
    if let Some(creator_id) = &request.creator_id {
        body["creator_id"] = json!(creator_id);
    }
    if let Some(page_size) = &request.page_size {
        body["page_size"] = json!(page_size);
    }
    if let Some(page_token) = &request.page_token {
        body["page_token"] = json!(page_token);
    }

    // 创建API请求
    let mut api_request: ApiRequest<SearchDocsResponse> =
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
    fn test_search_docs_request_builder() {
        let request = SearchDocsRequest::new("测试文档");

        assert_eq!(request.query, "测试文档");
    }

    #[test]
    fn test_search_docs_request_builder_chain() {
        let request = SearchDocsRequest::new("测试文档")
            .search_type("doc")
            .file_type("docx")
            .page_size(20);

        assert_eq!(request.query, "测试文档");
        assert_eq!(request.search_type, Some("doc".to_string()));
        assert_eq!(request.file_type, Some("docx".to_string()));
        assert_eq!(request.page_size, Some(20));
    }

    #[test]
    fn test_doc_item_structure() {
        let creator = CreatorInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
        };

        let doc_item = DocItem {
            token: "doc_token".to_string(),
            title: "文档标题".to_string(),
            doc_type: "docx".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            modify_time: "2023-01-02T00:00:00Z".to_string(),
            creator: creator.clone(),
            parent_token: Some("parent_token".to_string()),
        };

        assert_eq!(doc_item.token, "doc_token");
        assert_eq!(doc_item.title, "文档标题");
        assert_eq!(doc_item.creator.name, "用户名");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(SearchDocsResponse::data_format(), ResponseFormat::Data);
    }
}
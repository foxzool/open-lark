/// 搜索文档
///
/// 此接口用于根据搜索条件进行文档搜索，支持关键词、类型、时间等过滤条件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::CcmDocsApiOld, api_utils::*};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// 搜索文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectRequest {
    /// 搜索关键字
    pub query: String,
    /// 文档类型过滤
    pub obj_type: Option<String>,
    /// 页面大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 排序字段
    pub sort_field: Option<String>,
    /// 排序方式
    pub sort_order: Option<String>,
}

impl SearchObjectRequest {
    /// 创建搜索文档请求
    ///
    /// # 参数
    /// * `query` - 搜索关键字
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            obj_type: None,
            page_size: None,
            page_token: None,
            sort_field: None,
            sort_order: None,
        }
    }

    /// 设置文档类型过滤
    pub fn obj_type(mut self, obj_type: impl Into<String>) -> Self {
        self.obj_type = Some(obj_type.into());
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置排序字段
    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.sort_field = Some(sort_field.into());
        self
    }

    /// 设置排序方式
    pub fn sort_order(mut self, sort_order: impl Into<String>) -> Self {
        self.sort_order = Some(sort_order.into());
        self
    }
}

/// 搜索文档结果项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectItem {
    /// 文档ID
    pub document_id: String,
    /// 文档token
    pub document_token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub obj_type: String,
    /// 文档URL
    pub url: String,
    /// 文档所有者
    pub owner: Option<UserInfo>,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 文档大小
    pub size: Option<i64>,
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

/// 搜索文档数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectData {
    /// 搜索结果列表
    pub items: Vec<SearchObjectItem>,
    /// 分页token
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: bool,
    /// 总数
    pub total: Option<i32>,
}

/// 搜索文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectResponse {
    /// 搜索结果数据
    pub data: Option<SearchObjectData>,
}

impl ApiResponseTrait for SearchObjectResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索文档
///
/// 根据搜索条件进行文档搜索，支持关键词、类型、时间等过滤条件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search
pub async fn search_object(
    request: SearchObjectRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<SearchObjectResponse>> {
    // 创建请求体
    let mut request_body = json!({
        "query": request.query
    });

    if let Some(obj_type) = &request.obj_type {
        request_body["obj_type"] = json!(obj_type);
    }
    if let Some(page_size) = &request.page_size {
        request_body["page_size"] = json!(page_size);
    }
    if let Some(page_token) = &request.page_token {
        request_body["page_token"] = json!(page_token);
    }
    if let Some(sort_field) = &request.sort_field {
        request_body["sort_field"] = json!(sort_field);
    }
    if let Some(sort_order) = &request.sort_order {
        request_body["sort_order"] = json!(sort_order);
    }

    // 使用CcmDocsApiOld枚举生成API端点
    let api_endpoint = CcmDocsApiOld::SearchObject;

    // 创建API请求
    let mut api_request: ApiRequest<SearchObjectResponse> =
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
    fn test_search_object_request_builder() {
        let request = SearchObjectRequest::new("测试关键词")
            .obj_type("doc")
            .page_size(20)
            .sort_field("create_time")
            .sort_order("desc");

        assert_eq!(request.query, "测试关键词");
        assert_eq!(request.obj_type, Some("doc".to_string()));
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.sort_field, Some("create_time".to_string()));
        assert_eq!(request.sort_order, Some("desc".to_string()));
    }

    #[test]
    fn test_search_object_request_minimal() {
        let request = SearchObjectRequest::new("关键词");

        assert_eq!(request.query, "关键词");
        assert_eq!(request.obj_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_search_object_item_structure() {
        let user_info = UserInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
            email: Some("email@example.com".to_string()),
        };

        let item = SearchObjectItem {
            document_id: "doc_id".to_string(),
            document_token: "doc_token".to_string(),
            title: "文档标题".to_string(),
            obj_type: "doc".to_string(),
            url: "https://example.com".to_string(),
            owner: Some(user_info),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            update_time: "2023-01-01T00:00:00Z".to_string(),
            size: Some(1024),
        };

        assert_eq!(item.document_id, "doc_id");
        assert_eq!(item.owner.as_ref().unwrap().name, "用户名");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(SearchObjectResponse::data_format(), ResponseFormat::Data);
    }
}

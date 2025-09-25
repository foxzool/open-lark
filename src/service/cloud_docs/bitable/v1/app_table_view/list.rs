use reqwest::Method;
use serde::Deserialize;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use super::AppTableViewService;

impl AppTableViewService {
    /// 列出视图
    pub async fn list(
        &self,
        request: ListViewsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListViewsResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = BITABLE_V1_VIEWS
            .replace("{app_token}", &request.app_token)
            .replace("{table_id}", &request.table_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 列出视图请求
#[derive(Debug, Default, Clone)]
pub struct ListViewsRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 分页标记，第一次请求不填，表示从头开始遍历
    page_token: Option<String>,
    /// 分页大小，最大值是 100
    page_size: Option<i32>,
}

impl ListViewsRequest {
    pub fn builder() -> ListViewsRequestBuilder {
        ListViewsRequestBuilder::default()
    }

    /// 创建列出视图请求
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            page_token: None,
            page_size: None,
        }
    }
}

#[derive(Default)]
pub struct ListViewsRequestBuilder {
    request: ListViewsRequest,
}

impl ListViewsRequestBuilder {
    /// 多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 数据表的 table_id
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 分页大小，最大值是 100
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size.min(100));
        self
    }

    pub fn build(self) -> ListViewsRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait到ListViewsRequestBuilder
crate::impl_executable_builder_owned!(
    ListViewsRequestBuilder,
    super::AppTableViewService,
    ListViewsRequest,
    BaseResponse<ListViewsResponse>,
    list
);

#[derive(Deserialize, Debug)]
pub struct ListViewsResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token
    pub page_token: Option<String>,
    /// 总数量
    pub total: i32,
    /// 视图信息列表
    pub items: Vec<ViewInfo>,
}

#[derive(Deserialize, Debug)]
pub struct ViewInfo {
    /// 视图 ID
    pub view_id: String,
    /// 视图名称
    pub view_name: String,
    /// 视图类型
    pub view_type: String,
    /// 视图的自定义属性
    #[serde(default)]
    pub property: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListViewsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_views_request() {
        let request = ListViewsRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .page_size(50)
            .page_token("next_page_token")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
    }

    #[test]
    fn test_list_views_request_new() {
        let request = ListViewsRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_page_size_limit() {
        let request = ListViewsRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .page_size(150) // 超过最大值100
            .build();

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100
    }
}

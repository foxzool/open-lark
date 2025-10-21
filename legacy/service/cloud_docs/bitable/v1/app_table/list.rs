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

use super::AppTableService;

impl AppTableService {
    /// 列出数据表
    pub async fn list(
        &self,
        request: ListTablesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListTablesResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = BITABLE_V1_TABLES.replace("{app_token}", &request.app_token);
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

/// 列出数据表请求
#[derive(Debug, Default)]
pub struct ListTablesRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 分页标记，第一次请求不填，表示从头开始遍历
    page_token: Option<String>,
    /// 分页大小，最大值是 100
    page_size: Option<i32>,
}

impl ListTablesRequest {
    pub fn builder() -> ListTablesRequestBuilder {
        ListTablesRequestBuilder::default()
    }

    /// 创建列出数据表请求
    pub fn new(app_token: impl ToString) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
            page_token: None,
            page_size: None,
        }
    }
}

#[derive(Default)]
pub struct ListTablesRequestBuilder {
    request: ListTablesRequest,
}

impl ListTablesRequestBuilder {
    /// 多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
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

    pub fn build(self) -> ListTablesRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait到ListTablesRequestBuilder
crate::impl_executable_builder_owned!(
    ListTablesRequestBuilder,
    super::AppTableService,
    ListTablesRequest,
    BaseResponse<ListTablesResponse>,
    list
);

#[derive(Deserialize, Debug)]
pub struct ListTablesResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token
    pub page_token: Option<String>,
    /// 总数量
    pub total: i32,
    /// 数据表信息列表
    pub items: Vec<TableInfo>,
}

#[derive(Deserialize, Debug)]
pub struct TableInfo {
    /// 数据表的 table_id
    pub table_id: String,
    /// 数据表的版本号
    pub revision: i32,
    /// 数据表的名称
    pub name: String,
}

impl ApiResponseTrait for ListTablesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_tables_request() {
        let request = ListTablesRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .page_size(50)
            .page_token("next_page_token")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
    }

    #[test]
    fn test_list_tables_request_new() {
        let request = ListTablesRequest::new("bascnmBA*****yGehy8");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_page_size_limit() {
        let request = ListTablesRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .page_size(150) // 超过最大值100
            .build();

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100
    }
}

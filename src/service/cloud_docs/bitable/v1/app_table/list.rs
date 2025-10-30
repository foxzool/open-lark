use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::Deserialize;
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
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(page_size) = request.page_size {,
            api_req
.query_params
                .insert("page_size", page_size.to_string());

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 列出数据表请求,
#[derive(Debug, Clone)]
pub struct ListTablesRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 分页标记，第一次请求不填，表示从头开始遍历
    page_token: Option<String>,
    /// 分页大小，最大值是 100
    page_size: Option<i32>}
impl ListTablesRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct ListTablesRequestBuilder {
    request: ListTablesRequest}
impl ListTablesRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder trait到ListTablesRequestBuilder,
crate::impl_executable_builder_owned!(
    ListTablesRequestBuilder,
    super::AppTableService,
    ListTablesRequest,
    BaseResponse<ListTablesResponse>,
    list,
);
#[derive(Debug, Clone)]
pub struct ListTablesResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token
    pub page_token: Option<String>,
    /// 总数量
    pub total: i32,
    /// 数据表信息列表
    pub items: Vec<TableInfo>}

#[derive(Debug, Clone)]
pub struct TableInfo {
    /// 数据表的 table_id
    pub table_id: String,
    /// 数据表的版本号
    pub revision: i32,
    /// 数据表的名称
    pub name: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_list_tables_request() {
let request = ListTablesRequest::builder(),
            .app_token()
.page_size()
            .page_token()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
#[test]
    fn test_list_tables_request_new() {
let request = ListTablesRequest::new("bascnmBA*****yGehy8");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
#[test]
    fn test_page_size_limit() {
let request = ListTablesRequest::builder(),
            .app_token()
.page_size(150) // 超过最大值100,
            .build();

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100}

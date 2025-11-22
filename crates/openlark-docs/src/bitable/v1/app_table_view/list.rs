#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::Deserialize;
use openlark_core::{
use serde_json::Value;
use SDKResult;    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use super::AppTableViewService;
impl AppTableViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(page_size) = request.page_size {,
            api_req
.query_params
                .insert("page_size", page_size.to_string());

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 列出视图请求,
#[derive(Clone)]
pub struct ListViewsRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 分页标记，第一次请求不填，表示从头开始遍历
    page_token: Option<String>,
    /// 分页大小，最大值是 100
    page_size: Option<i32>}
impl ListViewsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct ListViewsRequestBuilder {
    request: ListViewsRequest}
impl ListViewsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder trait到ListViewsRequestBuilder,
crate::impl_executable_builder_owned!(
    ListViewsRequestBuilder,
    super::AppTableViewService,
    ListViewsRequest,
    Response<ListViewsResponse>,
    list,
);
#[derive(Clone)]
pub struct ListViewsResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token
    pub page_token: Option<String>,
    /// 总数量
    pub total: i32,
    /// 视图信息列表
    pub items: Vec<ViewInfo>}

#[derive(Clone)]
pub struct ViewInfo {
    /// 视图 ID
    pub view_id: String,
    /// 视图名称
    pub view_name: String,
    /// 视图类型
    pub view_type: String,
    /// 视图的自定义属性,
#[serde(default)]
    pub property: Option<serde_json::Value>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_list_views_request() {
let request = ListViewsRequest::builder(),
            .app_token()
.table_id()
            .page_size()
.page_token()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
#[test]
    ,
        let request = ListViewsRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
#[test]
    fn test_page_size_limit() {
let request = ListViewsRequest::builder(),
            .app_token()
.table_id()
            .page_size(150) // 超过最大值100,
.build();
        assert_eq!(request.page_size, Some(100)); // 应该被限制为100}

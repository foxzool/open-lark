use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::Deserialize;
use crate::,
{,
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
use super::AppTableViewService;
impl AppTableViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取视图请求,
#[derive(Debug, Clone)]
pub struct GetViewRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图的 view_id
    view_id: String}
impl GetViewRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct GetViewRequestBuilder {
    request: GetViewRequest}
impl GetViewRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    GetViewRequestBuilder,
    AppTableViewService,
    GetViewRequest,
    BaseResponse<GetViewResponse>,
    get,
);
#[derive(Debug, Clone)]
pub struct GetViewResponse {
    /// 视图信息
    pub view: ViewDetailInfo,

#[derive(Debug, Clone)]
pub struct ViewDetailInfo {
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
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_get_view_request() {
let request = GetViewRequest::builder(),
            .app_token()
.table_id()
            .view_id()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");
#[test]
    ,
        let request = GetViewRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW", "vewTpR1urY");

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");

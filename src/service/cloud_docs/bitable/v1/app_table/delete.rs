#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use reqwest::Method;
use openlark_core::api_req::ApiRequest;use serde::Deserialize;
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
use super::AppTableService;
impl AppTableService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除数据表请求,
#[derive(Debug, Clone)]
pub struct DeleteTableRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String}
impl DeleteTableRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeleteTableRequestBuilder {
    request: DeleteTableRequest}
impl DeleteTableRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    DeleteTableRequestBuilder,
    AppTableService,
    DeleteTableRequest,
    BaseResponse<DeleteTableResponse>,
    delete,
);
#[derive(Debug, Clone)]
pub struct DeleteTableResponse {
    /// 删除的数据表ID
    pub deleted: bool,
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
    fn test_delete_table_request() {
let request = DeleteTableRequest::builder(),
            .app_token()
.table_id()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
#[test]
    ,
        let request = DeleteTableRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");

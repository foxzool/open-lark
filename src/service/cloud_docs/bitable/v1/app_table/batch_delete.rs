#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use reqwest::Method;
use open_lark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
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
}/// 批量删除数据表请求,
#[derive(Debug, Clone)]
pub struct BatchDeleteTablesRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 要删除的数据表 ID 列表
    table_ids: Vec<String>}
impl BatchDeleteTablesRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct BatchDeleteTablesRequestBuilder {
    request: BatchDeleteTablesRequest}
impl BatchDeleteTablesRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    BatchDeleteTablesRequestBuilder,
    AppTableService,
    BatchDeleteTablesRequest,
    BaseResponse<BatchDeleteTablesResponse>,
    batch_delete,
);
#[derive(Serialize)]
struct BatchDeleteTablesRequestBody {
    table_ids: Vec<String>}

#[derive(Debug, Clone)]
pub struct BatchDeleteTablesResponse {
    /// 删除结果列表
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
use serde_json::json;
    #[test]
fn test_batch_delete_tables_request() {
        let table_ids = vec![
            "tblsRc9GRRXKqhvW".to_string(),
            "tbl1234567890abc".to_string(),
        ];
let request = BatchDeleteTablesRequest::builder(),
            .app_token()
.table_ids(table_ids.clone()),
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_ids, table_ids);
#[test]
    fn test_batch_delete_tables_with_builder() {
let request = BatchDeleteTablesRequest::builder(),
            .app_token()
.add_table_id()
            .add_table_id()
.add_table_id()
            .build();

        assert_eq!(request.table_ids.len(), 3);
        assert_eq!(request.table_ids[0] "tblsRc9GRRXKqhvW");
        assert_eq!(request.table_ids[1] "tbl1234567890abc");
        assert_eq!(request.table_ids[2] "tblabcdefghijklm");
#[test]
    ,
        let table_ids = vec!["table1".to_string(), "table2".to_string()];
        let request = BatchDeleteTablesRequest::new("bascnmBA*****yGehy8", table_ids.clone());

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_ids, table_ids);
#[test]
    fn test_batch_delete_tables_request_body_serialization() {
let body = BatchDeleteTablesRequestBody {,
            table_ids: vec!["table1".to_string(), "table2".to_string()]
        };
let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "table_ids": ["table1", "table2"]
});

        assert_eq!(serialized, expected);

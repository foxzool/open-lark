use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
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

use super::{AppTableService, TableData};
impl AppTableService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 批量新增数据表请求,
#[derive(Debug, Clone)]
pub struct BatchCreateTablesRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表信息列表
    tables: Vec<TableData>}
impl BatchCreateTablesRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct BatchCreateTablesRequestBuilder {
    request: BatchCreateTablesRequest}
impl BatchCreateTablesRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    BatchCreateTablesRequestBuilder,
    AppTableService,
    BatchCreateTablesRequest,
    BaseResponse<BatchCreateTablesResponse>,
    batch_create,
);
#[derive(Serialize)]
struct BatchCreateTablesRequestBody {
    tables: Vec<TableData>}

#[derive(Debug, Clone)]
pub struct BatchCreateTablesResponse {
    /// 数据表信息列表
    pub table_ids: Vec<String>}
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
use crate::service::bitable::v1::app_table::TableField;
    #[test]
fn test_batch_create_tables_request() {
        let table1 = TableData::new("用户表")
            .with_fields(vec![TableField::text("姓名"), TableField::text("邮箱")]);
let table2 = TableData::new("订单表").with_fields(vec![,
            TableField::text("订单号"),
            TableField::number("金额"),
            TableField::single_select("状态", vec!["待支付".to_string(), "已支付".to_string()]),
        ]);
let request = BatchCreateTablesRequest::builder(),
            .app_token()
.add_table()
            .add_table()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.tables.len(), 2);
        assert_eq!(request.tables[0].name, "用户表");
        assert_eq!(request.tables[1].name, "订单表");
#[test]
    fn test_batch_create_tables_with_vec() {
let tables = vec![,
            TableData::new("表格1"),
            TableData::new("表格2"),
            TableData::new("表格3"),
        ];

        let request = BatchCreateTablesRequest::new("bascnmBA*****yGehy8", tables);
        assert_eq!(request.tables.len(), 3);

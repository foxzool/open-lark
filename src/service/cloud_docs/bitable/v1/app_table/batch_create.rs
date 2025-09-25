use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::{AppTableService, TableData};

impl AppTableService {
    /// 新增多个数据表
    pub async fn batch_create(
        &self,
        request: BatchCreateTablesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchCreateTablesResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path =
            BITABLE_V1_TABLES_BATCH_CREATE.replace("{app_token}", &request.app_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = serde_json::to_vec(&BatchCreateTablesRequestBody {
            tables: request.tables,
        })?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 批量新增数据表请求
#[derive(Debug, Default)]
pub struct BatchCreateTablesRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表信息列表
    tables: Vec<TableData>,
}

impl BatchCreateTablesRequest {
    pub fn builder() -> BatchCreateTablesRequestBuilder {
        BatchCreateTablesRequestBuilder::default()
    }

    /// 创建批量新增数据表请求
    pub fn new(app_token: impl ToString, tables: Vec<TableData>) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
            tables,
        }
    }
}

#[derive(Default)]
pub struct BatchCreateTablesRequestBuilder {
    request: BatchCreateTablesRequest,
}

impl BatchCreateTablesRequestBuilder {
    /// 多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 数据表信息列表
    pub fn tables(mut self, tables: Vec<TableData>) -> Self {
        self.request.tables = tables;
        self
    }

    /// 添加单个数据表
    pub fn add_table(mut self, table: TableData) -> Self {
        self.request.tables.push(table);
        self
    }

    pub fn build(self) -> BatchCreateTablesRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    BatchCreateTablesRequestBuilder,
    AppTableService,
    BatchCreateTablesRequest,
    BaseResponse<BatchCreateTablesResponse>,
    batch_create
);

#[derive(Serialize)]
struct BatchCreateTablesRequestBody {
    tables: Vec<TableData>,
}

#[derive(Deserialize, Debug)]
pub struct BatchCreateTablesResponse {
    /// 数据表信息列表
    pub table_ids: Vec<String>,
}

impl ApiResponseTrait for BatchCreateTablesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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

        let table2 = TableData::new("订单表").with_fields(vec![
            TableField::text("订单号"),
            TableField::number("金额"),
            TableField::single_select("状态", vec!["待支付".to_string(), "已支付".to_string()]),
        ]);

        let request = BatchCreateTablesRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .add_table(table1)
            .add_table(table2)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.tables.len(), 2);
        assert_eq!(request.tables[0].name, "用户表");
        assert_eq!(request.tables[1].name, "订单表");
    }

    #[test]
    fn test_batch_create_tables_with_vec() {
        let tables = vec![
            TableData::new("表格1"),
            TableData::new("表格2"),
            TableData::new("表格3"),
        ];

        let request = BatchCreateTablesRequest::new("bascnmBA*****yGehy8", tables);
        assert_eq!(request.tables.len(), 3);
    }
}

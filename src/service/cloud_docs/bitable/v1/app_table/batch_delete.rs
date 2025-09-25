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

use super::AppTableService;

impl AppTableService {
    /// 删除多个数据表
    pub async fn batch_delete(
        &self,
        request: BatchDeleteTablesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchDeleteTablesResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path =
            BITABLE_V1_TABLES_BATCH_DELETE.replace("{app_token}", &request.app_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = serde_json::to_vec(&BatchDeleteTablesRequestBody {
            table_ids: request.table_ids,
        })?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 批量删除数据表请求
#[derive(Debug, Default)]
pub struct BatchDeleteTablesRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 要删除的数据表 ID 列表
    table_ids: Vec<String>,
}

impl BatchDeleteTablesRequest {
    pub fn builder() -> BatchDeleteTablesRequestBuilder {
        BatchDeleteTablesRequestBuilder::default()
    }

    /// 创建批量删除数据表请求
    pub fn new(app_token: impl ToString, table_ids: Vec<String>) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
            table_ids,
        }
    }
}

#[derive(Default)]
pub struct BatchDeleteTablesRequestBuilder {
    request: BatchDeleteTablesRequest,
}

impl BatchDeleteTablesRequestBuilder {
    /// 多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 要删除的数据表 ID 列表
    pub fn table_ids(mut self, table_ids: Vec<String>) -> Self {
        self.request.table_ids = table_ids;
        self
    }

    /// 添加单个数据表 ID
    pub fn add_table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_ids.push(table_id.to_string());
        self
    }

    pub fn build(self) -> BatchDeleteTablesRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    BatchDeleteTablesRequestBuilder,
    AppTableService,
    BatchDeleteTablesRequest,
    BaseResponse<BatchDeleteTablesResponse>,
    batch_delete
);

#[derive(Serialize)]
struct BatchDeleteTablesRequestBody {
    table_ids: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct BatchDeleteTablesResponse {
    /// 删除结果列表
    pub deleted: bool,
}

impl ApiResponseTrait for BatchDeleteTablesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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

        let request = BatchDeleteTablesRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_ids(table_ids.clone())
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_ids, table_ids);
    }

    #[test]
    fn test_batch_delete_tables_with_builder() {
        let request = BatchDeleteTablesRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .add_table_id("tblsRc9GRRXKqhvW")
            .add_table_id("tbl1234567890abc")
            .add_table_id("tblabcdefghijklm")
            .build();

        assert_eq!(request.table_ids.len(), 3);
        assert_eq!(request.table_ids[0], "tblsRc9GRRXKqhvW");
        assert_eq!(request.table_ids[1], "tbl1234567890abc");
        assert_eq!(request.table_ids[2], "tblabcdefghijklm");
    }

    #[test]
    fn test_batch_delete_tables_request_new() {
        let table_ids = vec!["table1".to_string(), "table2".to_string()];
        let request = BatchDeleteTablesRequest::new("bascnmBA*****yGehy8", table_ids.clone());

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_ids, table_ids);
    }

    #[test]
    fn test_batch_delete_tables_request_body_serialization() {
        let body = BatchDeleteTablesRequestBody {
            table_ids: vec!["table1".to_string(), "table2".to_string()],
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "table_ids": ["table1", "table2"]
        });

        assert_eq!(serialized, expected);
    }
}

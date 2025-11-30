#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use openlark_core::{
    api::ApiRequest,
    core::{BaseResponse, ResponseFormat, api::ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    service::bitable::v1::Record,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除记录请求
#[derive(Clone)]
pub struct DeleteRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 记录的唯一标识符
    #[serde(skip)]
    record_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
}

impl DeleteRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::DELETE, "/open-apis/bitable/v1/apps/{}/tables/{}/records/{}".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
        }
    }

    pub fn builder() -> DeleteRecordRequestBuilder {
        DeleteRecordRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteRecordRequestBuilder {
    request: DeleteRecordRequest,
}

impl DeleteRecordRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    pub fn record_id(mut self, record_id: impl Into<String>) -> Self {
        self.request.record_id = record_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn build(self) -> DeleteRecordRequest {
        self.request
    }
}

/// 删除记录响应
#[derive(Clone)]
pub struct DeleteRecordResponse {
    /// 被删除的记录
    pub record: Record,
}

impl ApiResponseTrait for DeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除记录
pub async fn delete_record(
    request: DeleteRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<DeleteRecordResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
    api_req.api_path = BITABLE_V1_RECORD
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{record_id}", &request.record_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    let api_resp: openlark_core::core::StandardResponse<DeleteRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_record_request_builder() {
        let request = DeleteRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .record_id("rec123456")
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.record_id, "rec123456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_delete_record_request_minimal() {
        let request = DeleteRecordRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .record_id("test-record")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.record_id, "test-record");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_delete_record_request_builder_chaining() {
        let request = DeleteRecordRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .record_id("record123")
            .user_id_type("user_id")
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.record_id, "record123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_delete_record_response_trait() {
        assert_eq!(DeleteRecordResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_delete_record_response() {
        let record = Record {
            record_id: Some("rec123".to_string()),
            fields: std::collections::HashMap::from([
                ("标题".to_string(), serde_json::json!("删除的记录")),
            ]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None,
        };

        let response = DeleteRecordResponse { record };
        assert_eq!(response.record.record_id, Some("rec123".to_string()));
        assert_eq!(response.record.fields["标题"], "删除的记录");
    }

    #[test]
    fn test_delete_record_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = DeleteRecordRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.record_id, "");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_delete_record_request_different_user_id_types() {
        let open_id_request = DeleteRecordRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .record_id("record-id")
            .user_id_type("open_id")
            .build();

        let user_id_request = DeleteRecordRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .record_id("record-id")
            .user_id_type("user_id")
            .build();

        let union_id_request = DeleteRecordRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .record_id("record-id")
            .user_id_type("union_id")
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
        assert_eq!(union_id_request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_delete_record_request_empty_strings() {
        let request = DeleteRecordRequest::builder()
            .app_token("")
            .table_id("")
            .record_id("")
            .build();

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.record_id, "");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_delete_record_request_long_ids() {
        let long_app_token = "bascn".repeat(20); // 长app_token
        let long_table_id = "tbl".repeat(10); // 长table_id
        let long_record_id = "rec".repeat(15); // 长record_id

        let request = DeleteRecordRequest::builder()
            .app_token(&long_app_token)
            .table_id(&long_table_id)
            .record_id(&long_record_id)
            .build();

        assert_eq!(request.app_token, long_app_token);
        assert_eq!(request.table_id, long_table_id);
        assert_eq!(request.record_id, long_record_id);
    }
}
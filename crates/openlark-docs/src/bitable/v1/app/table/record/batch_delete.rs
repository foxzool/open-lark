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
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量删除记录请求
#[derive(Clone)]
pub struct BatchDeleteRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 记录 ID 列表
    record_ids: Vec<String>,
}

impl BatchDeleteRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::POST, "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_delete".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            record_ids: Vec::new(),
        }
    }

    pub fn builder() -> BatchDeleteRecordRequestBuilder {
        BatchDeleteRecordRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct BatchDeleteRecordRequestBuilder {
    request: BatchDeleteRecordRequest,
}

impl BatchDeleteRecordRequestBuilder {
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

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn record_ids(mut self, record_ids: Vec<String>) -> Self {
        self.request.record_ids = record_ids;
        self
    }

    pub fn build(self) -> BatchDeleteRecordRequest {
        self.request
    }
}

/// 批量删除记录响应
#[derive(Clone)]
pub struct BatchDeleteRecordResponse {
    /// 成功删除的记录 ID 列表
    pub records: Vec<DeletedRecord>,
}

/// 被删除的记录信息
#[derive(Clone)]
pub struct DeletedRecord {
    /// 记录 ID
    pub record_id: String,
    /// 是否删除成功
    pub deleted: bool,
}

impl ApiResponseTrait for BatchDeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 请求体结构
#[derive(Serialize)]
struct BatchDeleteRecordRequestBody {
    record_ids: Vec<String>,
}

/// 批量删除记录
pub async fn batch_delete_record(
    request: BatchDeleteRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BatchDeleteRecordResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = BITABLE_V1_RECORDS_BATCH_DELETE
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    // 设置请求体
    let body = BatchDeleteRecordRequestBody {
        record_ids: request.record_ids,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<BatchDeleteRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_delete_record_request_builder() {
        let request = BatchDeleteRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .user_id_type("open_id")
            .record_ids(vec![
                "rec123".to_string(),
                "rec456".to_string(),
                "rec789".to_string(),
            ])
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.record_ids.len(), 3);
        assert!(request.record_ids.contains(&"rec123".to_string()));
        assert!(request.record_ids.contains(&"rec456".to_string()));
        assert!(request.record_ids.contains(&"rec789".to_string()));
    }

    #[test]
    fn test_batch_delete_record_request_body_serialization() {
        let body = BatchDeleteRecordRequestBody {
            record_ids: vec!["rec123".to_string(), "rec456".to_string(), "rec789".to_string()],
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = serde_json::json!({
            "record_ids": ["rec123", "rec456", "rec789"]
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_batch_delete_record_request_minimal() {
        let request = BatchDeleteRecordRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .record_ids(vec!["rec123".to_string()])
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.record_ids, vec!["rec123".to_string()]);
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_batch_delete_record_request_empty_record_ids() {
        let request = BatchDeleteRecordRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .record_ids(vec![])
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert!(request.record_ids.is_empty());
    }

    #[test]
    fn test_deleted_record() {
        let deleted = DeletedRecord {
            record_id: "rec123".to_string(),
            deleted: true,
        };

        assert_eq!(deleted.record_id, "rec123");
        assert!(deleted.deleted);
    }

    #[test]
    fn test_batch_delete_record_response_trait() {
        assert_eq!(BatchDeleteRecordResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_batch_delete_record_request_builder_chaining() {
        let request = BatchDeleteRecordRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .user_id_type("user_id")
            .record_ids(vec!["rec1".to_string(), "rec2".to_string()])
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.record_ids.len(), 2);
        assert!(request.record_ids.contains(&"rec1".to_string()));
        assert!(request.record_ids.contains(&"rec2".to_string()));
    }

    #[test]
    fn test_batch_delete_record_response() {
        let response = BatchDeleteRecordResponse {
            records: vec![
                DeletedRecord {
                    record_id: "rec123".to_string(),
                    deleted: true,
                },
                DeletedRecord {
                    record_id: "rec456".to_string(),
                    deleted: false,
                },
            ],
        };

        assert_eq!(response.records.len(), 2);
        assert_eq!(response.records[0].record_id, "rec123");
        assert!(response.records[0].deleted);
        assert_eq!(response.records[1].record_id, "rec456");
        assert!(!response.records[1].deleted);
    }
}
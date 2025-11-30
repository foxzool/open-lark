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
use serde_json::Value;

/// 批量更新记录请求
#[derive(Clone)]
pub struct BatchUpdateRecordRequest {
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
    /// 要更新的记录列表
    records: Vec<BatchUpdateRecordItem>,
}

/// 批量更新记录项
#[derive(Clone, Serialize)]
pub struct BatchUpdateRecordItem {
    /// 记录ID
    pub record_id: String,
    /// 记录字段
    pub fields: Value,
}

impl BatchUpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::POST, "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            records: Vec::new(),
        }
    }

    pub fn builder() -> BatchUpdateRecordRequestBuilder {
        BatchUpdateRecordRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct BatchUpdateRecordRequestBuilder {
    request: BatchUpdateRecordRequest,
}

impl BatchUpdateRecordRequestBuilder {
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

    pub fn records(mut self, records: Vec<BatchUpdateRecordItem>) -> Self {
        self.request.records = records;
        self
    }

    pub fn add_record(mut self, record_id: impl Into<String>, fields: Value) -> Self {
        self.request.records.push(BatchUpdateRecordItem {
            record_id: record_id.into(),
            fields,
        });
        self
    }

    pub fn build(self) -> BatchUpdateRecordRequest {
        self.request
    }
}

/// 批量更新记录响应
#[derive(Clone)]
pub struct BatchUpdateRecordResponse {
    /// 更新的记录列表
    pub records: Vec<BatchUpdateRecordResult>,
}

/// 批量更新记录结果
#[derive(Clone)]
pub struct BatchUpdateRecordResult {
    /// 记录ID
    pub record_id: String,
    /// 是否更新成功
    pub success: bool,
    /// 更新后的记录
    pub record: Option<Record>,
    /// 错误信息
    pub error: Option<String>,
}

impl ApiResponseTrait for BatchUpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 请求体结构
#[derive(Serialize)]
struct BatchUpdateRecordRequestBody {
    records: Vec<BatchUpdateRecordItem>,
}

/// 批量更新记录
pub async fn batch_update_record(
    request: BatchUpdateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BatchUpdateRecordResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = BITABLE_V1_RECORDS_BATCH_UPDATE
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
    let body = BatchUpdateRecordRequestBody {
        records: request.records,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<BatchUpdateRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_batch_update_record_request_builder() {
        let record1 = BatchUpdateRecordItem {
            record_id: "rec123".to_string(),
            fields: json!({"标题": "更新后的标题1", "状态": "完成"}),
        };

        let record2 = BatchUpdateRecordItem {
            record_id: "rec456".to_string(),
            fields: json!({"标题": "更新后的标题2", "状态": "进行中"}),
        };

        let request = BatchUpdateRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .user_id_type("open_id")
            .records(vec![record1, record2])
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.records.len(), 2);
        assert_eq!(request.records[0].record_id, "rec123");
        assert_eq!(request.records[1].record_id, "rec456");
    }

    #[test]
    fn test_batch_update_record_request_add_record() {
        let request = BatchUpdateRecordRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .add_record("rec123", json!({"标题": "标题1"}))
            .add_record("rec456", json!({"标题": "标题2"}))
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.records.len(), 2);
        assert_eq!(request.records[0].record_id, "rec123");
        assert_eq!(request.records[1].record_id, "rec456");
        assert_eq!(request.records[0].fields["标题"], "标题1");
        assert_eq!(request.records[1].fields["标题"], "标题2");
    }

    #[test]
    fn test_batch_update_record_request_body_serialization() {
        let records = vec![
            BatchUpdateRecordItem {
                record_id: "rec123".to_string(),
                fields: json!({"标题": "测试更新", "数量": 42}),
            },
            BatchUpdateRecordItem {
                record_id: "rec456".to_string(),
                fields: json!({"状态": "完成", "日期": "2023-12-01"}),
            },
        ];

        let body = BatchUpdateRecordRequestBody { records };
        let serialized = serde_json::to_value(&body).unwrap();
        let expected = serde_json::json!({
            "records": [
                {
                    "record_id": "rec123",
                    "fields": {
                        "标题": "测试更新",
                        "数量": 42
                    }
                },
                {
                    "record_id": "rec456",
                    "fields": {
                        "状态": "完成",
                        "日期": "2023-12-01"
                    }
                }
            ]
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_batch_update_record_request_minimal() {
        let request = BatchUpdateRecordRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .records(vec![])
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.records.len(), 0);
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_batch_update_record_request_empty_records() {
        let request = BatchUpdateRecordRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .records(vec![])
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert!(request.records.is_empty());
    }

    #[test]
    fn test_batch_update_record_request_builder_chaining() {
        let request = BatchUpdateRecordRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .user_id_type("user_id")
            .add_record("rec1", json!({"field": "value1"}))
            .add_record("rec2", json!({"field": "value2"}))
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.records.len(), 2);
        assert_eq!(request.records[0].record_id, "rec1");
        assert_eq!(request.records[1].record_id, "rec2");
        assert_eq!(request.records[0].fields["field"], "value1");
        assert_eq!(request.records[1].fields["field"], "value2");
    }

    #[test]
    fn test_batch_update_record_response_trait() {
        assert_eq!(BatchUpdateRecordResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_batch_update_record_response() {
        let response = BatchUpdateRecordResponse {
            records: vec![
                BatchUpdateRecordResult {
                    record_id: "rec123".to_string(),
                    success: true,
                    record: Some(Record {
                        record_id: Some("rec123".to_string()),
                        fields: std::collections::HashMap::from([
                            ("标题".to_string(), json!("更新后的记录")),
                        ]),
                        created_by: None,
                        created_time: None,
                        last_modified_by: None,
                        last_modified_time: None,
                    }),
                    error: None,
                },
                BatchUpdateRecordResult {
                    record_id: "rec456".to_string(),
                    success: false,
                    record: None,
                    error: Some("更新失败".to_string()),
                },
            ],
        };

        assert_eq!(response.records.len(), 2);
        assert_eq!(response.records[0].record_id, "rec123");
        assert!(response.records[0].success);
        assert!(response.records[0].record.is_some());
        assert!(response.records[0].error.is_none());

        assert_eq!(response.records[1].record_id, "rec456");
        assert!(!response.records[1].success);
        assert!(response.records[1].record.is_none());
        assert_eq!(response.records[1].error, Some("更新失败".to_string()));
    }

    #[test]
    fn test_batch_update_record_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = BatchUpdateRecordRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert!(request.user_id_type.is_none());
        assert!(request.records.is_empty());
    }

    #[test]
    fn test_batch_update_record_item() {
        let fields = json!({
            "标题": "测试记录",
            "状态": "进行中",
            "优先级": "高",
            "数字字段": 42,
            "布尔字段": true,
            "日期字段": "2023-12-01",
            "多选字段": ["选项1", "选项2"]
        });

        let item = BatchUpdateRecordItem {
            record_id: "rec123456".to_string(),
            fields: fields.clone(),
        };

        assert_eq!(item.record_id, "rec123456");
        assert_eq!(item.fields, fields);
    }

    #[test]
    fn test_batch_update_record_large_number_of_records() {
        let mut request = BatchUpdateRecordRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .user_id_type("open_id");

        // 添加100个记录
        for i in 0..100 {
            request = request.add_record(
                format!("rec{:03}", i),
                json!({"标题": format!("记录{}", i), "索引": i}),
            );
        }

        let request = request.build();

        assert_eq!(request.app_token, "app-token");
        assert_eq!(request.table_id, "table-id");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.records.len(), 100);
        assert_eq!(request.records[0].record_id, "rec000");
        assert_eq!(request.records[99].record_id, "rec099");
    }

    #[test]
    fn test_batch_update_record_complex_fields() {
        let complex_fields = json!({
            "多选字段": ["选项1", "选项2", "选项3"],
            "数字字段": 42.5,
            "布尔字段": true,
            "日期字段": "2023-12-01",
            "时间字段": "14:30:00",
            "嵌套对象": {
                "子字段1": "值1",
                "子字段2": 123,
                "子字段3": true
            },
            "数组字段": ["元素1", "元素2", 123, true]
        });

        let request = BatchUpdateRecordRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .add_record("rec123", complex_fields.clone())
            .build();

        assert_eq!(request.app_token, "app-token");
        assert_eq!(request.table_id, "table-id");
        assert_eq!(request.records.len(), 1);
        assert_eq!(request.records[0].record_id, "rec123");
        assert_eq!(request.records[0].fields, complex_fields);
    }
}
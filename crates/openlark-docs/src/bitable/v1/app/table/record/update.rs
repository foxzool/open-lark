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

/// 更新记录请求
#[derive(Clone)]
pub struct UpdateRecordRequest {
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
    /// 要更新的记录的数据
    fields: Value,
}

impl UpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::PUT, "/open-apis/bitable/v1/apps/{}/tables/{}/records/{}".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
            fields: Value::Object(Default::default()),
        }
    }

    pub fn builder() -> UpdateRecordRequestBuilder {
        UpdateRecordRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateRecordRequestBuilder {
    request: UpdateRecordRequest,
}

impl UpdateRecordRequestBuilder {
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

    pub fn fields(mut self, fields: Value) -> Self {
        self.request.fields = fields;
        self
    }

    pub fn build(self) -> UpdateRecordRequest {
        self.request
    }
}

/// 更新记录响应
#[derive(Clone)]
pub struct UpdateRecordResponse {
    /// 更新的记录
    pub record: Record,
}

impl ApiResponseTrait for UpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 请求体结构
#[derive(Serialize)]
struct UpdateRecordRequestBody {
    fields: Value,
}

/// 更新记录
pub async fn update_record(
    request: UpdateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<UpdateRecordResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::PUT);
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

    // 设置请求体
    let body = UpdateRecordRequestBody {
        fields: request.fields,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<UpdateRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_update_record_request_builder() {
        let fields = json!({
            "标题": "更新的标题",
            "状态": "已完成",
            "优先级": "高"
        });

        let request = UpdateRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .record_id("rec123456")
            .user_id_type("open_id")
            .fields(fields)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.record_id, "rec123456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.fields["标题"], "更新的标题");
        assert_eq!(request.fields["状态"], "已完成");
        assert_eq!(request.fields["优先级"], "高");
    }

    #[test]
    fn test_update_record_request_body_serialization() {
        let fields = json!({
            "标题": "测试更新",
            "描述": "这是一个测试描述"
        });

        let body = UpdateRecordRequestBody { fields };
        let serialized = serde_json::to_value(&body).unwrap();
        let expected = serde_json::json!({
            "fields": {
                "标题": "测试更新",
                "描述": "这是一个测试描述"
            }
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_update_record_request_minimal() {
        let fields = json!({
            "标题": "最小更新"
        });

        let request = UpdateRecordRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .record_id("test-record")
            .fields(fields)
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.record_id, "test-record");
        assert_eq!(request.fields["标题"], "最小更新");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_update_record_request_empty_fields() {
        let request = UpdateRecordRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .record_id("test-record")
            .fields(Value::Object(Default::default()))
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.record_id, "test-record");
        assert_eq!(request.fields, Value::Object(Default::default()));
    }

    #[test]
    fn test_update_record_request_builder_chaining() {
        let fields = json!({
            "field1": "value1",
            "field2": 123,
            "field3": true
        });

        let request = UpdateRecordRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .record_id("record123")
            .user_id_type("user_id")
            .fields(fields)
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.record_id, "record123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.fields["field1"], "value1");
        assert_eq!(request.fields["field2"], 123);
        assert_eq!(request.fields["field3"], true);
    }

    #[test]
    fn test_update_record_response_trait() {
        assert_eq!(UpdateRecordResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_update_record_response() {
        let record = Record {
            record_id: Some("rec123".to_string()),
            fields: std::collections::HashMap::from([
                ("标题".to_string(), json!("更新的记录")),
                ("状态".to_string(), json!("完成")),
            ]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None,
        };

        let response = UpdateRecordResponse { record };
        assert_eq!(response.record.record_id, Some("rec123".to_string()));
        assert_eq!(response.record.fields["标题"], "更新的记录");
        assert_eq!(response.record.fields["状态"], "完成");
    }

    #[test]
    fn test_update_record_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = UpdateRecordRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.record_id, "");
        assert!(request.user_id_type.is_none());
        assert_eq!(request.fields, Value::Object(Default::default()));
    }

    #[test]
    fn test_update_record_complex_fields() {
        let complex_fields = json!({
            "多选字段": ["选项1", "选项2", "选项3"],
            "数字字段": 42.5,
            "布尔字段": true,
            "日期字段": "2023-12-01",
            "嵌套对象": {
                "子字段1": "值1",
                "子字段2": 123
            }
        });

        let request = UpdateRecordRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .record_id("record-id")
            .fields(complex_fields.clone())
            .build();

        assert_eq!(request.app_token, "app-token");
        assert_eq!(request.table_id, "table-id");
        assert_eq!(request.record_id, "record-id");
        assert_eq!(request.fields, complex_fields);
    }
}
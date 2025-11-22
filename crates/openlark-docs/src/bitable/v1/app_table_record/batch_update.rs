#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use openlark_core::{
use serde_json::Value;
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
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    #[serde(skip)]
    client_token: Option<String>,
    /// 更新的记录列表
    records: Vec<Record>,
}

impl BatchUpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[derive(Clone)]
pub struct BatchUpdateRecordRequestBuilder {
    request: BatchUpdateRecordRequest,
}

impl BatchUpdateRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchUpdateRecordRequest {
                api_request: ApiRequest::post("/open-apis/bitable/v1/placeholder"),
                app_token: String::new(),
                table_id: String::new(),
                user_id_type: None,
                client_token: None,
                records: Vec::new(),
            },
        }
    }

    /// 设置多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 设置数据表的 table_id
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置客户端令牌，用于幂等操作
    pub fn client_token(mut self, client_token: impl ToString) -> Self {
        self.request.client_token = Some(client_token.to_string());
        self
    }

    /// 设置要更新的记录列表
    pub fn records(mut self, records: Vec<Record>) -> Self {
        self.request.records = records;
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchUpdateRecordRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait到BatchUpdateRecordRequestBuilder
crate::impl_executable_builder_owned!(
    BatchUpdateRecordRequestBuilder,
    super::AppTableRecordService,
    BatchUpdateRecordRequest,
    Response<BatchUpdateRecordResponse>,
    batch_update,
);

/// 批量更新记录响应
#[derive(Clone)]
pub struct BatchUpdateRecordResponse {
    /// 更新的记录列表
    pub records: Vec<BatchUpdateRecordResult>,
}

impl ApiResponseTrait for BatchUpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新记录的结果
#[derive(Clone, Serialize, Deserialize)]
pub struct BatchUpdateRecordResult {
    /// 记录 ID
    pub record_id: String,
    /// 记录的字段
    pub fields: std::collections::HashMap<String, serde_json::Value>,
    /// 是否更新成功
    pub success: bool,
    /// 错误信息（如果更新失败）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 请求体结构
#[derive(Serialize)]
struct BatchUpdateRecordRequestBody {
    records: Vec<Record>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_token: Option<String>,
}

/// 批量更新记录
pub async fn batch_update_record(
    request: BatchUpdateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<BatchUpdateRecordResponse>> {
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

    if let Some(client_token) = &request.client_token {
        api_req
            .query_params
            .insert("client_token".to_string(), client_token.clone());
    }

    // 设置请求体
    let body = BatchUpdateRecordRequestBody {
        records: request.records,
        user_id_type: request.user_id_type,
        client_token: request.client_token,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_batch_update_record_request_builder() {
        let record1 = Record {
            record_id: Some("rec123".to_string()),
            fields: HashMap::from([
                ("标题".to_string(), json!("更新后的标题")),
                ("状态".to_string(), json!("已完成")),
            ]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None,
        };

        let record2 = Record {
            record_id: Some("rec456".to_string()),
            fields: HashMap::from([
                ("标题".to_string(), json!("另一个更新")),
                ("优先级".to_string(), json!(1)),
            ]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None,
        };

        let request = BatchUpdateRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .user_id_type("open_id")
            .client_token("550e8400-e29b-41d4-a716-446655440000")
            .records(vec![record1, record2])
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.client_token, Some("550e8400-e29b-41d4-a716-446655440000".to_string()));
        assert_eq!(request.records.len(), 2);
    }

    #[test]
    fn test_batch_update_record_request_body_serialization() {
        let record = Record {
            record_id: Some("rec123".to_string()),
            fields: HashMap::from([
                ("标题".to_string(), json!("测试更新")),
            ]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None,
        };

        let body = BatchUpdateRecordRequestBody {
            records: vec![record],
            user_id_type: Some("open_id".to_string()),
            client_token: Some("test-token".to_string()),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "records": [{
                "record_id": "rec123",
                "fields": {
                    "标题": "测试更新"
                }
            }],
            "user_id_type": "open_id",
            "client_token": "test-token"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_batch_update_record_result() {
        let fields = HashMap::from([
            ("标题".to_string(), json!("更新后的标题")),
            ("状态".to_string(), json!("已完成")),
        ]);

        let success_result = BatchUpdateRecordResult {
            record_id: "rec123".to_string(),
            fields: fields.clone(),
            success: true,
            error: None,
        };

        assert_eq!(success_result.record_id, "rec123");
        assert!(success_result.success);
        assert!(success_result.error.is_none());
        assert_eq!(success_result.fields.get("标题"), Some(&json!("更新后的标题")));

        let failed_result = BatchUpdateRecordResult {
            record_id: "rec456".to_string(),
            fields: HashMap::new(),
            success: false,
            error: Some("记录不存在或权限不足".to_string()),
        };

        assert_eq!(failed_result.record_id, "rec456");
        assert!(!failed_result.success);
        assert_eq!(failed_result.error, Some("记录不存在或权限不足".to_string()));
    }

    #[test]
    fn test_batch_update_record_request_minimal() {
        let record = Record {
            record_id: Some("rec123".to_string()),
            fields: HashMap::from([("标题".to_string(), json!("最小更新"))]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None,
        };

        let request = BatchUpdateRecordRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .records(vec![record])
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.records.len(), 1);
        assert!(request.user_id_type.is_none());
        assert!(request.client_token.is_none());
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
}
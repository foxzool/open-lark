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

// 导入通用批量操作模块
use super::super::{BatchCommonParams, BatchCommonBody, BatchOperationResult};

/// 批量删除数据表请求
#[derive(Clone)]
pub struct BatchDeleteTableRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的 app_token
    #[serde(skip)]
    app_token: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    #[serde(skip)]
    client_token: Option<String>,
    /// 要删除的数据表 ID 列表
    table_ids: Vec<String>,
}

impl BatchDeleteTableRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[derive(Clone)]
pub struct BatchDeleteTableRequestBuilder {
    request: BatchDeleteTableRequest,
}

impl BatchDeleteTableRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchDeleteTableRequest {
                api_request: ApiRequest::post("/open-apis/bitable/v1/placeholder"),
                app_token: String::new(),
                user_id_type: None,
                client_token: None,
                table_ids: Vec::new(),
            },
        }
    }

    /// 设置多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
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

    /// 设置要删除的数据表 ID 列表
    pub fn table_ids(mut self, table_ids: Vec<String>) -> Self {
        self.request.table_ids = table_ids;
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchDeleteTableRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait到BatchDeleteTableRequestBuilder
crate::impl_executable_builder_owned!(
    BatchDeleteTableRequestBuilder,
    super::AppTableService,
    BatchDeleteTableRequest,
    Response<BatchDeleteTableResponse>,
    batch_delete,
);

/// 批量删除数据表响应
#[derive(Clone)]
pub struct BatchDeleteTableResponse {
    /// 删除的数据表结果列表
    pub results: Vec<BatchDeleteTableResult>,
}

impl ApiResponseTrait for BatchDeleteTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除数据表的结果
#[derive(Clone, Serialize, Deserialize)]
pub struct BatchDeleteTableResult {
    /// 数据表 ID
    pub table_id: String,
    /// 是否删除成功
    pub success: bool,
    /// 错误信息（如果删除失败）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 请求体结构
#[derive(Serialize)]
struct BatchDeleteTableRequestBody {
    table_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_token: Option<String>,
}

/// 批量删除数据表
pub async fn batch_delete_table(
    request: BatchDeleteTableRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<BatchDeleteTableResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = BITABLE_V1_TABLES_BATCH_DELETE
        .replace("{app_token}", &request.app_token);
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
    let body = BatchDeleteTableRequestBody {
        table_ids: request.table_ids,
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

    #[test]
    fn test_batch_delete_table_request_builder() {
        let table_ids = vec![
            "tbl123".to_string(),
            "tbl456".to_string(),
            "tbl789".to_string(),
        ];

        let request = BatchDeleteTableRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .user_id_type("open_id")
            .client_token("550e8400-e29b-41d4-a716-446655440000")
            .table_ids(table_ids.clone())
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.client_token, Some("550e8400-e29b-41d4-a716-446655440000".to_string()));
        assert_eq!(request.table_ids, table_ids);
    }

    #[test]
    fn test_batch_delete_table_request_body_serialization() {
        let table_ids = vec!["tbl123".to_string(), "tbl456".to_string()];

        let body = BatchDeleteTableRequestBody {
            table_ids: table_ids.clone(),
            user_id_type: Some("open_id".to_string()),
            client_token: Some("test-token".to_string()),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "table_ids": ["tbl123", "tbl456"],
            "user_id_type": "open_id",
            "client_token": "test-token"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_batch_delete_table_result() {
        let success_result = BatchDeleteTableResult {
            table_id: "tbl123".to_string(),
            success: true,
            error: None,
        };

        assert_eq!(success_result.table_id, "tbl123");
        assert!(success_result.success);
        assert!(success_result.error.is_none());

        let failed_result = BatchDeleteTableResult {
            table_id: "tbl456".to_string(),
            success: false,
            error: Some("表不存在或权限不足".to_string()),
        };

        assert_eq!(failed_result.table_id, "tbl456");
        assert!(!failed_result.success);
        assert_eq!(failed_result.error, Some("表不存在或权限不足".to_string()));
    }

    #[test]
    fn test_batch_delete_table_request_minimal() {
        let table_ids = vec!["tbl123".to_string()];

        let request = BatchDeleteTableRequest::builder()
            .app_token("test-token")
            .table_ids(table_ids.clone())
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_ids, table_ids);
        assert!(request.user_id_type.is_none());
        assert!(request.client_token.is_none());
    }

    #[test]
    fn test_batch_delete_table_request_empty_table_ids() {
        let request = BatchDeleteTableRequest::builder()
            .app_token("test-token")
            .table_ids(vec![])
            .build();

        assert_eq!(request.app_token, "test-token");
        assert!(request.table_ids.is_empty());
    }

    #[test]
    fn test_batch_delete_table_request_builder_chaining() {
        let request = BatchDeleteTableRequest::builder()
            .app_token("app123")
            .user_id_type("user_id")
            .client_token("client123")
            .table_ids(vec!["tbl1".to_string(), "tbl2".to_string()])
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.client_token, Some("client123".to_string()));
        assert_eq!(request.table_ids.len(), 2);
        assert!(request.table_ids.contains(&"tbl1".to_string()));
        assert!(request.table_ids.contains(&"tbl2".to_string()));
    }
}
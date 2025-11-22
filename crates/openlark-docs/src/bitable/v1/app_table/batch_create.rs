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
    service::bitable::v1::{TableData, TableField},
    SDKResult,
};
use serde::{Deserialize, Serialize};

// 导入通用批量操作模块
use super::super::{BatchCommonParams, BatchCommonBody, BatchOperationResult, AppToken, TableId};

/// 批量新增数据表请求
#[derive(Clone)]
pub struct BatchCreateTableRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的 app_token
    #[serde(skip)]
    app_token: AppToken,
    /// 通用批量操作参数
    #[serde(skip)]
    common_params: BatchCommonParams,
    /// 要新增的数据表列表
    tables: Vec<TableData>,
}

impl BatchCreateTableRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/placeholder"),
            app_token: AppToken::new(""),
            common_params: BatchCommonParams::new(),
            tables: Vec::new(),
        }
    }

    /// 获取应用令牌
    pub fn app_token(&self) -> &AppToken {
        &self.app_token
    }

    /// 获取用户ID类型
    pub fn user_id_type(&self) -> Option<&str> {
        self.common_params.user_id_type.as_deref()
    }

    /// 获取客户端令牌
    pub fn client_token(&self) -> Option<&str> {
        self.common_params.client_token.as_deref()
    }
}

#[derive(Clone)]
pub struct BatchCreateTableRequestBuilder {
    request: BatchCreateTableRequest,
}

impl BatchCreateTableRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchCreateTableRequest {
                api_request: ApiRequest::post("/open-apis/bitable/v1/placeholder"),
                app_token: String::new(),
                common_params: BatchCommonParams::new(),
                tables: Vec::new(),
            },
        }
    }

    /// 设置多维表格的 app_token
    pub fn app_token(mut self, app_token: impl Into<AppToken>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.common_params = self.request.common_params.with_user_id_type(user_id_type);
        self
    }

    /// 设置客户端令牌，用于幂等操作
    pub fn client_token(mut self, client_token: impl ToString) -> Self {
        self.request.common_params = self.request.common_params.with_client_token(client_token);
        self
    }

    /// 设置要新增的数据表列表
    pub fn tables(mut self, tables: Vec<TableData>) -> Self {
        self.request.tables = tables;
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchCreateTableRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait到BatchCreateTableRequestBuilder
crate::impl_executable_builder_owned!(
    BatchCreateTableRequestBuilder,
    super::AppTableService,
    BatchCreateTableRequest,
    Response<BatchCreateTableResponse>,
    batch_create,
);

/// 批量新增数据表响应
#[derive(Clone)]
pub struct BatchCreateTableResponse {
    /// 新增的数据表列表
    pub tables: Vec<BatchCreateTableResult>,
}

impl ApiResponseTrait for BatchCreateTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量创建数据表的结果
#[derive(Clone, Serialize, Deserialize)]
pub struct BatchCreateTableResult {
    /// 数据表 ID
    #[serde(skip)]
    pub table_id: TableId,
    /// 数据表名称
    pub name: String,
    /// 数据表的默认视图 ID
    pub default_view_id: String,
    /// 批量操作结果
    #[serde(flatten)]
    pub operation: BatchOperationResult,
}

impl BatchCreateTableResult {
    /// 创建成功的结果
    pub fn success(table_id: impl Into<TableId>, name: String, default_view_id: String) -> Self {
        Self {
            table_id: table_id.into(),
            name,
            default_view_id,
            operation: BatchOperationResult::success(),
        }
    }

    /// 创建失败的结果
    pub fn failure(table_id: impl Into<TableId>, name: String, error: impl ToString) -> Self {
        Self {
            table_id: table_id.into(),
            name,
            default_view_id: String::new(),
            operation: BatchOperationResult::failure(error),
        }
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.operation.success
    }

    /// 获取错误信息
    pub fn get_error(&self) -> Option<&str> {
        self.operation.error.as_deref()
    }

    /// 获取表格ID
    pub fn get_table_id(&self) -> &TableId {
        &self.table_id
    }
}

/// 请求体结构
type BatchCreateTableRequestBody = BatchCommonBody<Vec<TableData>>;

/// 批量新增数据表
pub async fn batch_create_table(
    request: BatchCreateTableRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<BatchCreateTableResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = BITABLE_V1_TABLES_BATCH_CREATE
        .replace("{app_token}", request.app_token.as_str());
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.common_params.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    if let Some(client_token) = &request.common_params.client_token {
        api_req
            .query_params
            .insert("client_token".to_string(), client_token.clone());
    }

    // 设置请求体
    let body = BatchCreateTableRequestBody::new(request.tables)
        .with_common_params(request.common_params);

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_batch_create_table_request_builder() {
        let table1 = TableData::new("任务表")
            .with_default_view_name("任务视图")
            .with_fields(vec![
                TableField::text("任务名称"),
                TableField::number("优先级"),
                TableField::single_select("状态", vec!["待开始".to_string(), "进行中".to_string(), "已完成".to_string()]),
                TableField::date("截止日期"),
            ]);

        let table2 = TableData::new("人员表")
            .with_fields(vec![
                TableField::text("姓名"),
                TableField::single_select("部门", vec!["技术部".to_string(), "产品部".to_string(), "运营部".to_string()]),
            ]);

        let request = BatchCreateTableRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .user_id_type("open_id")
            .client_token("550e8400-e29b-41d4-a716-446655440000")
            .tables(vec![table1, table2])
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.user_id_type(), Some("open_id"));
        assert_eq!(request.client_token(), Some("550e8400-e29b-41d4-a716-446655440000"));
        assert_eq!(request.tables.len(), 2);
        assert_eq!(request.tables[0].name, "任务表");
        assert_eq!(request.tables[1].name, "人员表");
    }

    #[test]
    fn test_batch_create_table_request_body_serialization() {
        let table = TableData::new("测试表")
            .with_fields(vec![TableField::text("字段")]);

        let body = BatchCreateTableRequestBody::new(vec![table])
            .with_common_params(
                BatchCommonParams::new()
                    .with_user_id_type("open_id")
                    .with_client_token("test-token")
            );

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "data": [{
                "name": "测试表",
                "fields": [{
                    "field_name": "字段",
                    "type": 1
                }]
            }],
            "user_id_type": "open_id",
            "client_token": "test-token"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_batch_create_table_result() {
        let result = BatchCreateTableResult::success(
            "tbl123".to_string(),
            "测试表".to_string(),
            "view123".to_string()
        );

        assert_eq!(result.table_id, "tbl123");
        assert_eq!(result.name, "测试表");
        assert_eq!(result.default_view_id, "view123");
        assert!(result.is_success());
        assert!(result.get_error().is_none());

        let failed_result = BatchCreateTableResult::failure(
            "".to_string(),
            "失败表".to_string(),
            "创建失败"
        );

        assert!(!failed_result.is_success());
        assert_eq!(failed_result.get_error(), Some("创建失败"));
    }

    #[test]
    fn test_batch_create_table_request_minimal() {
        let request = BatchCreateTableRequest::builder()
            .app_token("test-token")
            .tables(vec![TableData::new("最小表")])
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.tables.len(), 1);
        assert_eq!(request.tables[0].name, "最小表");
        assert!(request.user_id_type().is_none());
        assert!(request.client_token().is_none());
    }
}
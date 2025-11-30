
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

// 导入通用批量操作模块
use super::super::{BatchCommonParams, BatchCommonBody, BatchOperationResult, AppToken, TableId};

/// 批量新增数据表请求
#[derive(Clone)]
pub struct BatchCreateTableRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
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
            api_request: ApiRequest::new()
                .method(HttpMethod::POST)
                .api_path("/open-apis/bitable/v1/apps/{}/tables:batch_create".to_string())
                .config(config)
                .build(),
            app_token: AppToken::new(),
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
                api_request: ApiRequest::new()
                    .method(HttpMethod::POST)
                    .api_path("/open-apis/bitable/v1/placeholder".to_string())
                    .config(config.clone())
                    .build(),
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
    batch_create
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
    let api_path = format!("/open-apis/bitable/v1/apps/{}/tables:batch_create", request.app_token.as_str());
    api_req = api_req.api_path(api_path);

    // 设置查询参数
    if let Some(user_id_type) = &request.common_params.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
    }

    if let Some(client_token) = &request.common_params.client_token {
        api_req
            .query_params
            .insert(client_token.to_string(), client_token.clone());
    }

    // 设置请求体
    let body = BatchCreateTableRequestBody::new(request.tables)
        .with_common_params(request.common_params);

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}



use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    
    config::Config,
    
    
    http::Transport,
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
    api_request: ApiRequest<Self>,
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
                api_request: ApiRequest::new()
                    .method(HttpMethod::POST)
                    .api_path("/open-apis/bitable/v1/placeholder".to_string())
                    .config(config.clone())
                    .build(),
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
    batch_delete
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
    #[serde(skip_serializing_if = Option::is_none)]
    pub error: Option<String>,
}

/// 请求体结构
#[derive(Serialize)]
struct BatchDeleteTableRequestBody {
    table_ids: Vec<String>,
    #[serde(skip_serializing_if = Option::is_none)]
    user_id_type: Option<String>,
    #[serde(skip_serializing_if = Option::is_none)]
    client_token: Option<String>,
}

/// 批量删除数据表
pub async fn batch_delete_table(
    request: BatchDeleteTableRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<BatchDeleteTableResponse>> {
    let mut api_req = request.api_request;
    let api_path = format!("/open-apis/bitable/v1/apps/{}/tables:batch_delete", &request.app_token);
    api_req = api_req.api_path(api_path);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
    }

    if let Some(client_token) = &request.client_token {
        api_req
            .query_params
            .insert(client_token.to_string(), client_token.clone());
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


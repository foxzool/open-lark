
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    api::{ApiResponseTrait},
    config::Config,
    
    
    http::Transport,
    req_option::RequestOption,
    
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量获取记录请求
#[derive(Clone)]
pub struct BatchGetRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
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
    /// 控制是否返回自动计算的字段
    #[serde(skip_serializing_if = Option::is_none)]
    automatic: Option<bool>,
    /// 控制是否返回记录权限
    #[serde(skip_serializing_if = Option::is_none)]
    with_shared_url: Option<bool>,
}

impl BatchGetRecordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[derive(Clone)]
pub struct BatchGetRecordRequestBuilder {
    request: BatchGetRecordRequest,
}

impl BatchGetRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchGetRecordRequest {
                api_request: ApiRequest::post(/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_get.to_string()),
                app_token: String::new(),
                table_id: String::new(),
                user_id_type: None,
                record_ids: Vec::new(),
                automatic: None,
                with_shared_url: None,
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

    /// 设置记录 ID 列表
    pub fn record_ids(mut self, record_ids: Vec<String>) -> Self {
        self.request.record_ids = record_ids;
        self
    }

    /// 设置是否返回自动计算的字段
    pub fn automatic(mut self, automatic: bool) -> Self {
        self.request.automatic = Some(automatic);
        self
    }

    /// 设置是否返回记录权限
    pub fn with_shared_url(mut self, with_shared_url: bool) -> Self {
        self.request.with_shared_url = Some(with_shared_url);
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchGetRecordRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait到BatchGetRecordRequestBuilder
crate::impl_executable_builder_owned!(
    BatchGetRecordRequestBuilder,
    super::AppTableRecordService,
    BatchGetRecordRequest,
    Response<BatchGetRecordResponse>,
    batch_get,
);

/// 批量获取记录响应
#[derive(Clone)]
pub struct BatchGetRecordResponse {
    /// 记录列表
    pub records: Vec<Record>,
}

impl ApiResponseTrait for BatchGetRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 请求体结构
#[derive(Serialize)]
struct BatchGetRecordRequestBody {
    record_ids: Vec<String>,
    #[serde(skip_serializing_if = Option::is_none)]
    automatic: Option<bool>,
    #[serde(skip_serializing_if = Option::is_none)]
    with_shared_url: Option<bool>,
}

/// 批量获取记录
pub async fn batch_get_record(
    request: BatchGetRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<BatchGetRecordResponse>> {
    let mut api_req = request.api_request;
    let api_path = format!(
        "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_get",
        &request.app_token, &request.table_id
    );
    api_req = api_req.api_path(api_path);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
    }

    // 设置请求体
    let body = BatchGetRecordRequestBody {
        record_ids: request.record_ids,
        automatic: request.automatic,
        with_shared_url: request.with_shared_url,
    };

    api_req = api_req.body(serde_json::to_vec(&body)?);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}


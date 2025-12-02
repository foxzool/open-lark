
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,

    http::Transport,
    req_option::RequestOption,

    SDKResult,
};
use serde::{Deserialize, Serialize};
use super::batch_create::Record;

/// 批量获取记录请求
#[derive(Debug, Clone)]
pub struct BatchGetRecordRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 多维表格数据表的唯一标识符
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 记录 ID 列表
    record_ids: Vec<String>,
    /// 控制是否返回自动计算的字段
    automatic: Option<bool>,
    /// 控制是否返回记录权限
    with_shared_url: Option<bool>,
}

impl Default for BatchGetRecordRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::post("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_get"),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            record_ids: Vec::new(),
            automatic: None,
            with_shared_url: None,
        }
    }
}

impl BatchGetRecordRequest {
    pub fn new(config: Config) -> Self {
        Self::default()
    }
}

pub struct BatchGetRecordRequestBuilder {
    request: BatchGetRecordRequest,
}

impl BatchGetRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchGetRecordRequest::new(config),
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
    batch_get
);

/// 批量获取记录响应
#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Serialize)]
struct BatchGetRecordRequestBody {
    record_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    with_shared_url: Option<bool>,
}

/// 批量获取记录
pub async fn batch_get_record(
    request: BatchGetRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<BatchGetRecordResponse>> {
    let url = format!(
        "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_get",
        &request.app_token, &request.table_id
    );
    let mut api_req = ApiRequest::<()>::post(&url);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req = api_req.query("user_id_type", user_id_type);
    }

    // 设置请求体
    let body = BatchGetRecordRequestBody {
        record_ids: request.record_ids,
        automatic: request.automatic,
        with_shared_url: request.with_shared_url,
    };

    api_req = api_req.body(openlark_core::api::RequestData::Json(serde_json::to_value(&body).unwrap()));

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}


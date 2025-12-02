
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, HttpMethod, RequestData, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量删除记录请求
#[derive(Debug, Clone)]
pub struct BatchDeleteRecordRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 多维表格数据表的唯一标识符
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 记录 ID 列表
    record_ids: Vec<String>,
}

impl Default for BatchDeleteRecordRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::delete("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_delete"),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            record_ids: Vec::new(),
        }
    }
}

impl BatchDeleteRecordRequest {
    pub fn new(config: Config) -> Self {
        Self::default()
    }

    pub fn builder() -> BatchDeleteRecordRequestBuilder {
        BatchDeleteRecordRequestBuilder::default()
    }
}

pub struct BatchDeleteRecordRequestBuilder {
    request: BatchDeleteRecordRequest,
}

impl Default for BatchDeleteRecordRequestBuilder {
    fn default() -> Self {
        Self {
            request: BatchDeleteRecordRequest::default(),
        }
    }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteRecordResponse {
    /// 成功删除的记录 ID 列表
    pub records: Vec<DeletedRecord>,
}

/// 被删除的记录信息
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Serialize)]
struct BatchDeleteRecordRequestBody {
    record_ids: Vec<String>,
}

/// 批量删除记录
pub async fn batch_delete_record(
    request: BatchDeleteRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BatchDeleteRecordResponse> {
    let url = format!(
        "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_delete",
        &request.app_token, &request.table_id
    );
    let mut api_req = ApiRequest::<()>::delete(&url);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req = api_req.query("user_id_type", user_id_type);
    }

    // 设置请求体
    let body = BatchDeleteRecordRequestBody {
        record_ids: request.record_ids,
    };

    api_req = api_req.body(RequestData::Json(serde_json::to_value(&body).unwrap()));

    let api_resp: Response<BatchDeleteRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}


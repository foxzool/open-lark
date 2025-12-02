
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat, HttpMethod, RequestData},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::batch_create::Record;

/// 批量更新记录请求
#[derive(Debug, Clone)]
pub struct BatchUpdateRecordRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 多维表格数据表的唯一标识符
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 要更新的记录列表
    records: Vec<BatchUpdateRecordItem>,
}

impl Default for BatchUpdateRecordRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::put("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update"),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            records: Vec::new(),
        }
    }
}


/// 批量更新记录项
#[derive(Debug, Clone, Serialize)]
pub struct BatchUpdateRecordItem {
    /// 记录ID
    pub record_id: String,
    /// 记录字段
    pub fields: Value,
}

impl BatchUpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update")
                
                ,
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
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchUpdateRecordRequest::new(config),
        }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateRecordResponse {
    /// 更新的记录列表
    pub records: Vec<BatchUpdateRecordResult>,
}

/// 批量更新记录结果
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Serialize)]
struct BatchUpdateRecordRequestBody {
    records: Vec<BatchUpdateRecordItem>,
}

/// 批量更新记录
pub async fn batch_update_record(
    request: BatchUpdateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BatchUpdateRecordResponse> {
    let url = format!(
        "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update",
        &request.app_token, &request.table_id
    );
    let mut api_req = ApiRequest::<()>::post(&url);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req = api_req.query("user_id_type", user_id_type);
    }

    // 设置请求体
    let body = BatchUpdateRecordRequestBody {
        records: request.records,
    };

    api_req = api_req.body(RequestData::Json(serde_json::to_value(&body).unwrap()));

    let api_resp: Response<BatchUpdateRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}


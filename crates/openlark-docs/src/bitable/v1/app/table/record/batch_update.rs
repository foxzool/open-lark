
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    
    config::Config,
    
    
    http::Transport,
    req_option::RequestOption,
    
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量更新记录请求
#[derive(Clone)]
pub struct BatchUpdateRecordRequest {
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
    /// 要更新的记录列表
    records: Vec<BatchUpdateRecordItem>,
}

/// 批量更新记录项
#[derive(Clone, Serialize)]
pub struct BatchUpdateRecordItem {
    /// 记录ID
    pub record_id: String,
    /// 记录字段
    pub fields: Value,
}

impl BatchUpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new().method(HttpMethod::POST).api_path( /open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update).config(config)),
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
#[derive(Clone)]
pub struct BatchUpdateRecordResponse {
    /// 更新的记录列表
    pub records: Vec<BatchUpdateRecordResult>,
}

/// 批量更新记录结果
#[derive(Clone)]
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
#[derive(Serialize)]
struct BatchUpdateRecordRequestBody {
    records: Vec<BatchUpdateRecordItem>,
}

/// 批量更新记录
pub async fn batch_update_record(
    request: BatchUpdateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BatchUpdateRecordResponse> {
    let mut api_req = request.api_request;
        let api_request = api_request.api_path(format!(        .replace({app_token}, &request.app_token)
        let api_request = api_request.api_path(format!(        .replace({table_id}, &request.table_id);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
    }

    // 设置请求体
    let body = BatchUpdateRecordRequestBody {
        records: request.records,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<BatchUpdateRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}



use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    
    config::Config,
    
    
    http::Transport,
    req_option::RequestOption,
    
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新记录请求
#[derive(Clone)]
pub struct UpdateRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 记录的唯一标识符
    #[serde(skip)]
    record_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 要更新的记录的数据
    fields: Value,
}

impl UpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new().method(HttpMethod::POST).api_path( /open-apis/bitable/v1/apps/{}/tables/{}/records/{}).config(config)),
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
            fields: Value::Object(Default::default()),
        }
    }

    pub fn builder() -> UpdateRecordRequestBuilder {
        UpdateRecordRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateRecordRequestBuilder {
    request: UpdateRecordRequest,
}

impl UpdateRecordRequestBuilder {
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

    pub fn record_id(mut self, record_id: impl Into<String>) -> Self {
        self.request.record_id = record_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn fields(mut self, fields: Value) -> Self {
        self.request.fields = fields;
        self
    }

    pub fn build(self) -> UpdateRecordRequest {
        self.request
    }
}

/// 更新记录响应
#[derive(Clone)]
pub struct UpdateRecordResponse {
    /// 更新的记录
    pub record: Record,
}

impl ApiResponseTrait for UpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 请求体结构
#[derive(Serialize)]
struct UpdateRecordRequestBody {
    fields: Value,
}

/// 更新记录
pub async fn update_record(
    request: UpdateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<UpdateRecordResponse> {
    let mut api_req = request.api_request;
        let api_request = api_request.api_path(format!(        .replace({app_token}, &request.app_token)
        let api_request = api_request.api_path(format!(        .replace({table_id}, &request.table_id)
        let api_request = api_request.api_path(format!(        .replace({record_id}, &request.record_id);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
    }

    // 设置请求体
    let body = UpdateRecordRequestBody {
        fields: request.fields,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<UpdateRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}


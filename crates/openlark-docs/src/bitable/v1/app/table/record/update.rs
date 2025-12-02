
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, responses::Response},

    config::Config,

    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::batch_create::Record;

/// 更新记录请求
#[derive(Debug, Clone)]pub struct UpdateRecordRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 多维表格数据表的唯一标识符
    table_id: String,
    /// 记录的唯一标识符
    record_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 要更新的记录的数据
    fields: Value,
}

impl Default for UpdateRecordRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::put("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/{}"),
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
            fields: serde_json::Value::Object(Default::default()),
        }
    }
}

impl UpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::put("").header("Content-Type", "application/json"),
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    // 设置查询参数
    let mut api_req = if let Some(user_id_type) = &request.user_id_type {
        api_req.query("user_id_type", user_id_type)
    } else {
        api_req
    };

    // 设置请求体
    let body = UpdateRecordRequestBody {
        fields: request.fields,
    };

    api_req = api_req.body(openlark_core::api::RequestData::Json(serde_json::to_value(&body)?));

    let api_resp: Response<UpdateRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}


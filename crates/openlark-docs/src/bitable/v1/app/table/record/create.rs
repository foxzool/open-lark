
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, HttpMethod, RequestData},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::batch_create::Record;
/// 新增记录请求
#[derive(Debug, Clone)]
pub struct CreateRecordRequest {
    api_request: ApiRequest<CreateRecordResponse>,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 多维表格数据表的唯一标识符
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    client_token: Option<String>,
    /// 要新增的记录的数据
    fields: Record,
}
impl Default for CreateRecordRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::post("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records"),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            client_token: None,
            fields: Record::default(),
        }
    }
}

impl CreateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self::default()
    }

    pub fn builder() -> CreateRecordRequestBuilder {
        CreateRecordRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateRecordRequestBuilder {
    request: CreateRecordRequest,
}

impl CreateRecordRequestBuilder {
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

    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.request.client_token = Some(client_token.into());
        self
    }

    pub fn fields(mut self, fields: Record) -> Self {
        self.request.fields = fields;
        self
    }

    pub fn build(self) -> CreateRecordRequest {
        self.request
    }
}
/// 新增记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecordResponse {
    /// 新增的记录
    pub record: Record,
}

impl ApiResponseTrait for CreateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新增记录
///
/// # API文档
/// https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/bitable/v1/apps/:app_token/tables/:table_id/records/create
pub async fn create_record(
    request: CreateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<CreateRecordResponse> {
    let url = format!(
        "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records",
        &request.app_token, &request.table_id
    );
    let mut api_req = ApiRequest::<()>::post(&url);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req = api_req.query("user_id_type", user_id_type);
    }

    if let Some(client_token) = &request.client_token {
        api_req = api_req.query("client_token", client_token);
    }

    // 设置请求体
    api_req = api_req.body(RequestData::Json(serde_json::to_value(&request.fields).unwrap()));

    let api_resp = Transport::request(api_req, config, option).await?;
    api_resp.data.ok_or_else(|| openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据"))
}


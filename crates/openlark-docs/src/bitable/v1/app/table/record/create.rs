
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// 新增记录请求
#[derive(Clone)]
pub struct CreateRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest<CreateRecordResponse>,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符,
#[serde(skip)]
    table_id: String,
    /// 用户 ID 类型,
#[serde(skip)]
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作,
#[serde(skip)]
    client_token: Option<String>,
    /// 要新增的记录的数据,
#[serde(flatten)]
    fields: Record}
impl CreateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new().method(HttpMethod::POST).api_path( /open-apis/bitable/v1/apps/{}/tables/{}/records).config(config)),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            client_token: None,
            fields: Record {
                record_id: None,
                fields: std::collections::HashMap::new(),
                created_by: None,
                created_time: None,
                last_modified_by: None,
                last_modified_time: None,
            },
        }
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
#[derive(Clone)]
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
    let mut api_req = request.api_request;
        let api_request = api_request.api_path(format!(        .replace({app_token}, &request.app_token)
        let api_request = api_request.api_path(format!(        .replace({table_id}, &request.table_id);

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
    match serde_json::to_vec(&request.fields) {
        Ok(bytes) => {
            api_req.body = bytes;
        }
        Err(e) => {
            error!(Failed to serialize create record request: {}, e);
            api_req.body = Vec::new();
        }
    }

    let api_resp: openlark_core::core::StandardResponse<CreateRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}


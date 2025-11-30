
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    
    config::Config,
    
    
    http::Transport,
    req_option::RequestOption,
    
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新字段请求
#[derive(Clone)]
pub struct UpdateFieldRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 字段的唯一标识符
    #[serde(skip)]
    field_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 多维表格字段名
    #[serde(skip)]
    field_name: Option<String>,
    /// 多维表格字段类型
    #[serde(skip)]
    field_type: Option<String>,
    /// 字段属性
    #[serde(skip)]
    property: Option<Value>,
    /// 字段的描述
    #[serde(skip)]
    description: Option<String>,
    /// 字段在界面上的展示类型
    #[serde(skip)]
    ui_type: Option<String>,
}

impl UpdateFieldRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new().method(HttpMethod::POST).api_path( /open-apis/bitable/v1/apps/{}/tables/{}/fields/{}).config(config)),
            app_token: String::new(),
            table_id: String::new(),
            field_id: String::new(),
            user_id_type: None,
            field_name: None,
            field_type: None,
            property: None,
            description: None,
            ui_type: None,
        }
    }

    pub fn builder() -> UpdateFieldRequestBuilder {
        UpdateFieldRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateFieldRequestBuilder {
    request: UpdateFieldRequest,
}

impl UpdateFieldRequestBuilder {
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

    pub fn field_id(mut self, field_id: impl Into<String>) -> Self {
        self.request.field_id = field_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn field_name(mut self, field_name: impl Into<String>) -> Self {
        self.request.field_name = Some(field_name.into());
        self
    }

    pub fn field_type(mut self, field_type: impl Into<String>) -> Self {
        self.request.field_type = Some(field_type.into());
        self
    }

    pub fn property(mut self, property: Value) -> Self {
        self.request.property = Some(property);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn ui_type(mut self, ui_type: impl Into<String>) -> Self {
        self.request.ui_type = Some(ui_type.into());
        self
    }

    pub fn build(self) -> UpdateFieldRequest {
        self.request
    }
}

/// 更新字段响应
#[derive(Clone)]
pub struct UpdateFieldResponse {
    /// 更新后的字段信息
    pub field: TableField,
}

impl ApiResponseTrait for UpdateFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 请求体结构
#[derive(Serialize)]
struct UpdateFieldRequestBody {
    #[serde(skip_serializing_if = Option::is_none)]
    field_name: Option<String>,
    #[serde(skip_serializing_if = Option::is_none)]
    r#type: Option<String>,
    #[serde(skip_serializing_if = Option::is_none)]
    property: Option<Value>,
    #[serde(skip_serializing_if = Option::is_none)]
    description: Option<String>,
    #[serde(skip_serializing_if = Option::is_none)]
    ui_type: Option<String>,
}

/// 更新字段
pub async fn update_field(
    request: UpdateFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<UpdateFieldResponse> {
    let mut api_req = request.api_request;
        let api_request = api_request.api_path(format!(        .replace({app_token}, &request.app_token)
        let api_request = api_request.api_path(format!(        .replace({table_id}, &request.table_id)
        let api_request = api_request.api_path(format!(        .replace({field_id}, &request.field_id);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
    }

    // 设置请求体
    let body = UpdateFieldRequestBody {
        field_name: request.field_name,
        r#type: request.field_type,
        property: request.property,
        description: request.description,
        ui_type: request.ui_type,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<UpdateFieldResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}


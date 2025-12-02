
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    
    config::Config,

    http::Transport,
    req_option::RequestOption,
    Response,    
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新字段请求
#[derive(Debug, Clone)]pub struct UpdateFieldRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 多维表格数据表的唯一标识符
    table_id: String,
    /// 字段的唯一标识符
    field_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 多维表格字段名
    field_name: Option<String>,
    /// 多维表格字段类型
    field_type: Option<String>,
    /// 字段属性
    property: Option<Value>,
    /// 字段的描述
    description: Option<String>,
    /// 字段在界面上的展示类型
    ui_type: Option<String>,
}

impl UpdateFieldRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}")
                
                ,
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
    field_name: Option<String>,
    r#type: Option<String>,
    property: Option<Value>,
    description: Option<String>,
    ui_type: Option<String>,
}

/// 更新字段
pub async fn update_field(
    request: UpdateFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<UpdateFieldResponse> {
    let mut api_req = request.api_request;
        let api_request = api_request
        let api_request = api_request
        let api_request = api_request;

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
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

    let api_resp: Response<UpdateFieldResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}


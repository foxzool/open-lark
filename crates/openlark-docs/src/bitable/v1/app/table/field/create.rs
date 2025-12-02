
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    
    config::Config,

    http::Transport,
    req_option::RequestOption,
    

    models::{FieldType, FieldProperty},
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 新增字段请求
#[derive(Debug, Clone)]pub struct CreateFieldRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 多维表格数据表的唯一标识符
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    client_token: Option<String>,
    /// 多维表格字段名
    field_name: String,
    /// 多维表格字段类型
    r#type: FieldType,
    /// 字段属性
    property: Option<FieldProperty>,
    /// 字段的描述
    description: Option<String>,
    /// 字段在界面上的展示类型
    ui_type: Option<String>,
}

impl CreateFieldRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/apps/{}/tables/{}/fields")
                
                ,
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            client_token: None,
            field_name: String::new(),
            r#type: FieldType::Text,
            property: None,
            description: None,
            ui_type: None,
        }
    }

    pub fn builder() -> CreateFieldRequestBuilder {
        CreateFieldRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateFieldRequestBuilder {
    request: CreateFieldRequest,
}

impl CreateFieldRequestBuilder {
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

    pub fn field_name(mut self, field_name: impl Into<String>) -> Self {
        self.request.field_name = field_name.into();
        self
    }

    pub fn field_type(mut self, field_type: FieldType) -> Self {
        self.request.r#type = field_type;
        self
    }

    pub fn property(mut self, property: FieldProperty) -> Self {
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

    pub fn build(self) -> CreateFieldRequest {
        self.request
    }
}

/// 请求体结构
#[derive(Serialize)]
struct CreateFieldRequestBody {
    field_name: String,
    r#type: FieldType,
    property: Option<FieldProperty>,
    description: Option<String>,
    ui_type: Option<String>,
}

/// 新增字段响应
#[derive(Debug, Clone)]
pub struct CreateFieldResponse {
    /// 新增的字段信息
    pub field: TableField,
}

impl ApiResponseTrait for CreateFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新增字段
pub async fn create_field(
    request: CreateFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<CreateFieldResponse> {
    let mut api_req = request.api_request;
        let api_request = api_request
        let api_request = api_request;

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
    }

    if let Some(client_token) = &request.client_token {
        api_req
    }

    // 设置请求体
    let body = CreateFieldRequestBody {
        field_name: request.field_name,
        r#type: request.r#type,
        property: request.property,
        description: request.description,
        ui_type: request.ui_type,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: Response<CreateFieldResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}


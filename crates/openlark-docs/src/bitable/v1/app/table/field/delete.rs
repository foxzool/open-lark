
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    
    config::Config,

    http::Transport,
    req_option::RequestOption,
    Response,    
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除字段请求
#[derive(Debug, Clone)]pub struct DeleteFieldRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 多维表格数据表的唯一标识符
    table_id: String,
    /// 字段的唯一标识符
    field_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl DeleteFieldRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}")
                
                ,
            app_token: String::new(),
            table_id: String::new(),
            field_id: String::new(),
            user_id_type: None,
        }
    }

    pub fn builder() -> DeleteFieldRequestBuilder {
        DeleteFieldRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteFieldRequestBuilder {
    request: DeleteFieldRequest,
}

impl DeleteFieldRequestBuilder {
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

    pub fn build(self) -> DeleteFieldRequest {
        self.request
    }
}

/// 删除字段响应
pub struct DeleteFieldResponse {
    /// 删除的字段信息
    pub field: TableField,
}

impl ApiResponseTrait for DeleteFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除字段
pub async fn delete_field(
    request: DeleteFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<DeleteFieldResponse> {
    let mut api_req = request.api_request;
        let api_request = api_request
        let api_request = api_request
        let api_request = api_request;

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
    }

    let api_resp: Response<DeleteFieldResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}


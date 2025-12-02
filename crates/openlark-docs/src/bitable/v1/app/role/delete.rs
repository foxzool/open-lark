
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,

    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除自定义角色请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 自定义角色的id
    #[serde(skip)]
    role_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
}

impl DeleteAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/apps/{}/roles/{}")
                
                ,
            app_token: String::new(),
            role_id: String::new(),
            user_id_type: None,
        }
    }

    pub fn builder() -> DeleteAppRoleRequestBuilder {
        DeleteAppRoleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteAppRoleRequestBuilder {
    request: DeleteAppRoleRequest,
}

impl DeleteAppRoleRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.request.role_id = role_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn build(self) -> DeleteAppRoleRequest {
        self.request
    }
}

/// 删除自定义角色响应
pub struct DeleteAppRoleResponse {
    /// 删除的角色ID
    pub role_id: String,
    /// 是否删除成功
    pub deleted: bool,
}

impl ApiResponseTrait for DeleteAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除自定义角色
pub async fn delete_app_role(
    request: DeleteAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<DeleteAppRoleResponse> {
    let mut api_req = request.api_request;
        let api_request = api_request
        let api_request = api_request;

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
    }

    let response: DeleteAppRoleResponse =
        Transport::request(api_request, config, option).await?;
    response
}



use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    config::Config,

    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 新增协作者请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 角色的唯一标识符
    #[serde(skip)]
    role_id: String,
}

impl CreateRoleMemberRequest {
    /// 创建新增协作者请求
    pub fn new(config: Config, app_token: impl Into<String>, role_id: impl Into<String>) -> Self {
        Self {
            api_request: ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/roles/{}/members",
                app_token.into(),
                role_id.into()
            ))
                .access_token_type(AccessTokenType::Tenant)
                ,
            app_token: app_token.into(),
            role_id: role_id.into(),
        }
    }

    /// 设置请求参数
    pub fn params(mut self, params: impl Serialize) -> Self {
        self.api_request = self.api_request.params(params);
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: impl Serialize) -> Self {
        self.api_request = self.api_request.body(body);
        self
    }

    /// 执行请求
    pub async fn send(self) -> SDKResult<BaseResponse> {
        self.api_request.send().await
    }
}

/// 新增协作者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleMemberResponse {
    /// 协作者信息
    pub member: Option<serde_json::Value>,
}

/// 新增协作者构建器
pub struct CreateRoleMemberBuilder {
    config: Config,
    app_token: String,
    role_id: String,
    request: CreateRoleMemberRequest,
}

impl CreateRoleMemberBuilder {
    /// 创建新增协作者构建器
    pub fn new(config: Config, app_token: impl Into<String>, role_id: impl Into<String>) -> Self {
        let app_token = app_token.into();
        let role_id = role_id.into();
        Self {
            request: CreateRoleMemberRequest::new(config.clone(), &app_token, &role_id),
            config,
            app_token,
            role_id,
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: impl Serialize) -> Self {
        self.request = self.request.body(body);
        self
    }

    /// 发送请求
    pub async fn send(self) -> SDKResult<BaseResponse> {
        self.request.send().await
    }
}
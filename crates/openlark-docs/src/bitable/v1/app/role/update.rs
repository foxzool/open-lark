
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,

    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新自定义角色请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppRoleRequest {
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
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<String>,
    /// 数据表权限
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

impl UpdateAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/apps/{}/roles/{}")
                
                ,
            app_token: String::new(),
            role_id: String::new(),
            user_id_type: None,
            role_name: None,
            table_roles: None,
            block_roles: None,
        }
    }

    pub fn builder() -> UpdateAppRoleRequestBuilder {
        UpdateAppRoleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateAppRoleRequestBuilder {
    request: UpdateAppRoleRequest,
}

impl UpdateAppRoleRequestBuilder {
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

    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.request.role_name = Some(role_name.into());
        self
    }

    pub fn table_roles(mut self, table_roles: Vec<TableRole>) -> Self {
        self.request.table_roles = Some(table_roles);
        self
    }

    pub fn block_roles(mut self, block_roles: Vec<BlockRole>) -> Self {
        self.request.block_roles = Some(block_roles);
        self
    }

    pub fn build(self) -> UpdateAppRoleRequest {
        self.request
    }
}

#[derive(Serialize)]
/// 数据表权限
pub struct TableRole {
    /// 数据表 id
    pub table_id: String,
    /// 权限
    pub role: String,
    /// 记录权限
    #[serde(skip_serializing_if = "Option::is_none")]
    rec_rule: Option<String>,
}

impl TableRole {
    pub fn new(table_id: impl ToString, role: impl ToString) -> Self {
        Self {
            table_id: table_id.to_string(),
            role: role.to_string(),
            rec_rule: None,
        }
    }

    pub fn with_rec_rule(mut self, rec_rule: impl ToString) -> Self {
        self.rec_rule = Some(rec_rule.to_string());
        self
    }
}

/// 数据表默认权限
pub struct BlockRole {
    /// 多维表格数据表的唯一标识符
    pub block_id: String,
    /// 权限
    pub role: String,
}

impl BlockRole {
    pub fn new(block_id: impl ToString, role: impl ToString) -> Self {
        Self {
            block_id: block_id.to_string(),
            role: role.to_string(),
        }
    }
}

#[derive(Serialize)]
/// 自定义角色信息
pub struct AppRole {
    /// 自定义角色的id
    pub role_id: String,
    /// 角色名称
    pub role_name: String,
    /// 数据表权限
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

/// 请求体结构
#[derive(Serialize)]
struct UpdateAppRoleRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    role_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

/// 更新自定义角色响应
pub struct UpdateAppRoleResponse {
    /// 更新后的角色信息
    pub role: AppRole,
}

impl ApiResponseTrait for UpdateAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新自定义角色
pub async fn update_app_role(
    request: UpdateAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<UpdateAppRoleResponse> {
    let mut api_req = request.api_request;
        let api_request = api_request
        let api_request = api_request;

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
    }

    // 设置请求体
    let body = UpdateAppRoleRequestBody {
        role_name: request.role_name,
        table_roles: request.table_roles,
        block_roles: request.block_roles,
    };

    api_req.body(serde_json::to_vec(.body(serde_json::to_vec(&body).unwrap();body).unwrap());

    let response: UpdateAppRoleResponse =
        Transport::request(api_request, config, option).await?;
    response
}


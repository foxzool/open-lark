
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 新增自定义角色请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAppRoleRequest {
    #[serde(skip)]
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 角色名称
    role_name: String,
    /// 数据表权限
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

impl CreateAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("").header("Content-Type", "application/json"),
            app_token: String::new(),
            role_name: String::new(),
            table_roles: None,
            block_roles: None,
        }
    }

    pub fn builder() -> CreateAppRoleRequestBuilder {
        CreateAppRoleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateAppRoleRequestBuilder {
    request: CreateAppRoleRequest,
}

impl CreateAppRoleRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.request.role_name = role_name.into();
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

    pub fn build(self) -> CreateAppRoleRequest {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// 请求体结构
#[derive(Serialize)]
struct CreateAppRoleRequestBody {
    role_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_roles: Option<Vec<TableRole>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

/// 自定义角色信息
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// 新增自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAppRoleResponse {
    /// 新增的自定义角色信息
    pub role: AppRole,
}

impl ApiResponseTrait for CreateAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新增自定义角色
pub async fn create_app_role(
    request: CreateAppRoleRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<CreateAppRoleResponse> {
    let api_path = format!("/open-apis/bitable/v1/apps/{}/roles", request.app_token);
    let mut api_request = ApiRequest::post(api_path)
        .header("Content-Type", "application/json");

    // 设置请求体
    let body = CreateAppRoleRequestBody {
        role_name: request.role_name,
        table_roles: request.table_roles,
        block_roles: request.block_roles,
    };

    api_request = api_request.body(body);

    let response: CreateAppRoleResponse =
        Transport::request(api_request, config, option).await?;
    Ok(response)
}


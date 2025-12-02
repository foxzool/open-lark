
//! Bitable V1 创建角色API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{SDKResult, validation_error},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 创建角色请求
#[derive(Debug, Clone)]
pub struct CreateAppRoleRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<CreateAppRoleResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 角色名称
    role_name: String,
    /// 数据表权限
    table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    block_roles: Option<Vec<BlockRole>>,
}

impl CreateAppRoleRequest {
    /// 创建角色请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::post("").header("Content-Type", "application/json"),
            app_token: String::new(),
            role_name: String::new(),
            table_roles: None,
            block_roles: None,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置角色名称
    pub fn role_name(mut self, role_name: String) -> Self {
        self.role_name = role_name;
        self
    }

    /// 设置数据表权限
    pub fn table_roles(mut self, table_roles: Vec<TableRole>) -> Self {
        self.table_roles = Some(table_roles);
        self
    }

    /// 设置数据表默认权限
    pub fn block_roles(mut self, block_roles: Vec<BlockRole>) -> Self {
        self.block_roles = Some(block_roles);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateAppRoleResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.role_name.trim().is_empty() {
            return Err(validation_error("role_name", "角色名称不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!("{}/open-apis/bitable/v1/apps/{}/roles",
                             self.config.base_url, self.app_token);

        // 构建请求体
        let request_body = CreateAppRoleRequestBody {
            role_name: self.role_name,
            table_roles: self.table_roles,
            block_roles: self.block_roles,
        };

        // 设置API URL和请求体
        let mut api_request = self.api_request;
        api_request = api_request.api_path(api_url);
        api_request = api_request.body(serde_json::to_vec(&request_body)?);

        // 发送请求
        let response: CreateAppRoleResponse =
            Transport::request(api_request, &self.config, None).await?;

        Ok(response)
    }
}

/// 创建角色Builder
pub struct CreateAppRoleRequestBuilder {
    request: CreateAppRoleRequest,
}

impl CreateAppRoleRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateAppRoleRequest::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置角色名称
    pub fn role_name(mut self, role_name: String) -> Self {
        self.request = self.request.role_name(role_name);
        self
    }

    /// 设置数据表权限
    pub fn table_roles(mut self, table_roles: Vec<TableRole>) -> Self {
        self.request = self.request.table_roles(table_roles);
        self
    }

    /// 设置数据表默认权限
    pub fn block_roles(mut self, block_roles: Vec<BlockRole>) -> Self {
        self.request = self.request.block_roles(block_roles);
        self
    }

    /// 构建请求
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
pub struct CreateAppRoleRequestBody {
    /// 角色名称
    pub role_name: String,
    /// 数据表权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_roles: Option<Vec<BlockRole>>,
}

/// 角色信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Role {
    /// 角色ID
    pub role_id: String,
    /// 角色名称
    pub role_name: String,
    /// 角色类型
    pub role_type: i32,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据表权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_roles: Option<Vec<TableRole>>,
    /// 数据表默认权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_roles: Option<Vec<BlockRole>>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

/// 创建角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAppRoleResponse {
    /// 角色信息
    pub role: Option<Role>,
    /// 操作结果
    pub success: bool,
}


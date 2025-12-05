
//! Bitable V1 列出角色API

use openlark_core::{
    api::{ApiRequest},
    config::Config,
    error::{SDKResult, validation_error},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 列出角色请求
#[derive(Debug, Clone)]
pub struct ListAppRoleRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<ListAppRoleResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 分页标记
    page_token: Option<String>,
    /// 分页大小
    page_size: Option<i32>,
}

impl ListAppRoleRequest {
    /// 创建列出角色请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::get(""),
            app_token: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListAppRoleResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!("{}/open-apis/bitable/v1/apps/{}/roles",
                             self.config.base_url, self.app_token);

        // 设置API URL和查询参数
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 设置查询参数
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        if let Some(page_size) = &self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;

        // 解析响应
        let response_data: ListAppRoleResponse = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析角色列表数据失败", "响应数据格式不正确"))?;

        Ok(response_data)
    }
}

/// 列出角色Builder
pub struct ListAppRoleRequestBuilder {
    request: ListAppRoleRequest,
}

impl ListAppRoleRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ListAppRoleRequest::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 构建请求
    pub fn build(self) -> ListAppRoleRequest {
        self.request
    }
}

/// 数据表权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableRole {
    /// 数据表 id
    pub table_id: String,
    /// 权限
    pub role: String,
    /// 记录权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_rule: Option<String>,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// 列出角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListAppRoleResponse {
    /// 角色信息列表
    pub items: Option<Vec<Role>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 操作结果
    pub success: bool,
}


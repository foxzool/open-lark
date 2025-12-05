
//! Bitable V1 创建角色成员API

use openlark_core::{
    api::ApiRequest,
    config::Config,
    error::{SDKResult, validation_error},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 创建角色成员请求
#[derive(Debug, Clone)]
pub struct CreateRoleMemberRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<CreateRoleMemberResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 角色的ID
    role_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 用户ID列表
    user_ids: Vec<String>,
    /// 成员类型
    member_type: String,
}

impl CreateRoleMemberRequest {
    /// 创建角色成员请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::post("").header("Content-Type", "application/json"),
            app_token: String::new(),
            role_id: String::new(),
            user_id_type: None,
            user_ids: Vec::new(),
            member_type: String::new(),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置角色 ID
    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置用户ID列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 添加用户ID
    pub fn add_user_id(mut self, user_id: String) -> Self {
        self.user_ids.push(user_id);
        self
    }

    /// 设置成员类型
    pub fn member_type(mut self, member_type: String) -> Self {
        self.member_type = member_type;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateRoleMemberResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.role_id.trim().is_empty() {
            return Err(validation_error("role_id", "角色ID不能为空"));
        }

        if self.user_ids.is_empty() {
            return Err(validation_error("user_ids", "用户ID列表不能为空"));
        }

        if self.member_type.trim().is_empty() {
            return Err(validation_error("member_type", "成员类型不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!("{}/open-apis/bitable/v1/apps/{}/roles/{}/members",
                             self.config.base_url, self.app_token, self.role_id);

        // 构建请求体
        let request_body = CreateRoleMemberRequestBody {
            user_ids: self.user_ids,
            member_type: self.member_type,
        };

        // 设置API URL和请求体
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 设置查询参数
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        api_request.body = Some(openlark_core::api::RequestData::Json(serde_json::to_value(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;

        // 解析响应
        let member_data: RoleMember = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析角色成员数据失败", "响应数据格式不正确"))?;

        Ok(CreateRoleMemberResponse {
            member: Some(member_data),
            success: response.raw_response.is_success(),
        })
    }
}

/// 创建角色成员Builder
pub struct CreateRoleMemberRequestBuilder {
    request: CreateRoleMemberRequest,
}

impl CreateRoleMemberRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateRoleMemberRequest::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置角色 ID
    pub fn role_id(mut self, role_id: String) -> Self {
        self.request = self.request.role_id(role_id);
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 设置用户ID列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.request = self.request.user_ids(user_ids);
        self
    }

    /// 添加用户ID
    pub fn add_user_id(mut self, user_id: String) -> Self {
        self.request = self.request.add_user_id(user_id);
        self
    }

    /// 设置成员类型
    pub fn member_type(mut self, member_type: String) -> Self {
        self.request = self.request.member_type(member_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateRoleMemberRequest {
        self.request
    }
}

/// 请求体结构
#[derive(Serialize)]
pub struct CreateRoleMemberRequestBody {
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 成员类型
    pub member_type: String,
}

/// 角色成员信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleMember {
    /// 成员ID
    pub member_id: String,
    /// 成员类型
    pub member_type: String,
    /// 用户ID
    pub user_id: String,
    /// 成员姓名
    pub name: String,
    /// 成员邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 成员头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 是否为管理员
    pub is_admin: bool,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 创建角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoleMemberResponse {
    /// 成员信息
    pub member: Option<RoleMember>,
    /// 操作结果
    pub success: bool,
}
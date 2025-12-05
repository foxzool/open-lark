//! Bitable V1 列出角色成员API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{SDKResult, validation_error},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 列出角色成员请求
#[derive(Debug, Clone)]
pub struct ListRoleMembersRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<ListRoleMembersResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 角色的ID
    role_id: String,
    /// 页面大小
    page_size: Option<i32>,
    /// 页面标记
    page_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 成员类型过滤
    member_type: Option<String>,
}

impl ListRoleMembersRequest {
    /// 创建列出角色成员请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::get("").header("Content-Type", "application/json"),
            app_token: String::new(),
            role_id: String::new(),
            page_size: None,
            page_token: None,
            user_id_type: None,
            member_type: None,
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

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置页面标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置成员类型过滤
    pub fn member_type(mut self, member_type: String) -> Self {
        self.member_type = Some(member_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListRoleMembersResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.role_id.trim().is_empty() {
            return Err(validation_error("role_id", "角色ID不能为空"));
        }

        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err(validation_error("page_size", "页面大小必须在1-100之间"));
            }
        }

        // 构建完整的API URL
        let api_url = format!("{}/open-apis/bitable/v1/apps/{}/roles/{}/members",
                             self.config.base_url, self.app_token, self.role_id);

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 设置查询参数
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }

        if let Some(ref page_token) = self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        if let Some(ref member_type) = self.member_type {
            api_request = api_request.query("member_type", member_type);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;

        // 解析响应
        let member_data: ListRoleMembersData = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析角色成员列表数据失败", "响应数据格式不正确"))?;

        Ok(ListRoleMembersResponse {
            data: member_data,
            success: response.raw_response.is_success(),
        })
    }
}

/// 列出角色成员Builder
pub struct ListRoleMembersRequestBuilder {
    request: ListRoleMembersRequest,
}

impl ListRoleMembersRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ListRoleMembersRequest::new(config),
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

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置页面标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 设置成员类型过滤
    pub fn member_type(mut self, member_type: String) -> Self {
        self.request = self.request.member_type(member_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> ListRoleMembersRequest {
        self.request
    }
}

/// 角色成员列表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRoleMembersData {
    /// 成员列表
    pub items: Vec<RoleMemberListItem>,
    /// 分页信息
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

/// 角色成员信息（列表视图）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleMemberListItem {
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

/// 列出角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRoleMembersResponse {
    /// 成员列表数据
    pub data: ListRoleMembersData,
    /// 操作结果
    pub success: bool,
}

impl ApiResponseTrait for ListRoleMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl super::RoleMemberService {
    /// 创建列出角色成员请求构建器
    pub fn list_members_builder(&self) -> ListRoleMembersRequestBuilder {
        ListRoleMembersRequestBuilder::new(self.config.clone())
    }

    /// 创建列出角色成员请求
    pub fn list_members(
        &self,
        app_token: String,
        role_id: String,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<String>,
        member_type: Option<String>,
    ) -> ListRoleMembersRequest {
        let mut request = ListRoleMembersRequest::new(self.config.clone())
            .app_token(app_token)
            .role_id(role_id);

        if let Some(page_size) = page_size {
            request = request.page_size(page_size);
        }

        if let Some(page_token) = page_token {
            request = request.page_token(page_token);
        }

        if let Some(user_id_type) = user_id_type {
            request = request.user_id_type(user_id_type);
        }

        if let Some(member_type) = member_type {
            request = request.member_type(member_type);
        }

        request
    }
}
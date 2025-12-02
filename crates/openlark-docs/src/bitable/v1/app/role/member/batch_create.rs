//! Bitable V1 批量创建角色成员API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{SDKResult, validation_error},
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::{BatchCreateRoleMemberRequest as ModelBatchCreateRequest, BatchCreateRoleMemberResponse as ModelBatchCreateResponse, BatchCreateMemberItem as ModelBatchCreateMemberItem};

/// 批量创建角色成员请求
#[derive(Debug, Clone)]
pub struct BatchCreateRoleMemberRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<BatchCreateRoleMemberResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 角色的ID
    role_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 成员列表
    member_list: Vec<BatchCreateMemberItem>,
}

/// 批量创建成员项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateMemberItem {
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 成员类型
    pub member_type: String,
    /// 权限列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl BatchCreateRoleMemberRequest {
    /// 创建批量创建角色成员请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::post("").header("Content-Type", "application/json"),
            app_token: String::new(),
            role_id: String::new(),
            user_id_type: None,
            member_list: Vec::new(),
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

    /// 设置成员列表
    pub fn member_list(mut self, member_list: Vec<BatchCreateMemberItem>) -> Self {
        self.member_list = member_list;
        self
    }

    /// 添加成员
    pub fn add_member(mut self, user_ids: Vec<String>, member_type: String) -> Self {
        self.member_list.push(BatchCreateMemberItem {
            user_ids,
            member_type,
            permissions: None,
        });
        self
    }

    /// 添加成员（带权限）
    pub fn add_member_with_permissions(mut self, user_ids: Vec<String>, member_type: String, permissions: Vec<String>) -> Self {
        self.member_list.push(BatchCreateMemberItem {
            user_ids,
            member_type,
            permissions: Some(permissions),
        });
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchCreateRoleMemberResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.role_id.trim().is_empty() {
            return Err(validation_error("role_id", "角色ID不能为空"));
        }

        if self.member_list.is_empty() {
            return Err(validation_error("member_list", "成员列表不能为空"));
        }

        // 验证每个成员项
        for (index, member) in self.member_list.iter().enumerate() {
            if member.user_ids.is_empty() {
                return Err(validation_error("member_list", &format!("第{}个成员的用户ID列表不能为空", index + 1)));
            }
            if member.member_type.trim().is_empty() {
                return Err(validation_error("member_list", &format!("第{}个成员的类型不能为空", index + 1)));
            }
        }

        // 构建完整的API URL
        let api_url = format!("{}/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_create",
                             self.config.base_url, self.app_token, self.role_id);

        // 构建请求体
        let request_body = ModelBatchCreateRequest {
            member_list: self.member_list.iter().map(|item| ModelBatchCreateMemberItem {
                user_ids: item.user_ids.clone(),
                member_type: item.member_type.clone(),
                permissions: item.permissions.clone(),
            }).collect(),
        };

        // 设置API URL和请求体
        let mut api_request = self.api_request;
        api_request = api_request.api_path(api_url);

        // 设置查询参数
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        api_request = api_request.body(serde_json::to_vec(&request_body)?);

        // 发送请求
        let response: ModelBatchCreateResponse =
            Transport::request(api_request, &self.config, None).await?;

        // 转换为标准响应格式
        let standard_response = BatchCreateRoleMemberResponse {
            results: response.results,
            has_more: response.has_more,
            page_token: response.page_token,
            success: true,
        };

        Ok(standard_response)
    }
}

/// 批量创建角色成员Builder
pub struct BatchCreateRoleMemberRequestBuilder {
    request: BatchCreateRoleMemberRequest,
}

impl BatchCreateRoleMemberRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchCreateRoleMemberRequest::new(config),
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

    /// 设置成员列表
    pub fn member_list(mut self, member_list: Vec<BatchCreateMemberItem>) -> Self {
        self.request = self.request.member_list(member_list);
        self
    }

    /// 添加成员
    pub fn add_member(mut self, user_ids: Vec<String>, member_type: String) -> Self {
        self.request = self.request.add_member(user_ids, member_type);
        self
    }

    /// 添加成员（带权限）
    pub fn add_member_with_permissions(mut self, user_ids: Vec<String>, member_type: String, permissions: Vec<String>) -> Self {
        self.request = self.request.add_member_with_permissions(user_ids, member_type, permissions);
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchCreateRoleMemberRequest {
        self.request
    }
}

/// 角色成员信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleMemberInfo {
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
    /// 权限列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// 是否为管理员
    pub is_admin: bool,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

/// 批量创建结果项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateResultItem {
    /// 用户ID
    pub user_id: String,
    /// 操作结果
    pub success: bool,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// 成员信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<RoleMemberInfo>,
}

/// 批量创建角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRoleMemberResponse {
    /// 批量操作结果
    pub results: Vec<BatchCreateResultItem>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 页面 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 操作结果
    pub success: bool,
}
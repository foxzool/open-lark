//! Bitable 删除角色成员API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/role/member/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 删除角色成员请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DeleteRoleMemberRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<DeleteRoleMemberResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 角色的ID
    role_id: String,
    /// 成员ID
    member_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl DeleteRoleMemberRequest {
    /// 创建删除角色成员请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::delete("").header("Content-Type", "application/json"),
            app_token: String::new(),
            role_id: String::new(),
            member_id: String::new(),
            user_id_type: None,
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

    /// 设置成员 ID
    pub fn member_id(mut self, member_id: String) -> Self {
        self.member_id = member_id;
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteRoleMemberResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.role_id.trim().is_empty() {
            return Err(validation_error("role_id", "角色ID不能为空"));
        }

        if self.member_id.trim().is_empty() {
            return Err(validation_error("member_id", "成员ID不能为空"));
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RoleMemberDelete(
            self.app_token.clone(),
            self.role_id.clone(),
            self.member_id.clone(),
        );

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<DeleteRoleMemberResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        // 设置查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        // 发送请求
        let response: openlark_core::api::Response<()> =
            Transport::request(api_request, &self.config, None).await?;

        Ok(DeleteRoleMemberResponse {
            success: response.raw_response.is_success(),
            member_id: self.member_id,
        })
    }
}

/// 删除角色成员Builder
pub struct DeleteRoleMemberRequestBuilder {
    request: DeleteRoleMemberRequest,
}

impl DeleteRoleMemberRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteRoleMemberRequest::new(config),
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

    /// 设置成员 ID
    pub fn member_id(mut self, member_id: String) -> Self {
        self.request = self.request.member_id(member_id);
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> DeleteRoleMemberRequest {
        self.request
    }
}

/// 删除角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteRoleMemberResponse {
    /// 操作结果
    pub success: bool,
    /// 被删除的成员ID
    pub member_id: String,
}

impl ApiResponseTrait for DeleteRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl super::RoleMemberService {
    /// 创建删除角色成员请求构建器
    pub fn delete_member_builder(&self) -> DeleteRoleMemberRequestBuilder {
        DeleteRoleMemberRequestBuilder::new(self.config.clone())
    }

    /// 创建删除角色成员请求
    pub fn delete_member(
        &self,
        app_token: String,
        role_id: String,
        member_id: String,
        user_id_type: Option<String>,
    ) -> DeleteRoleMemberRequest {
        let mut request = DeleteRoleMemberRequest::new(self.config.clone())
            .app_token(app_token)
            .role_id(role_id)
            .member_id(member_id);

        if let Some(user_id_type) = user_id_type {
            request = request.user_id_type(user_id_type);
        }

        request
    }
}

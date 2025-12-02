//! Bitable V1 批量删除协作者API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::{BatchDeleteRoleMemberRequest, BatchDeleteRoleMemberResponse};
use super::RoleMemberService;

/// 批量删除协作者请求
pub struct BatchDeleteRoleMemberV1Request {
    api_request: ApiRequest<BatchDeleteRoleMemberV1Response>,
    app_token: String,
    role_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 协作者ID列表
    member_ids: Vec<String>,
}

/// 批量删除协作者响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDeleteRoleMemberV1Response {
    /// 批量操作结果
    pub data: BatchDeleteRoleMemberResponse,
    pub success: bool,
}

impl BatchDeleteRoleMemberV1Request {
    /// 创建批量删除协作者请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_delete")
                
                ,
            app_token: String::new(),
            role_id: String::new(),
            user_id_type: None,
            member_ids: Vec::new(),
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

    /// 设置协作者ID列表
    pub fn member_ids(mut self, member_ids: Vec<String>) -> Self {
        self.member_ids = member_ids;
        self
    }

    /// 添加协作者ID
    pub fn add_member_id(mut self, member_id: String) -> Self {
        self.member_ids.push(member_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchDeleteRoleMemberV1Response> {
        // 构建API路径
        let path = format!(
            "/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_delete",
            self.app_token, self.role_id
        );

        // 更新API路径
        let mut api_request = self.api_request.api_path(path);

        // 构建请求体
        let request_body = BatchDeleteRoleMemberRequest {
            member_ids: self.member_ids.clone(),
        };

        // 设置查询参数
        if let Some(user_id_type) = self.user_id_type {
            api_request = api_request.query_param("user_id_type", user_id_type);
        }

        // 设置请求体
        api_request = api_request.body(serde_json::to_vec(&request_body)?);

        // 发送请求
        let config = api_request.config();
        let response = Transport::request(api_request, &config, None).await?;
        Ok(response)
    }
}

/// 批量删除协作者Builder
pub struct BatchDeleteRoleMemberV1Builder {
    request: BatchDeleteRoleMemberV1Request,
}

impl BatchDeleteRoleMemberV1Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchDeleteRoleMemberV1Request::new(config),
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

    /// 设置协作者ID列表
    pub fn member_ids(mut self, member_ids: Vec<String>) -> Self {
        self.request = self.request.member_ids(member_ids);
        self
    }

    /// 添加协作者ID
    pub fn add_member_id(mut self, member_id: String) -> Self {
        self.request = self.request.add_member_id(member_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchDeleteRoleMemberV1Request {
        self.request
    }
}

impl RoleMemberService {
    /// 创建批量删除协作者请求构建器
    pub fn batch_delete_role_member_v1_builder(&self) -> BatchDeleteRoleMemberV1Builder {
        BatchDeleteRoleMemberV1Builder::new(self.config.clone())
    }

    /// 创建批量删除协作者请求
    pub fn batch_delete_role_member_v1(
        &self,
        app_token: String,
        role_id: String,
        user_id_type: Option<String>,
        member_ids: Vec<String>,
    ) -> BatchDeleteRoleMemberV1Request {
        let mut request = BatchDeleteRoleMemberV1Request::new(self.config.clone())
            .app_token(app_token)
            .role_id(role_id);

        if let Some(user_id_type) = user_id_type {
            request = request.user_id_type(user_id_type);
        }

        request.member_ids(member_ids)
    }
}
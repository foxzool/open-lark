//! Bitable V1 批量新增协作者API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::{BatchCreateRoleMemberRequest, BatchCreateRoleMemberResponse};
use super::RoleMemberService;

/// 批量新增协作者请求
pub struct BatchCreateRoleMemberV1Request {
    api_request: ApiRequest<BatchCreateRoleMemberV1Response>,
    app_token: String,
    role_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 协作者列表
    member_list: Vec<BatchCreateMemberItem>,
}

/// 批量新增协作者项
pub struct BatchCreateMemberItem {
    /// 用户ID列表
    pub user_ids: Vec<String>,
    /// 协作者类型
    pub member_type: String,
}

/// 批量新增协作者响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRoleMemberV1Response {
    /// 批量操作结果
    pub data: BatchCreateRoleMemberResponse,
    pub success: bool,
}

impl BatchCreateRoleMemberV1Request {
    /// 创建批量新增协作者请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_create")
                
                ,
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

    /// 设置协作者列表
    pub fn member_list(mut self, member_list: Vec<BatchCreateMemberItem>) -> Self {
        self.member_list = member_list;
        self
    }

    /// 添加协作者
    pub fn add_member(mut self, user_ids: Vec<String>, member_type: String) -> Self {
        self.member_list.push(BatchCreateMemberItem {
            user_ids,
            member_type,
        });
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchCreateRoleMemberV1Response> {
        // 构建API路径
        let path = format!(
            "/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_create",
            self.app_token, self.role_id
        );

        // 更新API路径
        let mut api_request = self.api_request.api_path(path);

        // 构建请求体
        let request_body = BatchCreateRoleMemberRequest {
            member_list: self.member_list.clone(),
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

/// 批量新增协作者Builder
pub struct BatchCreateRoleMemberV1Builder {
    request: BatchCreateRoleMemberV1Request,
}

impl BatchCreateRoleMemberV1Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchCreateRoleMemberV1Request::new(config),
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

    /// 设置协作者列表
    pub fn member_list(mut self, member_list: Vec<BatchCreateMemberItem>) -> Self {
        self.request = self.request.member_list(member_list);
        self
    }

    /// 添加协作者
    pub fn add_member(mut self, user_ids: Vec<String>, member_type: String) -> Self {
        self.request = self.request.add_member(user_ids, member_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchCreateRoleMemberV1Request {
        self.request
    }
}

impl RoleMemberService {
    /// 创建批量新增协作者请求构建器
    pub fn batch_create_role_member_v1_builder(&self) -> BatchCreateRoleMemberV1Builder {
        BatchCreateRoleMemberV1Builder::new(self.config.clone())
    }

    /// 创建批量新增协作者请求
    pub fn batch_create_role_member_v1(
        &self,
        app_token: String,
        role_id: String,
        user_id_type: Option<String>,
        member_list: Vec<BatchCreateMemberItem>,
    ) -> BatchCreateRoleMemberV1Request {
        let mut request = BatchCreateRoleMemberV1Request::new(self.config.clone())
            .app_token(app_token)
            .role_id(role_id);

        if let Some(user_id_type) = user_id_type {
            request = request.user_id_type(user_id_type);
        }

        request.member_list(member_list)
    }
}
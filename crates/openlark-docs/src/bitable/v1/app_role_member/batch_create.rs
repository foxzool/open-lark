#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use openlark_core::{
use serde_json::Value;
    api::ApiRequest,
    core::{BaseResponse, ResponseFormat, api::ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    service::bitable::v1::app_role_member::RoleMember,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::AppRoleMemberService;

/// 批量新增协作者请求
#[derive(Clone)]
pub struct BatchCreateRoleMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的 app_token
    #[serde(skip)]
    app_token: String,
    /// 角色 ID
    #[serde(skip)]
    role_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    #[serde(skip)]
    client_token: Option<String>,
    /// 协作者列表
    pub members: Vec<RoleMember>,
}

impl BatchCreateRoleMemberRequest {
    /// 创建批量新增协作者请求
    pub fn new(app_token: String, role_id: String, members: Vec<RoleMember>) -> Self {
        Self {
            api_request: ApiRequest::post("/open-apis/bitable/v1/placeholder")
                .method(reqwest::Method::POST)
                .path("/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_create"),
            app_token,
            role_id,
            user_id_type: None,
            client_token: None,
            members,
        }
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置客户端令牌
    pub fn client_token(mut self, client_token: String) -> Self {
        self.client_token = Some(client_token);
        self
    }

    /// 执行批量新增协作者请求
    pub async fn execute(&self, transport: &Transport) -> SDKResult<serde_json::Value> {
        let path = self.api_request.path()
            .replace(":app_token", &self.app_token)
            .replace(":role_id", &self.role_id);

        let mut api_request = self.api_request.clone();
        api_request = api_request.path(&path);

        Transport::request(api_request, &self.config, None).await
    }
}

impl AppRoleMemberService {
    pub fn batch_create_builder(&self, role_id: String, members: Vec<RoleMember>) -> BatchCreateRoleMemberRequest {
        BatchCreateRoleMemberRequest::new(self.app_token.clone(), role_id, members)
    }
}
use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::AppRoleMemberService;
use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::bitable::v1::app_role_member::RoleMember,
};

/// 批量新增协作者请求
#[derive(Debug, Serialize, Default)]
pub struct BatchCreateRoleMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 自定义角色的id
    #[serde(skip)]
    role_id: String,
    /// 用户id类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 成员列表
    members: Vec<MemberInfo>,
}

/// 成员信息
#[derive(Debug, Serialize, Deserialize)]
pub struct MemberInfo {
    /// 成员id
    pub member_id: String,
    /// 成员类型: user, chat, department, open_department_id
    pub member_type: String,
}

impl BatchCreateRoleMemberRequest {
    pub fn builder() -> BatchCreateRoleMemberRequestBuilder {
        BatchCreateRoleMemberRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, role_id: impl ToString, members: Vec<MemberInfo>) -> Self {
        Self {
            app_token: app_token.to_string(),
            role_id: role_id.to_string(),
            members,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct BatchCreateRoleMemberRequestBuilder {
    request: BatchCreateRoleMemberRequest,
}

impl BatchCreateRoleMemberRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 自定义角色的id
    pub fn role_id(mut self, role_id: impl ToString) -> Self {
        self.request.role_id = role_id.to_string();
        self
    }

    /// 用户id类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 成员列表
    pub fn members(mut self, members: Vec<MemberInfo>) -> Self {
        self.request.members = members;
        self
    }

    /// 添加单个成员
    pub fn add_member(mut self, member_id: impl ToString, member_type: impl ToString) -> Self {
        self.request.members.push(MemberInfo {
            member_id: member_id.to_string(),
            member_type: member_type.to_string(),
        });
        self
    }

    pub fn build(mut self) -> BatchCreateRoleMemberRequest {
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_request
                .query_params
                .insert("user_id_type", user_id_type.clone());
        }
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    BatchCreateRoleMemberRequestBuilder,
    AppRoleMemberService,
    BatchCreateRoleMemberRequest,
    BaseResponse<BatchCreateRoleMemberResponse>,
    batch_create
);

/// 批量新增协作者响应
#[derive(Debug, Deserialize)]
pub struct BatchCreateRoleMemberResponse {
    /// 新增的协作者信息列表
    pub members: Vec<RoleMember>,
}

impl ApiResponseTrait for BatchCreateRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量新增协作者
pub async fn batch_create_role_members(
    request: BatchCreateRoleMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<BatchCreateRoleMemberResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = BITABLE_V1_ROLE_MEMBERS_BATCH_CREATE
        .replace("{app_token}", &request.app_token)
        .replace("{role_id}", &request.role_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_create_role_member_request_builder() {
        let request = BatchCreateRoleMemberRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .role_id("rolxxxxxx")
            .add_member("ou_xxxxxx", "user")
            .add_member("ou_yyyyyy", "user")
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.members.len(), 2);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }
}

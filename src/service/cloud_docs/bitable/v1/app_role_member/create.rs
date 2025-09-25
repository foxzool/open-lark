use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::AppRoleMemberService;

impl AppRoleMemberService {
    /// 新增协作者
    pub async fn create(
        &self,
        request: CreateRoleMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateRoleMemberResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = BITABLE_V1_ROLE_MEMBERS
            .replace("{app_token}", &request.app_token)
            .replace("{role_id}", &request.role_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 新增协作者请求
#[derive(Debug, Serialize, Default)]
pub struct CreateRoleMemberRequest {
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
    /// 成员id
    member_id: String,
    /// 成员类型: user, chat, department, open_department_id
    member_type: String,
}

impl CreateRoleMemberRequest {
    pub fn builder() -> CreateRoleMemberRequestBuilder {
        CreateRoleMemberRequestBuilder::default()
    }

    pub fn new(
        app_token: impl ToString,
        role_id: impl ToString,
        member_id: impl ToString,
        member_type: impl ToString,
    ) -> Self {
        Self {
            app_token: app_token.to_string(),
            role_id: role_id.to_string(),
            member_id: member_id.to_string(),
            member_type: member_type.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct CreateRoleMemberRequestBuilder {
    request: CreateRoleMemberRequest,
}

impl CreateRoleMemberRequestBuilder {
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

    /// 成员id
    pub fn member_id(mut self, member_id: impl ToString) -> Self {
        self.request.member_id = member_id.to_string();
        self
    }

    /// 成员类型
    pub fn member_type(mut self, member_type: impl ToString) -> Self {
        self.request.member_type = member_type.to_string();
        self
    }

    pub fn build(mut self) -> CreateRoleMemberRequest {
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
    CreateRoleMemberRequestBuilder,
    AppRoleMemberService,
    CreateRoleMemberRequest,
    BaseResponse<CreateRoleMemberResponse>,
    create
);

/// 协作者信息
#[derive(Debug, Deserialize)]
pub struct RoleMember {
    /// 成员id
    pub member_id: String,
    /// 成员类型
    pub member_type: String,
    /// 成员名称
    pub member_name: Option<String>,
}

/// 新增协作者响应
#[derive(Debug, Deserialize)]
pub struct CreateRoleMemberResponse {
    /// 新增的协作者信息
    pub member: RoleMember,
}

impl ApiResponseTrait for CreateRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_create_role_member_request_builder() {
        let request = CreateRoleMemberRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .role_id("rolxxxxxx")
            .member_id("ou_xxxxxx")
            .member_type("user")
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.member_id, "ou_xxxxxx");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }
}

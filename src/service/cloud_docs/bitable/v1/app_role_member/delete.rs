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
};

/// 删除协作者请求
#[derive(Debug, Serialize, Default)]
pub struct DeleteRoleMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 自定义角色的id
    #[serde(skip)]
    role_id: String,
    /// 成员id
    #[serde(skip)]
    member_id: String,
    /// 用户id类型
    #[serde(skip)]
    user_id_type: Option<String>,
}

impl DeleteRoleMemberRequest {
    pub fn builder() -> DeleteRoleMemberRequestBuilder {
        DeleteRoleMemberRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, role_id: impl ToString, member_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            role_id: role_id.to_string(),
            member_id: member_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct DeleteRoleMemberRequestBuilder {
    request: DeleteRoleMemberRequest,
}

impl DeleteRoleMemberRequestBuilder {
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

    /// 成员id
    pub fn member_id(mut self, member_id: impl ToString) -> Self {
        self.request.member_id = member_id.to_string();
        self
    }

    /// 用户id类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    pub fn build(mut self) -> DeleteRoleMemberRequest {
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_request
                .query_params
                .insert("user_id_type", user_id_type.clone());
        }
        self.request
    }
}

impl_executable_builder_owned!(
    DeleteRoleMemberRequestBuilder,
    AppRoleMemberService,
    DeleteRoleMemberRequest,
    BaseResponse<DeleteRoleMemberResponse>,
    delete
);

/// 删除协作者响应
#[derive(Debug, Deserialize)]
pub struct DeleteRoleMemberResponse {
    /// 删除的成员ID
    pub member_id: String,
    /// 是否删除成功
    pub deleted: bool,
}

impl ApiResponseTrait for DeleteRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除协作者
pub async fn delete_role_member(
    request: DeleteRoleMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeleteRoleMemberResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::DELETE;
    api_req.api_path = BITABLE_V1_ROLE_MEMBER_DELETE
        .replace("{app_token}", &request.app_token)
        .replace("{role_id}", &request.role_id)
        .replace("{member_id}", &request.member_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_role_member_request_builder() {
        let request = DeleteRoleMemberRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .role_id("rolxxxxxx")
            .member_id("ou_xxxxxx")
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.role_id, "rolxxxxxx");
        assert_eq!(request.member_id, "ou_xxxxxx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }
}

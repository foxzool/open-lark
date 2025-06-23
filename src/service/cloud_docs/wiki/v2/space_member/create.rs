use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 添加知识空间成员请求
#[derive(Debug, Serialize, Default)]
pub struct CreateSpaceMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id
    #[serde(skip)]
    space_id: String,
    /// 成员类型：user
    member_type: String,
    /// 成员id，根据member_type决定
    member_id: String,
    /// 成员权限角色：admin(管理员)、edit_member(协作者)、view_member(阅读者)
    role: String,
}

impl CreateSpaceMemberRequest {
    pub fn builder() -> CreateSpaceMemberRequestBuilder {
        CreateSpaceMemberRequestBuilder::default()
    }

    pub fn new(
        space_id: impl ToString,
        member_type: impl ToString,
        member_id: impl ToString,
        role: impl ToString,
    ) -> Self {
        Self {
            space_id: space_id.to_string(),
            member_type: member_type.to_string(),
            member_id: member_id.to_string(),
            role: role.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct CreateSpaceMemberRequestBuilder {
    request: CreateSpaceMemberRequest,
}

impl CreateSpaceMemberRequestBuilder {
    /// 知识空间id
    pub fn space_id(mut self, space_id: impl ToString) -> Self {
        self.request.space_id = space_id.to_string();
        self
    }

    /// 成员类型：user
    pub fn member_type(mut self, member_type: impl ToString) -> Self {
        self.request.member_type = member_type.to_string();
        self
    }

    /// 成员id，根据member_type决定
    pub fn member_id(mut self, member_id: impl ToString) -> Self {
        self.request.member_id = member_id.to_string();
        self
    }

    /// 成员权限角色：admin(管理员)、edit_member(协作者)、view_member(阅读者)
    pub fn role(mut self, role: impl ToString) -> Self {
        self.request.role = role.to_string();
        self
    }

    /// 设置为管理员
    pub fn as_admin(mut self) -> Self {
        self.request.role = "admin".to_string();
        self
    }

    /// 设置为协作者
    pub fn as_editor(mut self) -> Self {
        self.request.role = "edit_member".to_string();
        self
    }

    /// 设置为阅读者
    pub fn as_viewer(mut self) -> Self {
        self.request.role = "view_member".to_string();
        self
    }

    pub fn build(mut self) -> CreateSpaceMemberRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }

    /// 直接执行添加知识空间成员请求
    ///
    /// 这是一个便捷方法，相当于 `builder.build()` 然后调用 `service.create()`
    pub async fn execute(
        self,
        service: &crate::service::cloud_docs::wiki::v2::space_member::SpaceMemberService,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<CreateSpaceMemberResponse>>
    {
        service.create(self.build(), None).await
    }

    /// 直接执行添加知识空间成员请求（带选项）
    ///
    /// 这是一个便捷方法，相当于 `builder.build()` 然后调用 `service.create()`
    pub async fn execute_with_options(
        self,
        service: &crate::service::cloud_docs::wiki::v2::space_member::SpaceMemberService,
        option: crate::core::req_option::RequestOption,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<CreateSpaceMemberResponse>>
    {
        service.create(self.build(), Some(option)).await
    }
}

/// 添加的成员信息
#[derive(Debug, Deserialize)]
pub struct CreatedMember {
    /// 成员类型：user
    pub member_type: String,
    /// 成员id
    pub member_id: String,
    /// 成员权限角色
    pub role: String,
}

/// 添加知识空间成员响应
#[derive(Debug, Deserialize)]
pub struct CreateSpaceMemberResponse {
    /// 添加的成员信息
    pub member: CreatedMember,
}

impl ApiResponseTrait for CreateSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加知识空间成员
pub async fn create_space_member(
    request: CreateSpaceMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreateSpaceMemberResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = format!("/open-apis/wiki/v2/spaces/{}/members", request.space_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_space_member_request_builder() {
        let request = CreateSpaceMemberRequest::builder()
            .space_id("spcxxxxxx")
            .member_type("user")
            .member_id("ou_xxxxxx")
            .as_editor()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "ou_xxxxxx");
        assert_eq!(request.role, "edit_member");
    }
}

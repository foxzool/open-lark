//! 查询角色下某个成员的管理范围
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/get

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::{
        functional_role::member::models::GetMemberResponse, user::models::UserIdType,
    },
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 查询角色下某个成员的管理范围请求
pub struct GetRoleMemberRequest {
    config: Config,
    role_id: String,
    member_id: String,
    user_id_type: Option<UserIdType>,
}

impl GetRoleMemberRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            role_id: String::new(),
            member_id: String::new(),
            user_id_type: None,
        }
    }

    /// 角色 ID（路径参数）
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 成员 ID（路径参数）
    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.member_id = member_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/get
    pub async fn execute(self) -> SDKResult<GetMemberResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetMemberResponse> {
        validate_required!(self.role_id, "role_id 不能为空");
        validate_required!(self.member_id, "member_id 不能为空");

        // url: GET:/open-apis/contact/v3/functional_roles/:role_id/members/:member_id
        let mut req: ApiRequest<GetMemberResponse> = ApiRequest::get(format!(
            "{}/{}/members/{}",
            CONTACT_V3_FUNCTIONAL_ROLES, self.role_id, self.member_id
        ));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询角色下某个成员的管理范围")
    }
}

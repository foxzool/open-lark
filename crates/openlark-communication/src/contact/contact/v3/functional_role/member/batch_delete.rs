//! 删除角色下的成员
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/batch_delete

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::{
        functional_role::member::models::BatchDeleteMembersResponse, user::models::UserIdType,
    },
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 删除角色下的成员请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteMembersBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

/// 删除角色下的成员请求
pub struct BatchDeleteRoleMembersRequest {
    config: Config,
    role_id: String,
    user_id_type: Option<UserIdType>,
}

impl BatchDeleteRoleMembersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            role_id: String::new(),
            user_id_type: None,
        }
    }

    /// 角色 ID（路径参数）
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/batch_delete
    pub async fn execute(
        self,
        body: BatchDeleteMembersBody,
    ) -> SDKResult<BatchDeleteMembersResponse> {
        openlark_core::validate_required!(self.role_id, "role_id 不能为空");

        // url: PATCH:/open-apis/contact/v3/functional_roles/:role_id/members/batch_delete
        let mut req: ApiRequest<BatchDeleteMembersResponse> = ApiRequest::patch(format!(
            "{}/{}/members/batch_delete",
            CONTACT_V3_FUNCTIONAL_ROLES, self.role_id
        ))
        .body(serialize_params(&body, "删除角色下的成员")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除角色下的成员")
    }
}

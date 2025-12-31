//! 批量设置角色成员管理范围
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/scopes

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::{
        functional_role::member::models::PatchMembersScopesResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 批量设置角色成员管理范围请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMembersScopesBody {
    pub members: Vec<String>,
    pub departments: Vec<String>,
}

/// 批量设置角色成员管理范围请求
pub struct PatchRoleMembersScopesRequest {
    config: Config,
    role_id: String,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
}

impl PatchRoleMembersScopesRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            role_id: String::new(),
            user_id_type: None,
            department_id_type: None,
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

    /// 部门 ID 类型（查询参数，可选）
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/functional_role-member/scopes
    pub async fn execute(self, body: PatchMembersScopesBody) -> SDKResult<PatchMembersScopesResponse> {
        validate_required!(self.role_id, "role_id 不能为空");
        if body.members.is_empty() {
            return Err(openlark_core::error::validation_error(
                "members 不能为空".to_string(),
                "members 至少需要包含 1 个用户 ID".to_string(),
            ));
        }
        if body.departments.is_empty() {
            return Err(openlark_core::error::validation_error(
                "departments 不能为空".to_string(),
                "departments 至少需要包含 1 个部门 ID".to_string(),
            ));
        }

        // url: PATCH:/open-apis/contact/v3/functional_roles/:role_id/members/scopes
        let mut req: ApiRequest<PatchMembersScopesResponse> = ApiRequest::patch(format!(
            "{}/{}/members/scopes",
            CONTACT_V3_FUNCTIONAL_ROLES, self.role_id
        ))
        .body(serialize_params(&body, "批量设置角色成员管理范围")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "批量设置角色成员管理范围")
    }
}


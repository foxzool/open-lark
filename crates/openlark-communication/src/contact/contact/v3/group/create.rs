//! 创建用户组
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/group/create

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
        group::models::CreateGroupResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_GROUP,
};

/// 创建用户组请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

/// 创建用户组请求
pub struct CreateGroupRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
}

impl CreateGroupRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            department_id_type: None,
        }
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
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/group/create
    pub async fn execute(self, body: CreateGroupBody) -> SDKResult<CreateGroupResponse> {
        validate_required!(body.name, "name 不能为空");

        // url: POST:/open-apis/contact/v3/group
        let mut req: ApiRequest<CreateGroupResponse> =
            ApiRequest::post(CONTACT_V3_GROUP).body(serialize_params(&body, "创建用户组")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建用户组")
    }
}

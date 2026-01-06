//! 查询指定用户组
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/group/get

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::{
        group::models::GetGroupResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_GROUP,
};

/// 查询指定用户组请求
pub struct GetGroupRequest {
    config: Config,
    group_id: String,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
}

impl GetGroupRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            group_id: String::new(),
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 用户组 ID（路径参数）
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = group_id.into();
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
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/group/get
    pub async fn execute(self) -> SDKResult<GetGroupResponse> {
        validate_required!(self.group_id, "group_id 不能为空");

        // url: GET:/open-apis/contact/v3/group/:group_id
        let mut req: ApiRequest<GetGroupResponse> =
            ApiRequest::get(format!("{}/{}", CONTACT_V3_GROUP, self.group_id));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询指定用户组")
    }
}

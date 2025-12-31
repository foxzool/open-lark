//! 获取用户列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version//user/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::user::models::{DepartmentIdType, User, UserIdType},
    endpoints::CONTACT_V3_USERS,
};

/// 获取用户列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersResponse {
    #[serde(default)]
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub items: Vec<User>,
}

impl ApiResponseTrait for ListUsersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户列表请求
pub struct ListUsersRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
    department_id: Option<String>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl ListUsersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            department_id_type: None,
            department_id: None,
            page_token: None,
            page_size: None,
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

    /// 部门 ID（查询参数，可选）
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（查询参数，可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version//user/list
    pub async fn execute(self) -> SDKResult<ListUsersResponse> {
        // url: GET:/open-apis/contact/v3/users
        let mut req: ApiRequest<ListUsersResponse> = ApiRequest::get(CONTACT_V3_USERS);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(department_id) = self.department_id {
            req = req.query("department_id", department_id);
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取用户列表")
    }
}


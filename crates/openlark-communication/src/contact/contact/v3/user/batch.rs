//! 批量获取用户信息
//!
//! docPath: https://open.feishu.cn/document/contact-v3/user/batch

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::user::models::{DepartmentIdType, User, UserIdType},
    endpoints::CONTACT_V3_USERS_BATCH,
};

/// 批量获取用户信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetUsersResponse {
    #[serde(default)]
    pub items: Vec<User>,
}

impl ApiResponseTrait for BatchGetUsersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取用户信息请求
pub struct BatchGetUsersRequest {
    config: Config,
    user_ids: Vec<String>,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
}

impl BatchGetUsersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_ids: Vec::new(),
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 追加一个用户 ID（查询参数 user_ids，可多次传递）
    pub fn push_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_ids.push(user_id.into());
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
    /// docPath: https://open.feishu.cn/document/contact-v3/user/batch
    pub async fn execute(self) -> SDKResult<BatchGetUsersResponse> {
        if self.user_ids.is_empty() {
            return Err(error::validation_error(
                "user_ids 不能为空".to_string(),
                "请至少传入 1 个 user_ids（最多 50 个）".to_string(),
            ));
        }

        // url: GET:/open-apis/contact/v3/users/batch
        let mut req: ApiRequest<BatchGetUsersResponse> = ApiRequest::get(CONTACT_V3_USERS_BATCH);

        for user_id in self.user_ids {
            req = req.query("user_ids", user_id);
        }
        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "批量获取用户信息")
    }
}


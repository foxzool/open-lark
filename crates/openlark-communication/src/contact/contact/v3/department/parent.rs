//! 获取父部门信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/department/parent

use openlark_core::{api::ApiRequest, config::Config, error, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::{
        department::models::DepartmentListResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_DEPARTMENTS_PARENT,
};

/// 获取父部门信息请求
pub struct GetDepartmentParentsRequest {
    config: Config,
    department_id: Option<String>,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl GetDepartmentParentsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: None,
            user_id_type: None,
            department_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 部门 ID（查询参数，必填）
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
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

    /// 分页大小（查询参数，可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/department/parent
    pub async fn execute(self) -> SDKResult<DepartmentListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DepartmentListResponse> {
        let department_id = self.department_id.ok_or_else(|| {
            error::validation_error(
                "department_id 不能为空".to_string(),
                "department_id 为必填查询参数".to_string(),
            )
        })?;

        // url: GET:/open-apis/contact/v3/departments/parent
        let mut req: ApiRequest<DepartmentListResponse> =
            ApiRequest::get(CONTACT_V3_DEPARTMENTS_PARENT).query("department_id", department_id);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取父部门信息")
    }
}

//! 更新部门所有信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/department/update

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::{
        department::models::DepartmentResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_DEPARTMENTS,
};

/// 更新部门所有信息请求
pub struct UpdateDepartmentRequest {
    config: Config,
    department_id: String,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
}

impl UpdateDepartmentRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: String::new(),
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 部门 ID（路径参数）
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = department_id.into();
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/department/update
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<DepartmentResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DepartmentResponse> {

        validate_required!(self.department_id, "department_id 不能为空");

        // url: PUT:/open-apis/contact/v3/departments/:department_id
        let mut req: ApiRequest<DepartmentResponse> =
            ApiRequest::put(format!("{}/{}", CONTACT_V3_DEPARTMENTS, self.department_id))
                .body(serialize_params(&body, "更新部门所有信息")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        
        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新部门所有信息")
}
}

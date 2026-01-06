//! 更新部门 ID
//!
//! docPath: https://open.feishu.cn/document/contact-v3/department/update_department_id

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    contact::contact::v3::user::models::DepartmentIdType,
    endpoints::CONTACT_V3_DEPARTMENTS,
};

/// 更新部门 ID 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentIdBody {
    pub new_department_id: String,
}

/// 更新部门 ID 请求
pub struct UpdateDepartmentIdRequest {
    config: Config,
    department_id: String,
    department_id_type: Option<DepartmentIdType>,
}

impl UpdateDepartmentIdRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: String::new(),
            department_id_type: None,
        }
    }

    /// 部门 ID（路径参数）
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = department_id.into();
        self
    }

    /// 部门 ID 类型（查询参数，可选）
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/contact-v3/department/update_department_id
    pub async fn execute(self, body: UpdateDepartmentIdBody) -> SDKResult<EmptyData> {
        validate_required!(self.department_id, "department_id 不能为空");
        validate_required!(body.new_department_id, "new_department_id 不能为空");

        // url: PATCH:/open-apis/contact/v3/departments/:department_id/update_department_id
        let mut req: ApiRequest<EmptyData> = ApiRequest::patch(format!(
            "{}/{}/update_department_id",
            CONTACT_V3_DEPARTMENTS, self.department_id
        ))
        .body(serialize_params(&body, "更新部门 ID")?);

        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新部门 ID")
    }
}

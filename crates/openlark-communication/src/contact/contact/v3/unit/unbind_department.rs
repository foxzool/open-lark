//! 解除部门与单位的绑定关系
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/unit/unbind_department

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
    endpoints::CONTACT_V3_UNIT_UNBIND_DEPARTMENT,
};

/// 解除部门与单位的绑定关系请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindDepartmentBody {
    pub unit_id: String,
    pub department_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<DepartmentIdType>,
}

/// 解除部门与单位的绑定关系请求
pub struct UnbindDepartmentRequest {
    config: Config,
}

impl UnbindDepartmentRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/unit/unbind_department
    pub async fn execute(self, body: UnbindDepartmentBody) -> SDKResult<EmptyData> {
        validate_required!(body.unit_id, "unit_id 不能为空");
        validate_required!(body.department_id, "department_id 不能为空");

        // url: POST:/open-apis/contact/v3/unit/unbind_department
        let req: ApiRequest<EmptyData> = ApiRequest::post(CONTACT_V3_UNIT_UNBIND_DEPARTMENT)
            .body(serialize_params(&body, "解除部门与单位的绑定关系")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "解除部门与单位的绑定关系")
    }
}

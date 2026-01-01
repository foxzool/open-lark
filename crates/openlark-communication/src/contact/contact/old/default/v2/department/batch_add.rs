//! 批量新增部门
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version//import-api/batch-add-departments

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::CONTACT_V2_DEPARTMENT_BATCH_ADD,
};

/// 批量新增部门请求
pub struct BatchAddDepartmentsRequest {
    config: Config,
}

impl BatchAddDepartmentsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version//import-api/batch-add-departments
    pub async fn execute(self, params: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/contact/v2/department/batch_add
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(CONTACT_V2_DEPARTMENT_BATCH_ADD)
            .body(serialize_params(&params, "批量新增部门")?);
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "批量新增部门")
    }
}

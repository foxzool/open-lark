//! 删除人员类型
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/employee_type_enum/delete

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    endpoints::CONTACT_V3_EMPLOYEE_TYPE_ENUMS,
};

/// 删除人员类型请求
pub struct DeleteEmployeeTypeEnumRequest {
    config: Config,
    enum_id: String,
}

impl DeleteEmployeeTypeEnumRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            enum_id: String::new(),
        }
    }

    /// 枚举 ID（路径参数）
    pub fn enum_id(mut self, enum_id: impl Into<String>) -> Self {
        self.enum_id = enum_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/employee_type_enum/delete
    pub async fn execute(self) -> SDKResult<EmptyData> {
        validate_required!(self.enum_id, "enum_id 不能为空");

        // url: DELETE:/open-apis/contact/v3/employee_type_enums/:enum_id
        let req: ApiRequest<EmptyData> = ApiRequest::delete(format!(
            "{}/{}",
            CONTACT_V3_EMPLOYEE_TYPE_ENUMS, self.enum_id
        ));
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除人员类型")
    }
}

//! 查询人员类型
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/employee_type_enum/list

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::employee_type_enum::models::ListEmployeeTypeEnumsResponse,
    endpoints::CONTACT_V3_EMPLOYEE_TYPE_ENUMS,
};

/// 查询人员类型请求
pub struct ListEmployeeTypeEnumsRequest {
    config: Config,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl ListEmployeeTypeEnumsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_token: None,
            page_size: None,
        }
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（查询参数，可选，默认 20，最大 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/employee_type_enum/list
    pub async fn execute(self) -> SDKResult<ListEmployeeTypeEnumsResponse> {
        // url: GET:/open-apis/contact/v3/employee_type_enums
        let mut req: ApiRequest<ListEmployeeTypeEnumsResponse> =
            ApiRequest::get(CONTACT_V3_EMPLOYEE_TYPE_ENUMS);

        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询人员类型")
    }
}

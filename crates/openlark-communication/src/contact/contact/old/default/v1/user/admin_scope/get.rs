//! 获取应用管理员管理范围
//!
//! docPath: https://open.feishu.cn/document/server-docs/application-v6/admin/obtain-an-app-admin%E2%80%99s-management-permissions

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CONTACT_V1_USER_ADMIN_SCOPE_GET};

/// 获取应用管理员管理范围
pub struct GetAdminScopeRequest {
    config: Config,
}

impl GetAdminScopeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/application-v6/admin/obtain-an-app-admin%E2%80%99s-management-permissions
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/contact/v1/user/admin_scope/get
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(CONTACT_V1_USER_ADMIN_SCOPE_GET);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取应用管理员管理范围")
    }
}

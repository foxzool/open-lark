//! 查询应用管理员列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/application-v6/admin/query-app-administrator-list

use std::collections::HashMap;

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::extract_response_data;

/// 查询应用管理员列表请求
pub struct ListAppAdminUsersRequest {
    config: Config,
    query: HashMap<String, String>,
}

impl ListAppAdminUsersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: HashMap::new(),
        }
    }

    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/application-v6/admin/query-app-administrator-list
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/user/v4/app_admin_user/list
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/user/v4/app_admin_user/list");
        for (k, v) in self.query {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询应用管理员列表")
    }
}

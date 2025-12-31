//! 搜索用户
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/user/search-users

use std::collections::HashMap;

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::extract_response_data;

/// 搜索用户请求（GET /open-apis/search/v1/user）
pub struct SearchUsersRequest {
    config: Config,
    query: HashMap<String, String>,
}

impl SearchUsersRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/user/search-users
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/search/v1/user
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get("/open-apis/search/v1/user");
        for (k, v) in self.query {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "搜索用户")
    }
}

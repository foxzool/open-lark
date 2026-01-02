//! 获取角色列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version//user/obtain-a-role-list

use std::collections::HashMap;

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CONTACT_V2_ROLE_LIST};

/// 获取角色列表请求
pub struct ListRolesRequest {
    config: Config,
    query: HashMap<String, String>,
}

impl ListRolesRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: HashMap::new(),
        }
    }

    /// 添加查询参数（兼容旧接口，字段以文档为准）
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version//user/obtain-a-role-list
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/contact/v2/role/list
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(CONTACT_V2_ROLE_LIST);
        for (k, v) in self.query {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取角色列表")
    }
}

//! 查询批量任务执行状态
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version//import-api/query-the-execution-status-of-a-batch-task

use std::collections::HashMap;

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::extract_response_data;

/// 查询批量任务执行状态请求
pub struct GetBatchTaskRequest {
    config: Config,
    query: HashMap<String, String>,
}

impl GetBatchTaskRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: HashMap::new(),
        }
    }

    /// 添加查询参数（例如 task_id）
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version//import-api/query-the-execution-status-of-a-batch-task
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/contact/v2/task/get
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/contact/v2/task/get");
        for (k, v) in self.query {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询批量任务执行状态")
    }
}

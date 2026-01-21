//! 用户搜索 API
//!
//! 提供用户搜索相关功能

use crate::AnalyticsConfig;
use std::sync::Arc;

/// 用户搜索 API
#[derive(Debug, Clone)]
pub struct UserSearchApi {
    config: Arc<AnalyticsConfig>,
}

impl UserSearchApi {
    pub fn new(config: Arc<AnalyticsConfig>) -> Self {
        Self { config }
    }

    /// 搜索用户
    pub fn search(&self) -> SearchUserRequest {
        SearchUserRequest::new(self.config.clone(), self.client.clone())
    }
}

/// 搜索用户请求
pub struct SearchUserRequest {
    config: AnalyticsConfig,
    client: LarkClient,
    query: Option<String>,
    page_size: Option<u32>,
}

impl SearchUserRequest {
    fn new(config: AnalyticsConfig, client: LarkClient) -> Self {
        Self {
            config,
            client,
            query: None,
            page_size: None,
        }
    }

    /// 设置查询词
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> Result<serde_json::Value> {
        // TODO: 实现实际的 API 调用
        Ok(serde_json::json!({"items": []}))
    }
}

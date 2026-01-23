//! 搜索服务 V1 API
//!
//! 提供搜索服务 V1 版本的 API 访问

use crate::AnalyticsConfig;
use std::sync::Arc;

/// 搜索服务 V1 API
#[derive(Debug, Clone)]
pub struct SearchV1 {
    /// 客户端配置
    config: Arc<AnalyticsConfig>,
}

impl SearchV1 {
    /// 创建新的搜索服务 V1 实例
    pub fn new(config: Arc<AnalyticsConfig>) -> Self {
        Self { config }
    }

    /// 查询搜索
    pub fn query(&self) -> super::v1::query::QueryApi {
        super::v1::query::QueryApi::new(self.config.clone())
    }

    /// 用户搜索
    pub fn user(&self) -> super::v1::user::UserSearchApi {
        super::v1::user::UserSearchApi::new(self.config.clone())
    }
}

pub mod query;
pub mod user;

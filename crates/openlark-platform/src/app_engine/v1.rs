//! 应用引擎 V1 API
//!
//! 提供应用引擎 V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

/// 应用引擎 V1 API
#[derive(Debug, Clone)]
pub struct AppEngineV1 {
    /// 客户端配置
    config: Arc<PlatformConfig>,
}

impl AppEngineV1 {
    /// 创建新的应用引擎 V1 实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// 应用管理
    pub fn apps(&self) -> super::v1::apps::AppsApi {
        super::v1::apps::AppsApi::new(self.config.clone())
    }

    /// 租户管理
    pub fn tenants(&self) -> super::v1::tenants::TenantsApi {
        super::v1::tenants::TenantsApi::new(self.config.clone())
    }

    /// 应用市场
    pub fn market(&self) -> super::v1::market::MarketApi {
        super::v1::market::MarketApi::new(self.config.clone())
    }
}

pub mod apps;
pub mod tenants;
pub mod market;

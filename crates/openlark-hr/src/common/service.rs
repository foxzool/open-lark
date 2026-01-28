//! HR 服务主模块
//!
//! 提供统一的配置封装，供各 project-version-resource API 复用。
//! HTTP 请求统一通过 openlark_core::Transport 处理。

use std::sync::Arc;

use openlark_core::config::Config;

/// HR 服务入口
#[derive(Clone)]
pub struct HrService {
    config: Arc<Config>,
}

impl HrService {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub fn from_config(config: Config) -> Self {
        Self::new(config)
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

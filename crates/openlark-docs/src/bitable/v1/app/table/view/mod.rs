//! 视图管理服务模块
//!
//! 提供多维表格视图的创建、更新、删除和查询功能。

use openlark_core::{
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    SDKResult,
    core::{
        BaseResponse,
        ResponseFormat,
    },
};
use serde::{Deserialize, Serialize};

// 导入子模块
pub mod patch;

// 重新导出主要类型
pub use patch::{
    PatchViewRequest, PatchViewRequestBuilder, PatchViewResponse
};

/// 视图服务
#[derive(Clone)]
pub struct AppTableViewService {
    config: Config,
}

impl AppTableViewService {
    /// 创建视图服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// Type alias for compatibility
pub type ServiceType = AppTableViewService;
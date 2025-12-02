//! 视图管理服务模块
//!
//! 提供多维表格视图的创建、更新、删除和查询功能。

use openlark_core::{
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
    api::{
        BaseResponse,
        ResponseFormat, HttpMethod,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

// 导入子模块
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod patch;

// 重新导出所有模块内容
pub use create::*;
pub use delete::*;
pub use get::*;
pub use list::*;
pub use patch::*;

/// 视图服务
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
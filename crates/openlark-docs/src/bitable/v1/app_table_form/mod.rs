//! 表单管理服务模块
//!
//! 提供多维表格表单的创建、更新、删除和查询功能。

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
        api::ApiResponseTrait,
    },
};
use serde::{Deserialize, Serialize};

// 导入子模块
pub mod list;
pub mod create;
pub mod update;
pub mod delete;

// 重新导出主要类型
pub use list::{
    ListFormRequest, ListFormRequestBuilder, ListFormResponse
};
pub use create::{
    CreateFormRequest, CreateFormRequestBuilder, CreateFormResponse
};
pub use update::{
    UpdateFormRequest, UpdateFormRequestBuilder, UpdateFormResponse
};
pub use delete::{
    DeleteFormRequest, DeleteFormRequestBuilder, DeleteFormResponse
};

/// 表单服务
#[derive(Clone)]
pub struct AppTableFormService {
    config: Config,
}

impl AppTableFormService {
    /// 创建表单服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// Type alias for compatibility
pub type ServiceType = AppTableFormService;
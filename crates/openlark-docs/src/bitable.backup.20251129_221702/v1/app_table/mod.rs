//! 数据表管理服务模块
//!
//! 提供多维表格数据表的创建、删除、查询等基础操作功能。

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

use openlark_core::{
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    service::bitable::v1::{TableData, TableField},
    SDKResult,
};
use serde::{Deserialize, Serialize};

// 导入子模块 - 暂时只包含稳定的模块
pub mod create;
pub mod delete;
pub mod list;
// 重新启用已修复的模块
pub mod batch_create;
pub mod batch_delete;
pub mod patch;

// 重新导出主要类型
pub use create::{
    CreateTableRequest, CreateTableRequestBuilder, CreateTableResponse,
    TableData, TableField
};
pub use delete::DeleteTableRequest;
pub use list::{
    ListTablesRequest, ListTablesRequestBuilder, ListTablesResponse, TableInfo
};
// 重新启用已修复的模块导出
pub use batch_create::{
    BatchCreateTableRequest, BatchCreateTableRequestBuilder, BatchCreateTableResponse,
    BatchCreateTableResult
};
pub use batch_delete::{
    BatchDeleteTableRequest, BatchDeleteTableRequestBuilder, BatchDeleteTableResponse,
    BatchDeleteTableResult
};
pub use patch::PatchTableRequest;

/// 数据表服务
#[derive(Clone)]
pub struct AppTableService {
    config: Config,
}

impl AppTableService {
    /// 创建数据表服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// Type alias for compatibility
pub type ServiceType = AppTableService;
// 导入优化模块
use super::{BatchCommonParams, BatchOperationResult, AppToken, TableId};

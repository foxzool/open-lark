//! 记录管理服务模块
//!
//! 提供多维表格记录的创建、更新、删除、查询等操作功能。

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
    service::bitable::v1::Record,
    SDKResult,
};
use serde::{Deserialize, Serialize};

// 导入子模块
pub mod batch_create;
pub mod batch_delete;
pub mod batch_get;
pub mod batch_update;
pub mod create;
pub mod delete;
pub mod search;
pub mod update;

// 重新导出主要类型
pub use create::CreateRecordRequest;
pub use batch_create::{
    BatchCreateRecordRequest, BatchCreateRecordRequestBuilder, BatchCreateRecordResponse
};
pub use batch_delete::{
    BatchDeleteRecordRequest, BatchDeleteRecordRequestBuilder, BatchDeleteRecordResponse
};
pub use batch_get::{
    BatchGetRecordRequest, BatchGetRecordRequestBuilder, BatchGetRecordResponse
};
pub use batch_update::{
    BatchUpdateRecordRequest, BatchUpdateRecordRequestBuilder, BatchUpdateRecordResponse,
    BatchUpdateRecordResult
};
pub use delete::DeleteRecordRequest;
pub use search::SearchRecordRequest;
pub use update::UpdateRecordRequest;

/// 记录服务
#[derive(Clone)]
pub struct AppTableRecordService {
    config: Config,
}

impl AppTableRecordService {
    /// 创建记录服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// Type alias for compatibility
pub type ServiceType = AppTableRecordService;
// 导入优化模块
use super::{BatchCommonParams, BatchOperationResult, AppToken, TableId, RecordId};

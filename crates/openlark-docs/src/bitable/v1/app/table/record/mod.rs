//! 记录管理服务模块
//!
//! 提供多维表格记录的创建、更新、删除、查询等操作功能。

use openlark_core::{
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
    api::ResponseFormat,
};
use serde::{Deserialize, Serialize};

// 导入子模块
pub mod batch_create;
pub mod batch_delete;
pub mod batch_get;
pub mod batch_update;
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod search;
pub mod update;

// 重新导出主要类型
pub use create::CreateRecordRequest;
pub use batch_create::{
    BatchCreateRecordRequest, BatchCreateRecordRequestBuilder, BatchCreateRecordResponse, Record
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
pub use get::{
    GetRecordRequest, GetRecordRequestBuilder, GetRecordResponse, RecordInfo,
    CreatorInfo, UpdaterInfo
};
pub use list::{
    ListRecordRequest, ListRecordRequestBuilder, ListRecordResponse, SortCondition,
    FilterInfo, FilterCondition
};
pub use search::SearchRecordRequest;
pub use update::UpdateRecordRequest;

/// 记录服务
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

    /// 批量获取记录
    pub async fn batch_get(
        &self,
        request: BatchGetRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<openlark_core::api::Response<BatchGetRecordResponse>> {
        batch_get::batch_get_record(request, &self.config, option).await
    }
}

// Type alias for compatibility
pub type ServiceType = AppTableRecordService;
// 导入优化模块
use crate::common::batch::{BatchCommonParams, BatchOperationResult};
use crate::common::types::{AppToken, TableId, RecordId};

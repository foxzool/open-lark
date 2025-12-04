//! 数据表管理服务模块
//!
//! 提供多维表格数据表的创建、删除、查询等基础操作功能。

use openlark_core::config::Config;

// 导入所有子模块
pub mod batch_create;
pub mod batch_delete;
pub mod create;
pub mod delete;
pub mod field;
pub mod form;
pub mod list;
pub mod patch;
pub mod record;
pub mod view;

// 导出所有模块内容，但避免命名冲突
pub use batch_create::*;
pub use batch_delete::*;
pub use create::*;
pub use delete::*;

// 字段相关 - 选择性导入以避免命名冲突
pub use field::{
    CreateFieldRequest, CreateFieldRequestBuilder, CreateFieldResponse,
    DeleteFieldRequest, DeleteFieldRequestBuilder, DeleteFieldResponse,
    ListFieldRequest, ListFieldRequestBuilder, ListFieldResponse,
    UpdateFieldRequest, UpdateFieldRequestBuilder, UpdateFieldResponse,
    Field, FieldType
};

// 表单相关 - 选择性导入以避免命名冲突
pub use form::{
    FormService, GetFormRequest, GetFormRequestBuilder, GetFormResponse,
    ListFormQuestionRequest, ListFormQuestionRequestBuilder, ListFormQuestionResponse,
    PatchFormQuestionRequest, PatchFormQuestionRequestBuilder, PatchFormQuestionResponse,
    FormFieldQuestion
};

pub use list::*;
pub use patch::*;

// 记录相关 - 选择性导入以避免命名冲突
pub use record::{
    BatchCreateRecordRequest, BatchCreateRecordRequestBuilder, BatchCreateRecordResponse,
    BatchDeleteRecordRequest, BatchDeleteRecordRequestBuilder, BatchDeleteRecordResponse,
    BatchGetRecordRequest, BatchGetRecordRequestBuilder, BatchGetRecordResponse,
    BatchUpdateRecordRequest, BatchUpdateRecordRequestBuilder, BatchUpdateRecordResponse,
    CreateRecordRequest, CreateRecordRequestBuilder, CreateRecordResponse,
    DeleteRecordRequest, DeleteRecordRequestBuilder, DeleteRecordResponse,
    GetRecordRequest, GetRecordRequestBuilder, GetRecordResponse,
    ListRecordRequest, ListRecordRequestBuilder, ListRecordResponse,
    SearchRecordRequest, SearchRecordRequestBuilder, SearchRecordResponse,
    AppTableRecordService, Record
};

// 视图相关 - 选择性导入以避免命名冲突
pub use view::{
    CreateViewRequest, CreateViewRequestBuilder, CreateViewResponse, CreateViewData,
    DeleteViewRequest, DeleteViewRequestBuilder, DeleteViewResponse,
    GetViewRequest, GetViewRequestBuilder, GetViewResponse,
    ListViewsRequest, ListViewsRequestBuilder, ListViewsResponse,
    PatchViewRequest, PatchViewRequestBuilder, PatchViewResponse, PatchViewData,
    AppTableViewService
};

/// 数据表服务
pub struct TableService {
    config: Config,
}

impl TableService {
    /// 创建数据表服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 创建批量新增数据表请求构建器
    pub fn batch_create(&self) -> BatchCreateTableRequestBuilder {
        BatchCreateTableRequestBuilder::new(self.config.clone())
    }

    /// 创建批量删除数据表请求构建器
    pub fn batch_delete(&self) -> BatchDeleteTableRequestBuilder {
        BatchDeleteTableRequestBuilder::new(self.config.clone())
    }
}

// Type alias for compatibility
pub type ServiceType = TableService;

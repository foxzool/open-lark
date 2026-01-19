/// 数据表管理服务模块
///
/// 提供多维表格数据表的创建、删除、查询等基础操作功能。
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
    CreateFieldRequest, CreateFieldResponse, DeleteFieldRequest, DeleteFieldResponse, Field,
    FieldType, ListFieldRequest, ListFieldResponse, UpdateFieldRequest, UpdateFieldResponse,
};

// 表单相关 - 选择性导入以避免命名冲突
pub use form::{FormFieldQuestion, FormService, GetFormRequest, GetFormResponse, PatchFormRequest, PatchFormResponse};

pub use list::*;
pub use patch::*;

// 记录相关 - 选择性导入以避免命名冲突
pub use record::{
    AppTableRecordService, BatchCreateRecordRequest, BatchCreateRecordResponse,
    BatchDeleteRecordRequest, BatchDeleteRecordResponse, BatchGetRecordRequest,
    BatchGetRecordResponse, BatchUpdateRecordRequest, BatchUpdateRecordResponse,
    CreateRecordRequest, CreateRecordResponse, DeleteRecordRequest, DeleteRecordResponse,
    GetRecordRequest, GetRecordResponse, ListRecordRequest, ListRecordResponse, Record,
    SearchRecordRequest, SearchRecordResponse,
};

// 视图相关 - 选择性导入以避免命名冲突
pub use view::{
    AppTableViewService, CreateViewData, CreateViewRequest, CreateViewResponse, DeleteViewRequest,
    DeleteViewResponse, GetViewRequest, GetViewResponse, ListViewsRequest, ListViewsResponse,
    PatchViewData, PatchViewRequest, PatchViewResponse,
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
}

// Type alias for compatibility
pub type ServiceType = TableService;

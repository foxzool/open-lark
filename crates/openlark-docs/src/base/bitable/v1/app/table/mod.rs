/// 表格管理模块
///
/// 提供多维表格数据表的 CRUD 操作，包括：
/// - 单个/批量创建数据表
/// - 单个/批量删除数据表
/// - 更新数据表
/// - 列出数据表
/// - 字段管理（field 子模块）
/// - 记录管理（record 子模块）
/// - 表单管理（form 子模块）
/// - 视图管理（view 子模块）

pub mod list;
pub mod batch_create;
pub mod create;
pub mod patch;
pub mod delete;
pub mod batch_delete;
pub mod field;
pub mod record;
pub mod form;
pub mod view;

// 显式导出 - 避免使用 glob reexport
pub use list::{
    ListTablesRequest,
    ListTablesResponse,
    TableInfo,
};

pub use batch_create::{BatchCreateTableRequest, BatchCreateTableResponse};

pub use create::{
    CreateTableRequest,
    CreateTableResponse,
    TableData,
    TableField,
    FieldDescription,
};

pub use delete::{DeleteTableRequest, DeleteTableResponse};

pub use patch::{PatchTableRequest, PatchTableResponse};

pub use batch_delete::{BatchDeleteTableRequest, BatchDeleteTableResponse};

// 显式导出子模块类型
pub use field::{
    CreateFieldRequest,
    DeleteFieldRequest,
    Field,
    FieldProperty,
    FieldType,
    ListFieldRequest,
    ListFieldResponse,
    UpdateFieldRequest,
    UpdateFieldResponse,
};

pub use record::{
    BatchCreateRecordRequest,
    BatchCreateRecordResponse,
    BatchDeleteRecordRequest,
    BatchDeleteRecordResponse,
    BatchGetRecordRequest,
    BatchGetRecordResponse,
    BatchUpdateRecordRequest,
    BatchUpdateRecordResponse,
    CreateRecordRequest,
    CreateRecordResponse,
    DeleteRecordRequest,
    DeleteRecordResponse,
    GetRecordRequest,
    GetRecordResponse,
    ListRecordRequest,
    ListRecordResponse,
    Record,
    SearchRecordRequest,
    SearchRecordResponse,
    UpdateRecordRequest,
    UpdateRecordResponse,
};

pub use form::{
    Form,
    GetFormRequest,
    GetFormResponse,
    PatchFormRequest,
    PatchFormResponse,
};

pub use view::{
    CreateViewData,
    CreateViewRequest,
    CreateViewResponse,
    ListViewsRequest,
    ListViewsResponse,
    PatchViewData,
    PatchViewRequest,
    PatchViewResponse,
    View,
};

pub mod batch_create;
pub mod batch_delete;
pub mod create;
pub mod delete;
pub mod field;
pub mod form;
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
pub mod patch;
pub mod record;
pub mod view;

// 显式导出 - 避免使用 glob reexport
pub use list::{ListTablesRequest, ListTablesResponse, TableInfo};

pub use batch_create::{BatchCreateTableRequest, BatchCreateTableResponse};

pub use create::{
    CreateTableRequest, CreateTableResponse, FieldDescription, TableData, TableField,
};

pub use delete::{DeleteTableRequest, DeleteTableResponse};

pub use patch::{PatchTableRequest, PatchTableResponse};

pub use batch_delete::{BatchDeleteTableRequest, BatchDeleteTableResponse};

// 显式导出子模块类型
pub use field::{
    CreateFieldRequest, CreateFieldResponse, DeleteFieldRequest, DeleteFieldResponse, Field,
    FieldProperty, FieldType, ListFieldRequest, ListFieldResponse, UpdateFieldRequest,
    UpdateFieldResponse,
};

pub use record::{
    BatchCreateRecordRequest, BatchCreateRecordResponse, BatchDeleteRecordRequest,
    BatchDeleteRecordResponse, BatchGetRecordRequest, BatchGetRecordResponse,
    BatchUpdateRecordRequest, BatchUpdateRecordResponse, CreateRecordItem, CreateRecordRequest,
    CreateRecordResponse, DeleteRecordRequest, DeleteRecordResponse, DeletedRecord,
    FilterCondition, FilterInfo, GetRecordRequest, GetRecordResponse, ListRecordRequest,
    ListRecordResponse, Person, Record, SearchRecordRequest, SearchRecordRequestBody,
    SearchRecordResponse, SortCondition, UpdateRecordItem, UpdateRecordRequest,
    UpdateRecordResponse,
};

pub use form::{
    Form, FormFieldQuestion, GetFormRequest, GetFormResponse, ListFormFieldQuestionRequest,
    ListFormFieldQuestionResponse, PatchFormFieldQuestionBuilder, PatchFormFieldQuestionRequest,
    PatchFormFieldQuestionResponse, PatchFormFieldRequest, PatchFormRequest, PatchFormResponse,
    PatchedFormFieldQuestion,
};

pub use view::{
    CreateViewData, CreateViewRequest, CreateViewResponse, DeleteViewRequest, DeleteViewResponse,
    GetViewRequest, GetViewResponse, ListViewsRequest, ListViewsResponse, PatchViewData,
    PatchViewRequest, PatchViewResponse, View,
};

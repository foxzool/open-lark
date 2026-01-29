/// 记录管理模块
//!
//! 提供多维表格记录的 CRUD 操作，包括批量操作。

pub mod batch_get;
pub mod list;
pub mod batch_create;
pub mod models;
pub mod create;
pub mod delete;
pub mod update;
pub mod batch_delete;
pub mod batch_update;
pub mod get;
pub mod search;

// 显式导出 - 避免使用 glob reexport
pub use batch_create::{BatchCreateRecordRequest, BatchCreateRecordResponse};

pub use batch_delete::{BatchDeleteRecordRequest, BatchDeleteRecordResponse};

pub use batch_get::{BatchGetRecordRequest, BatchGetRecordResponse};

pub use batch_update::{BatchUpdateRecordRequest, BatchUpdateRecordResponse};

pub use create::{CreateRecordRequest, CreateRecordResponse};

pub use delete::{DeleteRecordRequest, DeleteRecordResponse};

pub use get::{GetRecordRequest, GetRecordResponse};

pub use list::{ListRecordRequest, ListRecordResponse};

pub use models::{Person, Record, RecordFieldValue};

pub use search::{SearchRecordRequest, SearchRecordResponse};

pub use update::{UpdateRecordRequest, UpdateRecordResponse};

/// 表单管理模块
//!
//! 提供多维表格表单的获取和更新操作。

pub mod models;
pub mod patch;
pub mod get;

// 显式导出 - 避免使用 glob reexport
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

pub use get::{GetFormRequest, GetFormResponse};

pub use models::Form;

pub use patch::{PatchFormRequest, PatchFormResponse};

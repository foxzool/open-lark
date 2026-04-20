/// field 子模块。
pub mod field;
/// get 子模块。
pub mod get;
/// 表单管理模块
///
/// 提供多维表格表单的获取和更新操作。
pub mod models;
/// patch 子模块。
pub mod patch;
/// upgrade 子模块。
pub mod upgrade;

// 显式导出 - 避免使用 glob reexport
/// 重新导出相关类型。
pub use field::{
    FormFieldQuestion, ListFormFieldQuestionRequest, ListFormFieldQuestionResponse,
    PatchFormFieldQuestionBuilder, PatchFormFieldQuestionRequest, PatchFormFieldQuestionResponse,
    PatchFormFieldRequest, PatchedFormFieldQuestion,
};

/// 重新导出相关类型。
pub use get::{GetFormRequest, GetFormResponse};

/// 重新导出相关类型。
pub use models::Form;

/// 重新导出相关类型。
pub use patch::{PatchFormRequest, PatchFormResponse};

/// 重新导出相关类型。
pub use upgrade::{FormDisplayMode, UpgradeFormRequest, UpgradeFormResponse, UpgradedForm};

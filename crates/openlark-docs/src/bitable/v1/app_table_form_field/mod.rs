//! 表单问题管理模块
//!
//! 提供多维表格表单问题相关的API接口，包括：
//! - 更新表单问题 (Patch)
//! - 删除表单问题 (Delete)
//!
//! # 使用示例
//!
//! ```rust,no_run
//! use openlark_client::LarkClient;
//!
//! let client = LarkClient::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .build()?;
//!
//! // 更新表单问题
//! let patch_response = client.docs.base.bitable.v1.app_table_form_field
//!     .patch_form_question_builder()
//!     .app_token("your_app_token")
//!     .form_id("your_form_id")
//!     .question_id("your_question_id")
//!     .title("更新后的标题")
//!     .required(true)
//!     .build()
//!     .await?;
//!
//! // 删除表单问题
//! let delete_response = client.docs.base.bitable.v1.app_table_form_field
//!     .delete_form_question_builder()
//!     .app_token("your_app_token")
//!     .form_id("your_form_id")
//!     .question_id("your_question_id")
//!     .build()
//!     .await?;
//! ```

pub mod patch;
pub mod delete;

pub use patch::*;
pub use delete::*;
//! Open-Lark People Module
//!
//! 飞书人员管理服务模块，提供完整的用户和组织架构管理功能。
//!
//! ## 主要功能
//!
//! - **联系人管理**: 用户查询、联系人管理、联系人搜索
//! - **通讯录管理**: 部门架构、员工信息、组织关系
//! - **个人信息设置**: 用户偏好、个人资料、隐私设置
//!
//! ## 使用示例
//!
//! ```rust
//! use openlark_people::endpoints::*;
//!
//! // 使用端点常量
//! let contacts_endpoint = CONTACT_V3_USERS;
//! let directory_endpoint = DIRECTORY_V1_DEPARTMENTS;
//! let settings_endpoint = PERSONAL_SETTINGS_V1_SYSTEM_STATUS;
//! println!("联系人端点: {}", contacts_endpoint);
//! println!("通讯录端点: {}", directory_endpoint);
//! println!("个人设置端点: {}", settings_endpoint);
//! ```
//!
//! ## 端点组织
//!
//! - `contact`: 联系人管理端点
//! - `directory`: 通讯录管理端点
//! - `personal_settings`: 个人设置端点

#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

// 导入端点模块
pub mod endpoints;

// 重新导出端点常量，方便外部使用
pub use endpoints::*;

/// Re-exports from openlark-core for convenience.
pub mod prelude {
    pub use openlark_core::SDKResult;
}

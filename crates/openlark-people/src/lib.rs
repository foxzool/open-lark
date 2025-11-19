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
//! use openlark_people::*;
//!
//! // 使用端点常量
//! let contacts_endpoint = CONTACT_V3_USERS;
//! let directory_endpoint = DIRECTORY_V1_DEPARTMENTS;
//! println!("联系人端点: {}", contacts_endpoint);
//! println!("通讯录端点: {}", directory_endpoint);
//! ```
//!
//! ## 模块组织
//!
//! - **endpoints**: API端点常量定义
//! - **models**: 数据模型和类型定义 (计划中)
//! - **v3**: V3 API服务占位符 (计划中)
//! - **prelude**: 常用类型和特征导入

#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

// 导入核心模块
pub mod endpoints;
pub mod models;
pub mod v3;

// 重新导出端点常量和主要类型，方便外部使用
pub use endpoints::*;
// pub use models::*;  // 暂时注释，models 模块还没有实现
pub use v3::*;

/// Re-exports from openlark-core for convenience.
pub mod prelude {
    pub use openlark_core::{config::Config, SDKResult};
}

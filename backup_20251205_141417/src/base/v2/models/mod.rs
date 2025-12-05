//! Base V2 数据模型模块

pub mod role;

// 选择性导出以避免命名冲突
pub use role::{
    BaseRole, CreateRoleRequest, UpdateRoleRequest, ListRolesRequest, ListRolesResponse
};

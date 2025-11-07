//! 通讯录 API v3版本
//!
//! 提供飞书开放平台通讯录服务的v3版本API实现，包括：
//! - 用户管理服务
//! - 部门管理服务
//! - 用户组管理服务
//! - 自定义字段服务
//! - 各种业务实体管理服务

use serde::{Deserialize, Serialize};

// 事件处理相关模块 - 暂时禁用以修复语法错误
// pub mod p2_contact_user_created_v3;
// pub mod p2_contact_user_deleted_v3;
// pub mod p2_contact_user_updated_v3;
// pub mod p2_contact_department_created_v3;
// pub mod p2_contact_department_deleted_v3;
// pub mod p2_contact_department_updated_v3;

// 服务实现模块 - 只启用已修复且语法正确的服务
pub mod custom_attr;
pub mod department;
pub mod employee_type_enum;
pub mod functional_role;
pub mod functional_role_member;
pub mod group;
pub mod group_member;
pub mod job_family;
pub mod job_level;
pub mod job_title;
pub mod scope;
pub mod unit;
pub mod user;
pub mod work_city;

// 以下模块存在严重语法错误，需要完全重写后才能启用：
// - department.rs: ✅ 已修复并启用
// - group.rs: ✅ 已修复并启用
// - group_member.rs: ✅ 已修复并启用
// - unit.rs: ✅ 已修复并启用
// - work_city.rs: ✅ 已修复并启用
// - employee_type_enum.rs: ✅ 已修复并启用
// - functional_role.rs: ✅ 已修复并启用
// - functional_role_member.rs: ✅ 已修复并启用
// - job_family.rs: ✅ 已修复并启用
// - job_level.rs: ✅ 已修复并启用
// - scope.rs: ✅ 已修复并启用
// - user.rs: ✅ 已修复并启用

// 暂时禁用有语法错误的服务
// pub mod department;
// pub mod employee_type_enum;
// pub mod functional_role;
// pub mod functional_role_member;
// pub mod group;
// pub mod group_member;
// pub mod scope;
// pub mod unit;
// pub mod user;
// pub mod work_city;

// 重新导出已启用的服务类型
pub use custom_attr::*;
pub use department::*;
pub use employee_type_enum::*;
pub use functional_role::*;
pub use functional_role_member::*;
pub use group::*;
pub use job_family::*;
pub use job_level::*;
pub use job_title::*;
pub use scope::*;
pub use unit::*;
pub use user::*;
pub use work_city::*;

// 单独导出 group_member 模块避免命名冲突
pub use group_member::{
    AddGroupMemberBuilder, AddGroupMemberRequest, AddGroupMemberResponse,
    BatchAddGroupMembersRequest, BatchAddGroupMembersResponse, BatchRemoveGroupMembersRequest,
    BatchRemoveGroupMembersResponse, GroupMemberInfo, GroupMemberResult, GroupMemberService,
    ListGroupMembersBuilder, ListGroupMembersRequest, ListGroupMembersResponse,
    RemoveGroupMemberRequest, RemoveGroupMemberResponse,
};

// 以下模块导出被注释，因为对应模块存在语法错误：
// pub use department::*;
// pub use employee_type_enum::*;
// pub use functional_role::*;
// pub use functional_role_member::*;
// pub use group::*;
// pub use group_member.*;
// pub use job_family::*;
// pub use job_level::*;
// pub use scope::*;
// pub use unit::*;
// pub use user::*;
// pub use work_city::*;

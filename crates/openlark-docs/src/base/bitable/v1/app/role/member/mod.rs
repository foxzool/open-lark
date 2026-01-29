/// 自定义角色协作者（member）服务模块
///
/// 提供多维表格高级权限中自定义角色的协作者管理能力：
/// - 新增协作者
/// - 批量新增协作者
/// - 列出协作者
/// - 删除协作者
/// - 批量删除协作者
pub mod batch_create;
pub mod batch_delete;
pub mod create;
pub mod delete;
pub mod list;
pub mod models;

pub use batch_create::{BatchCreateRoleMemberRequest, BatchCreateRoleMemberResponse};
pub use batch_delete::{BatchDeleteRoleMemberRequest, BatchDeleteRoleMemberResponse};
pub use create::{CreateRoleMemberRequest, CreateRoleMemberResponse};
pub use delete::{DeleteRoleMemberRequest, DeleteRoleMemberResponse};
pub use list::{ListRoleMembersRequest, ListRoleMembersResponse};
pub use models::{RoleMemberId, RoleMemberIdType, RoleMemberInfo, RoleMemberType};

// Type alias for compatibility
pub type ServiceType = ();

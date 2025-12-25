/// 自定义角色协作者（member）服务模块
///
/// 提供多维表格高级权限中自定义角色的协作者管理能力：
/// - 新增协作者
/// - 批量新增协作者
/// - 列出协作者
/// - 删除协作者
/// - 批量删除协作者
use openlark_core::config::Config;

pub mod batch_create;
pub mod batch_delete;
pub mod create;
pub mod delete;
pub mod list;
pub mod models;

pub use batch_create::{
    BatchCreateRoleMemberRequest, BatchCreateRoleMemberRequestBuilder, BatchCreateRoleMemberResponse,
};
pub use batch_delete::{
    BatchDeleteRoleMemberRequest, BatchDeleteRoleMemberRequestBuilder, BatchDeleteRoleMemberResponse,
};
pub use create::{CreateRoleMemberRequest, CreateRoleMemberRequestBuilder, CreateRoleMemberResponse};
pub use delete::{DeleteRoleMemberRequest, DeleteRoleMemberRequestBuilder, DeleteRoleMemberResponse};
pub use list::{ListRoleMembersRequest, ListRoleMembersRequestBuilder, ListRoleMembersResponse};
pub use models::{RoleMemberId, RoleMemberIdType, RoleMemberInfo, RoleMemberType};

/// 协作者服务
pub struct RoleMemberService {
    config: Config,
}

impl RoleMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn create(&self) -> CreateRoleMemberRequestBuilder {
        CreateRoleMemberRequestBuilder::new(self.config.clone())
    }

    pub fn batch_create(&self) -> BatchCreateRoleMemberRequestBuilder {
        BatchCreateRoleMemberRequestBuilder::new(self.config.clone())
    }

    pub fn list(&self) -> ListRoleMembersRequestBuilder {
        ListRoleMembersRequestBuilder::new(self.config.clone())
    }

    pub fn delete(&self) -> DeleteRoleMemberRequestBuilder {
        DeleteRoleMemberRequestBuilder::new(self.config.clone())
    }

    pub fn batch_delete(&self) -> BatchDeleteRoleMemberRequestBuilder {
        BatchDeleteRoleMemberRequestBuilder::new(self.config.clone())
    }
}

// Type alias for compatibility
pub type ServiceType = RoleMemberService;

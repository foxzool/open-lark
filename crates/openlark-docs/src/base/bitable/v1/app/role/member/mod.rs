/// Bitable V1 App Role Member 模块
///
/// 提供多维表格应用角色成员管理功能，包括：
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

pub use batch_create::*;
pub use batch_delete::*;
pub use create::*;
pub use list::*;
pub use models::*;

/// 角色成员管理服务
pub struct RoleMemberService {
    pub config: Config,
}

impl RoleMemberService {
    /// 创建角色成员管理服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}

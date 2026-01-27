//! Hire v1 talent 资源模块
//!
//! 包含候选人相关 API：
//! - 获取候选人列表
//! - 获取候选人信息
//! - 综合创建候选人
//! - 综合更新候选人
//! - 批量获取候选人 ID
//! - 将人才加入/移除文件夹
//! - 操作人才标签
//! - 获取候选人入职状态
//! - 候选人外部信息管理

pub mod add_to_folder;
pub mod batch_get_id;
pub mod combined_create;
pub mod combined_update;
pub mod external_info;
pub mod get;
pub mod list;
pub mod models;
pub mod onboard_status;
pub mod remove_to_folder;
pub mod tag;

// Re-export 公共类型
pub use add_to_folder::*;
pub use batch_get_id::*;
pub use combined_create::*;
pub use combined_update::*;
pub use external_info::*;
pub use get::*;
pub use list::*;
pub use models::*;
pub use onboard_status::*;
pub use remove_to_folder::*;
pub use tag::*;

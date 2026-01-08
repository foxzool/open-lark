//! 会议预约（reserve）
//!
//! 资源：`/open-apis/vc/v1/reserves`

pub mod apply;
pub mod delete;
pub mod get;
pub mod get_active_meeting;
pub mod update;

// 导出所有模块内容
pub use apply::*;
pub use delete::*;
pub use get::*;
pub use get_active_meeting::*;
pub use update::*;

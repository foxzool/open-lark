//! 会议室配置（room_config）

pub mod query;
pub mod set;
pub mod set_checkboard_access_code;
pub mod set_room_access_code;

// 导出所有模块内容
pub use query::*;
pub use set::*;
pub use set_checkboard_access_code::*;
pub use set_room_access_code::*;

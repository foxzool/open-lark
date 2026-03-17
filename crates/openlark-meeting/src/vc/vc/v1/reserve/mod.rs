//! 会议预约（reserve）
//!
//! 资源：`/open-apis/vc/v1/reserves`

pub mod apply;
pub mod delete;
pub mod get;
pub mod get_active_meeting;
pub mod update;

// 导出所有模块内容
// apply 模块显式导出
pub use apply::{
    ApplyReserveRequest,
    ApplyReserveResponse,
};
// delete 模块显式导出
pub use delete::{
    DeleteReserveRequest,
    DeleteReserveResponse,
};
// get 模块显式导出
pub use get::{
    GetReserveRequest,
    GetReserveResponse,
};
// get_active_meeting 模块显式导出
pub use get_active_meeting::{
    GetActiveMeetingRequest,
};
// update 模块显式导出
pub use update::{
    UpdateReserveRequest,
    UpdateReserveResponse,
};

//! Open-Lark Collaboration Module
//!
//! 飞书协作服务模块，提供完整的协作和项目管理功能。
//!
//! ## 主要功能
//!
//! - **项目看板**: 看板管理、任务卡片、工作流
//! - **日程管理**: 日历安排、会议预订、时间协调
//! - **会议服务**: 会议室管理、会议纪要、参会安排
//! - **任务协作**: 任务分配、进度跟踪、团队协作
//!
//! ## 使用示例
//!
//! ```rust
//! use openlark_collab::endpoints::*;
//!
//! // 使用端点常量
//! let boards_endpoint = BOARD_V1_BOARDS;
//! let calendar_endpoint = CALENDAR_V4_CALENDARS;
//! let tasks_endpoint = TASK_V1_TASKS;
//! println!("看板端点: {}", boards_endpoint);
//! println!("日历端点: {}", calendar_endpoint);
//! println!("任务端点: {}", tasks_endpoint);
//! ```
//!
//! ## 端点组织
//!
//! - `board`: 项目看板端点
//! - `calendar`: 日程管理端点
//! - `meeting_room`: 会议室端点
//! - `minutes`: 会议纪要端点
//! - `task`: 任务协作端点

#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

// 导入端点模块
pub mod endpoints;

// 重新导出端点常量，方便外部使用
pub use endpoints::*;

/// Re-exports from openlark-core for convenience.
pub mod prelude {
    pub use openlark_core::SDKResult;
}

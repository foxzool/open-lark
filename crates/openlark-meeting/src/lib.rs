#![warn(clippy::all)]
#![allow(ambiguous_glob_reexports)]
#![allow(hidden_glob_reexports)]
#![allow(missing_docs)]

//! # OpenLark 会议与日程服务模块
//!
//! 飞书开放平台日历（calendar）、视频会议（vc）、会议室（meeting_room）相关 API。
//!
//! ## 模块与 API 数量（目标）
//! - **calendar**：44 APIs
//! - **vc**：56 APIs
//! - **meeting_room**：17 APIs
//!
//! ## 目录结构约定
//! 实现文件严格按 `src/bizTag/project/version/resource/name.rs` 组织，其中：
//! - `resource` 内用 `.` 分割目录（例如 `calendar.event.attendee` => `calendar/event/attendee`）
//! - 如 `meta.name` 内包含 `/`（历史接口），则按目录继续下沉（例如 `building/list` => `building/list.rs`）

// Core modules
pub mod common;
pub mod endpoints;

// 业务模块（按 bizTag 组织）
pub mod calendar;
pub mod meeting_room;
pub mod vc;

/// Prelude 模块 - 常用导入
pub mod prelude {
    pub use openlark_core::{config::Config, SDKResult};
}

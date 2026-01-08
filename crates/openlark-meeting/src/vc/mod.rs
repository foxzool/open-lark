//! 视频会议（vc）项目
//!
//! meta.project = vc

#![allow(clippy::module_inception)]

pub mod service;
pub mod v1;

pub use service::VcService;

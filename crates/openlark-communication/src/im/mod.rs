//! 即时通讯（im）
//!
//! 按 bizTag=im 组织的 API 实现。

#![allow(clippy::module_inception)]

// 先按 meta.Project 建目录，后续逐个补齐
pub mod card;
pub mod im;
pub mod im_ephemeral;
pub mod im_message;

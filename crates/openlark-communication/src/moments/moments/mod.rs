//! moments 项目
//!
//! 目录结构严格遵循：src/biztag/project/version/resource/name.rs

pub mod v1;

// 可选：统一 re-export，方便外部调用路径更短
pub use v1::*;

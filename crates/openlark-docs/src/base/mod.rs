#![allow(clippy::module_inception)]

// base bizTag 下包含 base 与 bitable 两个 project（CSV 的 meta.Project）
#[cfg(feature = "base")]
/// Base v2 基础服务模块。
pub mod base;

#[cfg(feature = "bitable")]
/// Bitable 多维表格模块。
pub mod bitable;

#[cfg(feature = "base")]
/// Base 兼容服务入口。
pub mod service;

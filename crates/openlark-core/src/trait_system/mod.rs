//! Trait System
//!
//! 为各业务 crate 提供最小且稳定的 trait 抽象（Service / ExecutableBuilder）。
//!
//! 说明：历史上部分业务 crate 直接依赖 `openlark_core::trait_system::*`，
//! 这里提供兼容层，避免在 workspace 内做大范围的路径迁移。

pub mod executable_builder;
pub mod service;

pub use executable_builder::ExecutableBuilder;
pub use service::Service;


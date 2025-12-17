// base bizTag 下包含 base 与 bitable 两个 project（CSV 的 meta.Project）
#[cfg(feature = "base")]
pub mod base;

#[cfg(feature = "bitable")]
pub mod bitable;

#[cfg(feature = "base")]
pub mod service;

#[cfg(feature = "base")]
pub use service::BaseService;

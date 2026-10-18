//! bot 资源模块（ADR 0001：Bot 转发壳已删，仅保留模块树维持 leaf 路径）。

#[cfg(feature = "v3")]
pub mod v3;
#[cfg(feature = "v4")]
pub mod v4;

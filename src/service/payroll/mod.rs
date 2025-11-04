//! Payroll service - 薪资管理服务
//!
//! 此服务模块暂时禁用，因为存在复杂的 serde 和导入冲突问题。
//! 将在后续版本中重新启用和修复。

// 重新导出核心模型，但不包括有问题的服务实现
pub use models::*;

// v1 模块暂时禁用大部分功能
pub mod v1;

// 基础模型仍然可用
pub mod models;

//! 筛选视图基础功能模块
//!
//! 提供筛选视图相关的API功能：
//! - 创建筛选视图
//! - 查询筛选视图列表
//! - 获取指定筛选视图信息
//! - 更新筛选视图属性
//! - 删除筛选视图

pub mod create;
pub mod delete;
pub mod get;
pub mod patch;
pub mod query;

// 重新导出所有API
pub use create::*;
pub use delete::*;
pub use get::*;
pub use patch::*;
pub use query::*;
/// Wiki V1 节点模块
pub mod search;

// 使用通配符导出所有子模块,避免维护大量重复的导出列表
pub use search::*;

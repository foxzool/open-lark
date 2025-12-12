//! 电子表格操作 API 模块
//!
//! 提供电子表格的核心操作功能，包括：
//! - 工作表操作
//! - 行列操作
//! - 单元格格式化
//! - 数据保护
//! - 数据验证

// 导入各个子模块
mod sheets_batch_update;
mod dimension_range;
mod dimension_range_update;
mod dimension_range_delete;
mod merge_cells;
mod protected_range;
mod data_validation;

// 重新导出公共API
pub use sheets_batch_update::*;
pub use dimension_range::*;
pub use dimension_range_update::*;
pub use dimension_range_delete::*;
pub use merge_cells::*;
pub use protected_range::*;
pub use data_validation::*;
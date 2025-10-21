//! 数据操作模块
//!
//! 提供电子表格数据的完整操作功能集，包括数据读写、查找替换、样式设置、
//! 单元格合并等核心功能。这是Sheets API v3版本中最重要的功能模块。
//!
//! # 核心功能
//!
//! ## 数据读写操作
//! - 📖 单范围和多范围数据读取
//! - ✏️ 单范围和多范围数据写入
//! - ➕ 数据追加（append_data）
//! - ⬆️ 数据前插（prepend_data）
//!
//! ## 数据查找和替换
//! - 🔍 查找单元格内容（find_cells）
//! - 🔄 替换单元格内容（replace_cells）
//! - 🎯 条件查找和批量替换
//!
//! ## 单元格操作
//! - 🔗 合并单元格（merge_cells）
//! - ✂️ 拆分单元格（split_cells）
//! - 🎨 设置单元格样式（set_cell_style）
//! - 🎨 批量设置样式（batch_set_cell_style）
//!
//! ## 图片和媒体
//! - 🖼️ 写入图片到单元格（write_images）
//! - 📸 图片格式处理和优化
//!
//! ## 共享和协作
//! - 👥 数据共享设置（share）
//! - 🔐 权限控制和访问管理
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 数据操作服务
//! let data_ops = &client.sheets.v3.data_operation;
//!
//! // 读取数据示例
//! // let read_request = ReadingSingleRangeRequest::builder()
//! //     .spreadsheet_token("spreadsheet_token")
//! //     .range("A1:C10")
//! //     .build();
//! // let data = data_ops.reading_single_range(read_request, None).await?;
//!
//! // 写入数据示例
//! // let write_request = WriteDataToMultipleRangesRequest::builder()
//! //     .spreadsheet_token("spreadsheet_token")
//! //     .value_ranges(vec![...])
//! //     .build();
//! // data_ops.write_data_to_multiple_ranges(write_request, None).await?;
//! ```
//!
//! # 最佳实践
//!
//! - 批量操作优于单个操作，提高性能
//! - 合理使用范围选择器，避免读取不必要的数据
//! - 数据写入前进行格式验证
//! - 大量数据操作时考虑分批处理
//! - 及时释放不需要的数据引用

pub use append_data::*;
pub use batch_set_cell_style::*;
pub use find_cells::*;
pub use merge_cells::*;
pub use prepend_data::*;
pub use reading_multiple_ranges::*;
pub use reading_single_range::*;
pub use replace_cells::*;
pub use set_cell_style::*;
pub use share::*;
pub use split_cells::*;
pub use write_data_to_multiple_ranges::*;
pub use write_images::*;

/// 追加数据到表格末尾
mod append_data;
/// 批量设置单元格样式
mod batch_set_cell_style;
/// 查找单元格内容
mod find_cells;
/// 合并单元格
mod merge_cells;
/// 在表格开头插入数据
mod prepend_data;
/// 读取多个范围的数据
mod reading_multiple_ranges;
/// 读取单个范围的数据
mod reading_single_range;
/// 替换单元格内容
mod replace_cells;
/// 设置单元格样式
mod set_cell_style;
/// 数据共享设置
mod share;
/// 拆分单元格
mod split_cells;
/// 写入数据到多个范围
mod write_data_to_multiple_ranges;
/// 写入图片到单元格
mod write_images;

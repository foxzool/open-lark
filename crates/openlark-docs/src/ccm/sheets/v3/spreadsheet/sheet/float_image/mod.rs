pub mod create;
pub mod delete;
pub mod get;
pub mod patch;
/// 浮动图片（spreadsheet.sheet.float_image）
pub mod query;

// 导出共享类型
pub use crate::ccm::sheets::v3::models::FloatImageInfo as FloatImage;

pub use create::*;
pub use delete::*;
pub use get::*;
pub use patch::*;
pub use query::*;

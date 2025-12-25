/// 浮动图片（spreadsheet.sheet.float_image）
pub mod create;
pub mod delete;
pub mod get;
pub mod patch;
pub mod query;

use serde::{Deserialize, Serialize};

/// 浮动图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatImage {
    pub float_image_id: String,
    pub float_image_token: String,
    pub range: String,
    pub width: f64,
    pub height: f64,
    pub offset_x: f64,
    pub offset_y: f64,
}

// 重新导出所有API函数
pub use create::*;
pub use delete::*;
pub use get::*;
pub use patch::*;
pub use query::*;

pub mod create;
pub mod delete;
pub mod get;
pub mod patch;
/// 浮动图片（spreadsheet.sheet.float_image）
pub mod query;

// 导出共享类型
pub use crate::ccm::sheets::v3::models::FloatImageInfo as FloatImage;

// create 模块显式导出

pub use create::{CreateFloatImageRequest, CreateFloatImageResponse};
// delete 模块显式导出
pub use delete::DeleteFloatImageResponse;
// get 模块显式导出
pub use get::GetFloatImageResponse;
// patch 模块显式导出
pub use patch::{UpdateFloatImageRequest, UpdateFloatImageResponse};
// query 模块显式导出
pub use query::QueryFloatImagesResponse;

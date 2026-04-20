/// 创建浮图接口。
pub mod create;
/// 删除浮图接口。
pub mod delete;
/// 获取浮图接口。
pub mod get;
/// 更新浮图接口。
pub mod patch;
/// 查询浮图接口。
pub mod query;

/// 重新导出共享浮图模型。
pub use crate::ccm::sheets::v3::models::FloatImageInfo as FloatImage;

/// 重新导出创建浮图类型。
pub use create::{CreateFloatImageRequest, CreateFloatImageResponse};
/// 重新导出删除浮图响应。
pub use delete::DeleteFloatImageResponse;
/// 重新导出获取浮图响应。
pub use get::GetFloatImageResponse;
/// 重新导出更新浮图类型。
pub use patch::{UpdateFloatImageRequest, UpdateFloatImageResponse};
/// 重新导出查询浮图响应。
pub use query::QueryFloatImagesResponse;

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
// create 模块显式导出
pub use create::{
    CreateFloatImageRequest,
    CreateFloatImageResponse,
    DeleteFloatImageResponse,
    GetFloatImageResponse,
    QueryFloatImagesResponse,
    UpdateFloatImageRequest,
    UpdateFloatImageResponse,
    create_float_image,
    create_float_image_with_options,
    delete_float_image,
    delete_float_image_with_options,
    get_float_image,
    get_float_image_with_options,
    query_float_images,
    query_float_images_with_options,
    update_float_image,
    update_float_image_with_options,
};
// delete 模块显式导出
pub use delete::{
    CreateFloatImageRequest,
    CreateFloatImageResponse,
    DeleteFloatImageResponse,
    GetFloatImageResponse,
    QueryFloatImagesResponse,
    UpdateFloatImageRequest,
    UpdateFloatImageResponse,
    create_float_image,
    create_float_image_with_options,
    delete_float_image,
    delete_float_image_with_options,
    get_float_image,
    get_float_image_with_options,
    query_float_images,
    query_float_images_with_options,
    update_float_image,
    update_float_image_with_options,
};
// get 模块显式导出
pub use get::{
    CreateFloatImageRequest,
    CreateFloatImageResponse,
    DeleteFloatImageResponse,
    GetFloatImageResponse,
    QueryFloatImagesResponse,
    UpdateFloatImageRequest,
    UpdateFloatImageResponse,
    create_float_image,
    create_float_image_with_options,
    delete_float_image,
    delete_float_image_with_options,
    get_float_image,
    get_float_image_with_options,
    query_float_images,
    query_float_images_with_options,
    update_float_image,
    update_float_image_with_options,
};
// patch 模块显式导出
pub use patch::{
    CreateFloatImageRequest,
    CreateFloatImageResponse,
    DeleteFloatImageResponse,
    GetFloatImageResponse,
    QueryFloatImagesResponse,
    UpdateFloatImageRequest,
    UpdateFloatImageResponse,
    create_float_image,
    create_float_image_with_options,
    delete_float_image,
    delete_float_image_with_options,
    get_float_image,
    get_float_image_with_options,
    query_float_images,
    query_float_images_with_options,
    update_float_image,
    update_float_image_with_options,
};
// query 模块显式导出
pub use query::{
    CreateFloatImageRequest,
    CreateFloatImageResponse,
    DeleteFloatImageResponse,
    GetFloatImageResponse,
    QueryFloatImagesResponse,
    UpdateFloatImageRequest,
    UpdateFloatImageResponse,
    create_float_image,
    create_float_image_with_options,
    delete_float_image,
    delete_float_image_with_options,
    get_float_image,
    get_float_image_with_options,
    query_float_images,
    query_float_images_with_options,
    update_float_image,
    update_float_image_with_options,
};

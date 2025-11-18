//! API模块便利导入 - 简化版本

// 从新api模块重新导出
pub use super::{
    traits::{AsyncApiClient, SyncApiClient},
    ApiRequest, ApiResponse, ResponseFormat,
};

// 临时从旧模块导出，保持兼容性
pub use crate::api_resp::{ApiResponseTrait, BaseResponse, RawResponse};

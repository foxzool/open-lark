//! OpenLark Client 类型系统
//!
//! 统一的类型定义，包括配置、错误、结果等

mod client;

// 重新导出类型
pub use client::*;

// 保留原有的auth模块用于兼容性
pub mod auth;

/// 📦 预导出常用类型
pub mod prelude {
    pub use super::{
        ApiResponse as ApiResponseTrait, ApiResponseData, PaginatedResponse, RequestOptions,
    };
}

//! 文档服务常用导入模块
//!
//! 提供文档服务开发中最常用的类型和trait的便捷导入。

// 重新导出核心模块（避免歧义重导出）
pub use openlark_core::prelude::{
    AccessTokenType, ApiResponseTrait, BaseResponse, LarkAPIError, RequestOption, ResponseFormat,
    SDKResult,
};

// 重新导出文档服务核心类型（避免冲突）
pub use crate::{
    error::{DocsError, DocsResult},
    service::DocsService,
};

// 显式导出常用模型，避免歧义
pub use crate::models::{
    DepartmentInfo, DocumentBase, DocumentStats, ErrorInfo, FileInfo, Permission, SearchResult,
    ShareInfo, UserInfo, VersionInfo,
};

// 重新导出服务类型（如果启用了相应功能）
#[cfg(feature = "ccm")]
pub use crate::ccm::CcmService;

#[cfg(feature = "base")]
pub use crate::base::BaseService;

#[cfg(feature = "baike")]
pub use crate::baike::BaikeService;

#[cfg(feature = "minutes")]
pub use crate::minutes::MinutesService;

// 为了向后兼容性，保留类型别名
pub type APIResult<T> = DocsResult<T>;

// 重新导出常用外部crate
pub use chrono::{DateTime, Utc};
pub use serde::{Deserialize, Serialize};
pub use std::collections::HashMap;
pub use std::sync::Arc;
pub use uuid::Uuid;

/// 文档服务预导入trait
pub trait DocsPrelude {}

impl<T> DocsPrelude for T {}

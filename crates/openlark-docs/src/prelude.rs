/// 文档服务常用导入模块
///
/// 提供文档服务开发中最常用的类型和trait的便捷导入。
// 重新导出核心模块（避免歧义重导出）
pub use openlark_core::prelude::{
    AccessTokenType, ApiResponseTrait, BaseResponse, HttpMethod, LarkAPIError, RequestOption,
    ResponseFormat, SDKResult,
};

// 显式导出常用模型，避免歧义
pub use crate::models::{
    DepartmentInfo, DocumentBase, DocumentStats, ErrorInfo, FileInfo, Permission, SearchResult,
    ShareInfo, UserInfo, VersionInfo,
};

// 重新导出服务类型（如果启用了相应功能）
#[cfg(feature = "ccm-core")]
pub use crate::ccm::CcmService;

#[cfg(feature = "base")]
pub use crate::base::BaseService;

#[cfg(feature = "baike")]
pub use crate::baike::BaikeService;

#[cfg(feature = "minutes")]
pub use crate::minutes::MinutesService;

#[cfg(feature = "docx")]
pub use crate::ccm::docx::DocxService;

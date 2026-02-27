/// 文档服务常用导入模块
///
/// 提供文档服务开发中最常用的类型和trait的便捷导入。
// 重新导出核心模块（避免歧义重导出）
pub use openlark_core::prelude::{
    AccessTokenType, ApiResponseTrait, BaseResponse, CoreError, HttpMethod, RequestOption,
    ResponseFormat, SDKResult,
};

// 显式导出常用模型，避免歧义
pub use crate::models::{
    DepartmentInfo, DocumentBase, DocumentStats, ErrorInfo, FileInfo, Permission, SearchResult,
    ShareInfo, UserInfo, VersionInfo,
};

// 已移除 Service 的 prelude 导出，统一使用 DocsClient 作为唯一入口
// 如需访问特定 Service，可通过完整路径访问：
// - CcmService: crate::ccm::CcmService
// - BaseService: crate::base::BaseService
// - BitableService: crate::base::bitable::BitableService
// - BaikeService: crate::baike::BaikeService
// - MinutesService: crate::minutes::MinutesService
// - WikiService: crate::ccm::wiki::WikiService

//! OpenLark Platform API 端点定义
//!
//! 提供统一的 API 端点管理和 URL 生成功能

/// Admin V1 API 端点枚举
#[derive(Debug, Clone)]
pub enum AdminApiV1 {
    /// 创建勋章
    CreateBadge,
    /// 获取勋章列表
    ListBadge,
}

impl AdminApiV1 {
    /// 获取对应的 API 路径
    pub fn path(&self) -> &'static str {
        match self {
            AdminApiV1::CreateBadge => "/open-apis/admin/v1/badges",
            AdminApiV1::ListBadge => "/open-apis/admin/v1/badges",
        }
    }
}

/// 模块导出
pub mod prelude {
    pub use super::AdminApiV1;
}

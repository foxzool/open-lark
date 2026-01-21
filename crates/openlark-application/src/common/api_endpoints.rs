//! 应用 API 端点定义（类型安全枚举系统）
//!
//! 本模块提供基于枚举的 API 端点定义，用于生产代码中的类型安全调用。

/// 应用 API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum AppApiV1 {
    /// 获取应用详情
    AppGet,
}

impl AppApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            AppApiV1::AppGet => "/open-apis/application/v1/applications".to_string(),
        }
    }
}

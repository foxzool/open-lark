//! 共享数据模型
//!
//! 提供所有安全服务共享的数据模型和类型定义。

use serde::{Deserialize, Serialize};

/// 安全服务配置
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// 应用ID
    pub app_id: String,
    /// 应用密钥
    pub app_secret: String,
    /// 基础URL
    pub base_url: String,
}

impl SecurityConfig {
    /// 创建新的安全配置实例
    pub fn new(app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            app_secret: app_secret.into(),
            base_url: "https://open.feishu.cn".to_string(),
        }
    }

    /// 设置基础URL
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }

    /// 获取应用访问令牌
    ///
    /// 使用 openlark-auth 的 AuthTokenProvider 获取真实的 app_access_token
    pub async fn get_app_access_token(&self) -> crate::SecurityResult<String> {
        use openlark_auth::AuthTokenProvider;
        use openlark_core::{auth::{TokenProvider, TokenRequest}, config::Config};

        // 从 SecurityConfig 创建 Config
        let config = Config::builder()
            .app_id(&self.app_id)
            .app_secret(&self.app_secret)
            .base_url(&self.base_url)
            .build();

        // 使用 AuthTokenProvider 获取 token
        let token_provider = AuthTokenProvider::new(config);
        let token: String = token_provider
            .get_token(TokenRequest::app())
            .await
            .map_err(|e: openlark_core::CoreError| openlark_core::error::authentication_error(e.to_string()))?;

        Ok(token)
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            app_id: "".to_string(),
            app_secret: "".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
        }
    }
}

/// 分页请求基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    /// 页面大小，默认 20，最大 100
    pub page_size: Option<i32>,
    /// 分页标记，第一页不传，后续页面传入上一页返回的 page_token
    pub page_token: Option<String>,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page_size: Some(20),
            page_token: None,
        }
    }
}

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    pub page_token: Option<String>,
    /// 数据列表
    pub data: Vec<T>,
}

/// API 响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
}

/// 时间戳类型
pub type Timestamp = i64;

/// 通用状态枚举
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// 激活
    Active,
    /// 禁用
    Disabled,
    /// 待处理
    Pending,
    /// 已删除
    Deleted,
    /// 已过期
    Expired,
}

// 子模块
pub mod acs;
pub mod common;

// 重新导出 ACS 相关模型
// acs 模块显式导出
pub use acs::{
    AccessRecord, AccessResult, DeviceBindRuleRequest, DeviceInfo, DeviceStatus, DeviceType,
    FaceImageInfo, HostInfo, PermissionRuleInfo, PermissionRuleRequest, UserInfo, UserListResponse,
    UserRequest, VerificationMethod, VisitorInfo, VisitorRequest, VisitorStatus,
};

// Security & Compliance 相关模型
pub mod security_and_compliance;
// security_and_compliance 模块显式导出
pub use security_and_compliance::{
    ApplyStatus, ComplianceCheckResult, ComplianceResult, ComplianceRuleType, ComplianceStatus,
    DeviceApplyRecord, DeviceApplyRecordApproveRequest, DeviceComplianceRule, DeviceRecord,
    DeviceRecordRequest, DeviceRecordStatus, DeviceRecordUpdateRequest, OpenApiLog,
    OpenApiLogQueryRequest,
};

// 重新导出通用模型
// common 模块显式导出
pub use common::{
    BatchOperationError, BatchOperationRequest, BatchOperationResponse, DeviceBase, ExtensionMap,
    FileUploadResponse, GeoLocation, KeyValue, OperationResponse, PermissionBase, QueryCondition,
    SortCondition, SortDirection, TimeRange, UserBase,
};

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}

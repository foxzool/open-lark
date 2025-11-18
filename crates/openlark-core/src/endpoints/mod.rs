//! API端点常量定义模块 (核心版)
//!
//! 本模块包含飞书开放平台的核心API端点常量。重构后采用分层架构：
//! - 核心端点保留在 openlark-core 中
//! - 业务服务端点已迁移到对应的 service crate 中
//!
//! # 核心设计原则
//!
//! 1. **单一职责**: openlark-core 只包含基础设施相关的端点
//! 2. **模块化**: 业务端点按服务域分离到对应 crate
//! 3. **清晰架构**: 每个服务通过专门的 crate 访问
//! 4. **功能标志**: 支持按需编译和服务组合
//!
//! # 核心端点保留原则
//!
//! 以下类型的端点保留在 core 中：
//! - 基础认证和授权 (auth)
//! - 应用管理 (application)
//! - 平台集成 (platform_integration)
//! - 通用基础设施 (apass)
//!
//! # 业务服务访问方式
//!
//! 业务服务端点已迁移到对应 crate：
//! - `openlark-admin`: admin, acs, mdm, tenant, workplace 等
//! - `openlark-ai`: ai, aily, ai_embedding, ai_workflow 等
//! - `openlark-communication`: im, mail, vc 等
//! - `openlark-people`: contact, directory 等
//! - `openlark-hr`: attendance, corehr 等
//! - `openlark-docs`: cloud_docs, drive, cardkit 等
//! - 其他服务以此类推

// 核心基础端点模块（仅保留基础设施相关的端点）
pub mod apass;
pub mod application;
pub mod auth;
pub mod platform_integration;

// 导出核心端点常量
pub use apass::*;
pub use application::*;
pub use auth::*;
pub use platform_integration::*;

// ==================== 端点构建器和类型定义 ====================

/// 端点管理器
///
/// 用于构建和管理 API 端点路径的工具结构
#[derive(Debug, Clone)]
pub struct Endpoints {
    // 端点构建逻辑将在具体实现中重新定义
}

impl Endpoints {
    /// 创建新的端点管理器
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Endpoints {
    fn default() -> Self {
        Self::new()
    }
}

/// 端点构建器
///
/// 用于动态构建 API 端点路径的构建器模式实现
#[derive(Debug, Clone)]
pub struct EndpointBuilder {
    base_url: String,
    path_segments: Vec<String>,
    query_params: Vec<(String, String)>,
}

impl EndpointBuilder {
    /// 创建新的端点构建器
    pub fn new() -> Self {
        Self {
            base_url: "/open-apis".to_string(),
            path_segments: Vec::new(),
            query_params: Vec::new(),
        }
    }

    /// 设置基础URL
    pub fn base_url<S: Into<String>>(mut self, url: S) -> Self {
        self.base_url = url.into();
        self
    }

    /// 添加路径段
    pub fn path<S: Into<String>>(mut self, segment: S) -> Self {
        self.path_segments.push(segment.into());
        self
    }

    /// 添加查询参数
    pub fn param<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 构建最终的端点URL
    pub fn build(self) -> String {
        let mut url = self.base_url;

        for segment in self.path_segments {
            url.push('/');
            url.push_str(&segment);
        }

        if !self.query_params.is_empty() {
            url.push('?');
            let params: Vec<String> = self
                .query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            url.push_str(&params.join("&"));
        }

        url
    }
}

impl Default for EndpointBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoints_creation() {
        let _endpoints = Endpoints::new();
        assert!(true); // 简单测试，确保创建成功
    }

    #[test]
    fn test_endpoint_builder() {
        let endpoint = EndpointBuilder::new()
            .path("auth")
            .path("v3")
            .path("app_access_token")
            .build();

        assert_eq!(endpoint, "/open-apis/auth/v3/app_access_token");
    }

    #[test]
    fn test_endpoint_builder_with_params() {
        let endpoint = EndpointBuilder::new()
            .path("im")
            .path("v1")
            .path("messages")
            .param("page_size", "20")
            .param("page_token", "abc123")
            .build();

        assert_eq!(
            endpoint,
            "/open-apis/im/v1/messages?page_size=20&page_token=abc123"
        );
    }

    #[test]
    fn test_endpoint_builder_custom_base() {
        let endpoint = EndpointBuilder::new()
            .base_url("/custom-api")
            .path("v1")
            .path("users")
            .build();

        assert_eq!(endpoint, "/custom-api/v1/users");
    }
}

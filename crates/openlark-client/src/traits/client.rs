//! OpenLark Client 核心特征
//!
//! 定义客户端的统一接口和行为

use crate::{Config, Result};
use async_trait::async_trait;
use std::time::Duration;

/// 🚀 OpenLark客户端核心特征
///
/// 所有OpenLark客户端实现都应该实现此特征
///
/// # 特性要求
/// - 异步支持：所有操作都是异步的
/// - 线程安全：客户端可以跨线程安全使用
/// - 配置访问：可以访问客户端配置
/// - 错误处理：统一的错误处理机制
#[async_trait]
pub trait LarkClient: Send + Sync {
    /// 🔧 获取客户端配置
    fn config(&self) -> &Config;

    /// ✅ 检查客户端是否已正确配置
    ///
    /// # 返回值
    /// 返回true如果app_id和app_secret都不为空
    fn is_configured(&self) -> bool {
        !self.config().app_id.is_empty() && !self.config().app_secret.is_empty()
    }

    /// 🔍 获取应用ID
    fn app_id(&self) -> &str {
        &self.config().app_id
    }

    /// 🔑 获取应用密钥
    fn app_secret(&self) -> &str {
        &self.config().app_secret
    }

    /// 🌐 获取API基础URL
    fn base_url(&self) -> &str {
        &self.config().base_url
    }

    /// ⏱️ 获取请求超时时间
    fn timeout(&self) -> Duration {
        self.config().timeout
    }

    /// 🔄 获取重试次数
    fn retry_count(&self) -> u32 {
        self.config().retry_count
    }

    /// 📝 检查是否启用了日志
    fn is_log_enabled(&self) -> bool {
        self.config().enable_log
    }
}

/// 🔐 可认证客户端特征
///
/// 扩展LarkClient，添加认证相关功能
#[async_trait]
pub trait AuthenticatedClient: LarkClient {
    /// 🔐 获取访问令牌
    async fn get_access_token(&self) -> Result<String>;

    /// 🔄 刷新访问令牌
    async fn refresh_token(&self) -> Result<()>;

    /// 🔍 检查令牌是否有效
    async fn is_token_valid(&self) -> Result<bool>;

    /// 🚫 注销令牌
    async fn revoke_token(&self) -> Result<()>;
}

/// 📡 请求发送客户端特征
///
/// 扩展LarkClient，添加HTTP请求发送功能
#[async_trait]
pub trait RequestClient: LarkClient {
    /// 📡 发送HTTP请求（通用方法）
    async fn send_request<R, Resp>(&self, request: R) -> Result<Resp>
    where
        R: serde::Serialize + Send + Sync,
        Resp: for<'de> serde::Deserialize<'de> + Send + 'static;

    /// 📡 发送GET请求
    async fn get<Resp>(&self, endpoint: &str) -> Result<Resp>
    where
        Resp: for<'de> serde::Deserialize<'de> + Send + 'static;

    /// 📤 发送POST请求
    async fn post<Req, Resp>(&self, endpoint: &str, data: &Req) -> Result<Resp>
    where
        Req: serde::Serialize + Send + Sync,
        Resp: for<'de> serde::Deserialize<'de> + Send + 'static;

    /// 🔄 发送PUT请求
    async fn put<Req, Resp>(&self, endpoint: &str, data: &Req) -> Result<Resp>
    where
        Req: serde::Serialize + Send + Sync,
        Resp: for<'de> serde::Deserialize<'de> + Send + 'static;

    /// 🗑️ 发送DELETE请求
    async fn delete(&self, endpoint: &str) -> Result<()>;

    /// 📡 发送PATCH请求
    async fn patch<Req, Resp>(&self, endpoint: &str, data: &Req) -> Result<Resp>
    where
        Req: serde::Serialize + Send + Sync,
        Resp: for<'de> serde::Deserialize<'de> + Send + 'static;
}

/// 🏗️ 客户端构建器特征
///
/// 定义客户端构建器的标准接口
pub trait ClientBuilder: Sized {
    /// 🔧 客户端类型
    type Output: LarkClient;

    /// 🆔 设置应用ID
    fn app_id<S: Into<String>>(self, app_id: S) -> Self;

    /// 🔑 设置应用密钥
    fn app_secret<S: Into<String>>(self, app_secret: S) -> Self;

    /// 🌐 设置基础URL
    fn base_url<S: Into<String>>(self, base_url: S) -> Self;

    /// ⏱️ 设置超时时间
    fn timeout(self, timeout: Duration) -> Self;

    /// 🔄 设置重试次数
    fn retry_count(self, count: u32) -> Self;

    /// 🌍 从环境变量加载配置
    fn with_env(self) -> Self;

    /// 🔨 构建客户端实例
    fn build(self) -> Result<Self::Output>;
}

/// 📊 可监控客户端特征
///
/// 扩展客户端，添加监控和统计功能
#[async_trait]
pub trait MonitorableClient: LarkClient {
    /// 📊 获取客户端统计信息
    async fn get_stats(&self) -> ClientStats;

    /// 🔄 重置统计信息
    async fn reset_stats(&self) -> Result<()>;

    /// 📝 获取客户端健康状态
    async fn health_check(&self) -> Result<ClientHealth>;
}

/// 📊 客户端统计信息
#[derive(Debug, Clone, Copy)]
pub struct ClientStats {
    /// 📡 总请求数
    pub total_requests: u64,
    /// ✅ 成功请求数
    pub successful_requests: u64,
    /// ❌ 失败请求数
    pub failed_requests: u64,
    /// ⏱️ 平均响应时间（毫秒）
    pub average_response_time: f64,
    /// 📈 当前QPS
    pub current_qps: f64,
    /// 🔄 上次请求时间
    pub last_request_time: Option<std::time::SystemTime>,
    /// 🔧 客户端运行时间
    pub uptime: Duration,
}

impl Default for ClientStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: 0.0,
            current_qps: 0.0,
            last_request_time: None,
            uptime: Duration::from_secs(0),
        }
    }
}

/// 🏥 客户端健康状态
#[derive(Debug, Clone)]
pub struct ClientHealth {
    /// ✅ 是否健康
    pub healthy: bool,
    /// 📝 状态描述
    pub status: String,
    /// 🔗 最后检查时间
    pub last_check: std::time::SystemTime,
    /// 🏷️ 健康标签
    pub tags: Vec<String>,
}

impl ClientHealth {
    /// ✅ 创建健康状态
    pub fn healthy(status: &str) -> Self {
        Self {
            healthy: true,
            status: status.to_string(),
            last_check: std::time::SystemTime::now(),
            tags: Vec::new(),
        }
    }

    /// ❌ 创建不健康状态
    pub fn unhealthy(status: &str) -> Self {
        Self {
            healthy: false,
            status: status.to_string(),
            last_check: std::time::SystemTime::now(),
            tags: Vec::new(),
        }
    }

    /// 🏷️ 添加标签
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
}

/// 🔧 可配置客户端特征
///
/// 扩展客户端，添加动态配置能力
#[async_trait]
pub trait ConfigurableClient: LarkClient {
    /// 🔧 更新配置
    async fn update_config(&self, config: Config) -> Result<()>;

    /// 🔍 获取当前配置的摘要
    async fn get_config_summary(&self) -> String;

    /// 🔄 重新加载配置
    async fn reload_config(&self) -> Result<()>;
}

/// 🔄 可重启客户端特征
///
/// 扩展客户端，添加重启和重置功能
#[async_trait]
pub trait RestartableClient: LarkClient {
    /// 🔄 重启客户端
    async fn restart(&self) -> Result<()>;

    /// 🧹 清理客户端状态
    async fn cleanup(&self) -> Result<()>;

    /// ✅ 验证客户端状态
    async fn validate(&self) -> Result<()>;
}

/// 📄 客户端信息特征
///
/// 扩展客户端，添加信息查询功能
pub trait InfoClient: LarkClient {
    /// 🏷️ 获取客户端版本
    fn version(&self) -> &'static str;

    /// 📋 获取客户端功能列表
    fn features(&self) -> Vec<&'static str>;

    /// 🔍 获取客户端元数据
    fn metadata(&self) -> ClientMetadata;

    /// 🏷️ 获取客户端标识符
    fn client_id(&self) -> String;
}

/// 📋 客户端元数据
#[derive(Debug, Clone)]
pub struct ClientMetadata {
    /// 🏷️ 客户端名称
    pub name: String,
    /// 🔢 客户端版本
    pub version: String,
    /// 📝 客户端描述
    pub description: String,
    /// 🏷️ 客户端标签
    pub tags: Vec<String>,
    /// 🔗 相关链接
    pub links: std::collections::HashMap<String, String>,
}

impl Default for ClientMetadata {
    fn default() -> Self {
        Self {
            name: "OpenLark Client".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Rust SDK for Feishu/Lark Open API".to_string(),
            tags: vec!["rust".to_string(), "feishu".to_string(), "lark".to_string()],
            links: std::collections::HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // 创建一个测试用的客户端实现
    struct TestClient {
        config: Config,
    }

    impl LarkClient for TestClient {
        fn config(&self) -> &Config {
            &self.config
        }
    }

    #[test]
    fn test_lark_client_basic_methods() {
        let config = Config {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://test.feishu.cn".to_string(),
            timeout: Duration::from_secs(30),
            retry_count: 3,
            enable_log: true,
            headers: std::collections::HashMap::new(),
        };

        let client = TestClient { config };

        assert_eq!(client.app_id(), "test_app_id");
        assert_eq!(client.app_secret(), "test_app_secret");
        assert_eq!(client.base_url(), "https://test.feishu.cn");
        assert_eq!(client.timeout(), Duration::from_secs(30));
        assert_eq!(client.retry_count(), 3);
        assert!(client.is_log_enabled());
        assert!(client.is_configured());
    }

    #[test]
    fn test_client_stats_default() {
        let stats = ClientStats::default();
        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.successful_requests, 0);
        assert_eq!(stats.failed_requests, 0);
        assert_eq!(stats.average_response_time, 0.0);
        assert_eq!(stats.current_qps, 0.0);
        assert!(stats.last_request_time.is_none());
    }

    #[test]
    fn test_client_health() {
        let healthy = ClientHealth::healthy("All systems operational");
        assert!(healthy.healthy);
        assert_eq!(healthy.status, "All systems operational");
        assert!(healthy.tags.is_empty());

        let unhealthy = ClientHealth::unhealthy("Database connection failed").with_tag("database");
        assert!(!unhealthy.healthy);
        assert_eq!(unhealthy.status, "Database connection failed");
        assert_eq!(unhealthy.tags.len(), 1);
        assert!(unhealthy.tags.contains(&"database".to_string()));
    }

    #[test]
    fn test_client_metadata_default() {
        let metadata = ClientMetadata::default();
        assert_eq!(metadata.name, "OpenLark Client");
        assert_eq!(metadata.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(metadata.description, "Rust SDK for Feishu/Lark Open API");
        assert!(metadata.tags.iter().any(|tag| tag == "rust"));
        assert!(metadata.tags.iter().any(|tag| tag == "feishu"));
        assert!(metadata.tags.iter().any(|tag| tag == "lark"));
    }

    #[test]
    fn test_not_configured_client() {
        let config = Config {
            app_id: String::new(),
            app_secret: String::new(),
            ..Default::default()
        };

        let client = TestClient { config };
        assert!(!client.is_configured());
    }
}

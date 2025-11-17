//! OpenLark 统一客户端核心特征定义

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

use openlark_core::SDKResult;

/// 统一服务特征
///
/// 所有服务适配器必须实现此特征，确保一致的行为模式。
pub trait UnifiedService: Send + Sync + Debug {
    /// 服务配置类型
    type Config: Send + Sync + Debug + Clone;

    /// 服务错误类型
    type Error: std::error::Error + Send + Sync + 'static;

    /// 服务名称，用于注册和查找
    fn name(&self) -> &'static str;

    /// 服务版本
    fn version(&self) -> &'static str;

    /// 配置服务
    async fn configure(&mut self, config: Self::Config) -> SDKResult<()>;

    /// 检查服务是否可用
    fn is_available(&self) -> bool;

    /// 获取服务状态
    fn status(&self) -> ServiceStatus;

    /// 获取服务描述信息
    fn descriptor(&self) -> ServiceDescriptor;
}

/// API调用特征
///
/// 统一请求/响应处理模式，所有API调用都遵循此接口。
pub trait APICall<T, R>: Send + Sync {
    /// 执行API调用
    async fn execute(&self, request: T) -> SDKResult<R>;

    /// 批量执行API调用
    async fn execute_batch(&self, requests: Vec<T>) -> SDKResult<Vec<R>> {
        let mut results = Vec::with_capacity(requests.len());
        for request in requests {
            results.push(self.execute(request).await?);
        }
        Ok(results)
    }

    /// 带重试的API调用
    async fn execute_with_retry(&self, request: T, max_retries: u32) -> SDKResult<R>
    where
        Self: Clone,
    {
        let mut retries = 0;
        let mut last_error = None;

        loop {
            match self.execute(request.clone()).await {
                Ok(result) => return Ok(result),
                Err(error) if retries < max_retries => {
                    retries += 1;
                    last_error = Some(error);
                    // 简单的退避策略
                    tokio::time::sleep(std::time::Duration::from_millis(100 * retries as u64)).await;
                }
                Err(error) => return Err(error),
            }
        }
    }
}

/// 服务构建器特征
///
/// 提供类型安全的服务配置和构建。
pub trait ServiceBuilder: Send + Sync + Sized {
    type Service: UnifiedService;
    type Config;

    /// 创建新的构建器实例
    fn new() -> Self;

    /// 设置服务配置
    fn config(self, config: Self::Config) -> Self;

    /// 构建服务实例
    fn build(self) -> SDKResult<Self::Service>;

    /// 从环境配置构建
    fn from_env() -> SDKResult<Self::Service>
    where
        Self::Config: FromEnvConfig,
    {
        let config = Self::Config::from_env()?;
        Self::new().config(config).build()
    }
}

/// 从环境变量加载配置的特征
pub trait FromEnvConfig: Sized {
    /// 从环境变量加载配置
    fn from_env() -> SDKResult<Self>;
}

/// 服务状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStatus {
    /// 未初始化
    Uninitialized,
    /// 正在启动
    Starting,
    /// 运行中
    Running,
    /// 暂停
    Paused,
    /// 错误状态
    Error,
    /// 已停止
    Stopped,
}

/// 服务描述信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDescriptor {
    /// 服务名称
    pub name: String,
    /// 服务版本
    pub version: String,
    /// 服务描述
    pub description: String,
    /// 服务状态
    pub status: ServiceStatus,
    /// 服务标签
    pub tags: Vec<String>,
    /// 服务依赖
    pub dependencies: Vec<String>,
    /// 服务元数据
    pub metadata: std::collections::HashMap<String, String>,
}

impl ServiceDescriptor {
    /// 创建新的服务描述
    pub fn new(name: &str, version: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            status: ServiceStatus::Uninitialized,
            tags: Vec::new(),
            dependencies: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// 添加标签
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// 添加依赖
    pub fn with_dependency(mut self, dependency: impl Into<String>) -> Self {
        self.dependencies.push(dependency.into());
        self
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// 更新状态
    pub fn update_status(&mut self, status: ServiceStatus) {
        self.status = status;
    }
}

/// 服务生命周期特征
///
/// 定义服务的生命周期管理方法。
#[async_trait]
pub trait ServiceLifecycle: Send + Sync {
    /// 启动服务
    async fn start(&mut self) -> SDKResult<()>;

    /// 停止服务
    async fn stop(&mut self) -> SDKResult<()>;

    /// 重启服务
    async fn restart(&mut self) -> SDKResult<()> {
        self.stop().await?;
        self.start().await
    }

    /// 健康检查
    async fn health_check(&self) -> SDKResult<bool>;
}

/// 可克隆的服务特征
///
/// 对于需要在多个地方共享的服务，提供安全的克隆机制。
pub trait CloneableService: UnifiedService + Clone {
    /// 创建服务实例的深拷贝
    fn clone_service(&self) -> Self;
}

/// 异步初始化特征
///
/// 支持异步初始化的服务。
#[async_trait]
pub trait AsyncInit: Send + Sync {
    /// 异步初始化
    async fn init(&mut self) -> SDKResult<()>;

    /// 检查是否已初始化
    fn is_initialized(&self) -> bool;
}

/// 配置验证特征
///
/// 支持配置验证的服务。
pub trait ConfigValidation {
    /// 验证配置
    fn validate_config(config: &Self::Config) -> SDKResult<()>
    where
        Self: UnifiedService;

    /// 获取配置模式（用于文档生成）
    fn config_schema() -> serde_json::Value;
}
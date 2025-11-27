//! 服务抽象层
//!
//! 提供统一的 `Service` / `ServiceKind` / `ServiceHealth` 定义，
//! 支撑新的服务生命周期与可观测性模型。

use async_trait::async_trait;
use std::borrow::Cow;

use super::context::ServiceContext;
use crate::{Config, Result};

/// 服务类型标识
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceKind {
    /// 服务名称（唯一）
    pub name: Cow<'static, str>,
    /// 版本号（语义化）
    pub version: Cow<'static, str>,
}

impl ServiceKind {
    /// 快速构建
    pub fn new(name: impl Into<Cow<'static, str>>, version: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }
}

/// 服务健康状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceHealth {
    /// 已就绪
    Ready,
    /// 启动中
    Starting,
    /// 降级运行（附带原因）
    Degraded { reason: String },
    /// 未就绪/不可用（附带原因）
    Unhealthy { reason: String },
    /// 已停止
    Stopped,
}

impl ServiceHealth {
    /// 是否可提供服务
    pub fn is_ready(&self) -> bool {
        matches!(self, ServiceHealth::Ready | ServiceHealth::Degraded { .. })
    }
}

/// 服务生命周期接口
#[async_trait]
pub trait Service: Send + Sync + std::fmt::Debug + 'static {
    /// 类型标识
    fn kind(&self) -> ServiceKind;

    /// 能力声明，用于能力发现/观测
    fn capabilities(&self) -> &'static [&'static str] {
        &[]
    }

    /// 静态依赖列表，配合拓扑排序
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    /// 初始化（创建连接、预热等）
    async fn init(&self, _ctx: &ServiceContext) -> Result<()> {
        Ok(())
    }

    /// 启动（可能触发后台任务）
    async fn start(&self, _ctx: &ServiceContext) -> Result<()> {
        Ok(())
    }

    /// 停止/清理
    async fn stop(&self) -> Result<()> {
        Ok(())
    }

    /// 健康检查
    fn health(&self) -> ServiceHealth {
        ServiceHealth::Ready
    }
}

/// 简易 Provider，避免重复闭包定义
pub struct ServiceProvider<F>
where
    F: Fn(&ServiceContext) -> Result<std::sync::Arc<dyn Service>> + Send + Sync + 'static,
{
    factory: F,
    kind: ServiceKind,
    dependencies: &'static [&'static str],
    capabilities: &'static [&'static str],
}

impl<F> ServiceProvider<F>
where
    F: Fn(&ServiceContext) -> Result<std::sync::Arc<dyn Service>> + Send + Sync + 'static,
{
    pub fn new(
        kind: ServiceKind,
        dependencies: &'static [&'static str],
        capabilities: &'static [&'static str],
        factory: F,
    ) -> Self {
        Self {
            factory,
            kind,
            dependencies,
            capabilities,
        }
    }

    pub fn kind(&self) -> &ServiceKind {
        &self.kind
    }

    pub fn dependencies(&self) -> &'static [&'static str] {
        self.dependencies
    }

    pub fn capabilities(&self) -> &'static [&'static str] {
        self.capabilities
    }

    pub fn create(&self, ctx: &ServiceContext) -> Result<std::sync::Arc<dyn Service>> {
        (self.factory)(ctx)
    }
}

impl From<&Config> for ServiceContext {
    fn from(config: &Config) -> Self {
        ServiceContext::new(config.clone())
    }
}

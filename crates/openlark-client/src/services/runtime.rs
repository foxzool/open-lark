//! 服务运行时
//!
//! 负责基于 ServiceProvider 构建拓扑、执行 init/start/stop 生命周期并维护健康状态。

use std::collections::HashMap;
use std::sync::Arc;

use crate::{error, Result};

use super::context::ServiceContext;
use super::graph::{self, GraphError};
use super::service::{Service, ServiceHealth, ServiceKind, ServiceProvider};
use super::typed_registry::{ServiceHandle, TypedServiceRegistry};

/// 运行时构建器
pub struct ServiceRuntimeBuilder {
    ctx: ServiceContext,
    providers: Vec<Arc<dyn ErasedProvider>>, // 保持对象安全
}

impl ServiceRuntimeBuilder {
    pub fn new(ctx: ServiceContext) -> Self {
        Self {
            ctx,
            providers: Vec::new(),
        }
    }

    /// 注册一个 Provider
    pub fn register<F>(&mut self, provider: ServiceProvider<F>) -> &mut Self
    where
        F: Fn(&ServiceContext) -> Result<Arc<dyn Service>> + Send + Sync + 'static,
    {
        self.providers.push(Arc::new(provider));
        self
    }

    pub fn build(self) -> ServiceRuntime {
        ServiceRuntime {
            ctx: self.ctx,
            providers: self.providers,
            registry: TypedServiceRegistry::new(),
        }
    }
}

/// 运行时主体
pub struct ServiceRuntime {
    ctx: ServiceContext,
    providers: Vec<Arc<dyn ErasedProvider>>,
    registry: TypedServiceRegistry,
}

impl ServiceRuntime {
    /// 拓扑化初始化+启动
    pub async fn bootstrap(&self) -> Result<()> {
        let graph = self.build_graph();
        let ordered = graph::topo_sort(&graph).map_err(|e| map_graph_error(e))?;

        for name in ordered {
            let provider = self
                .providers
                .iter()
                .find(|p| p.kind().name == name)
                .ok_or_else(|| error::validation_error("service", format!("未找到服务 {name}")))?;

            let service = provider.create(&self.ctx)?;

            service.init(&self.ctx).await?;
            service.start(&self.ctx).await?;

            let health = service.health();

            let handle = ServiceHandle {
                kind: provider.kind().clone(),
                health,
                service,
            };

            self.registry.register(handle)?;
        }

        Ok(())
    }

    /// 停止所有服务（逆拓扑）
    pub async fn shutdown(&self) {
        // 逆序保证先停下游
        if let Ok(order) = graph::topo_sort(&self.build_graph()) {
            for name in order.into_iter().rev() {
                if let Some(handle) = self.registry.get(&name) {
                    let _ = handle.service.stop().await;
                }
            }
        }
    }

    pub fn registry(&self) -> &TypedServiceRegistry {
        &self.registry
    }

    pub fn health_report(&self) -> HashMap<String, ServiceHealth> {
        self.registry()
            .list()
            .into_iter()
            .map(|h| (h.kind.name.to_string(), h.health))
            .collect()
    }

    fn build_graph(&self) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();
        for provider in &self.providers {
            map.insert(
                provider.kind().name.to_string(),
                provider
                    .dependencies()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            );
        }
        map
    }
}

fn map_graph_error(err: GraphError) -> error::Error {
    match err {
        GraphError::Cyclic(cycle) => error::validation_error("service_dependencies", cycle),
        GraphError::Missing(name) => error::validation_error("missing_dependency", name),
    }
}

/// 对象安全包装，便于存储在 Vec<Arc<dyn ...>> 中
trait ErasedProvider: Send + Sync {
    fn kind(&self) -> &ServiceKind;
    fn dependencies(&self) -> &'static [&'static str];
    fn capabilities(&self) -> &'static [&'static str];
    fn create(&self, ctx: &ServiceContext) -> Result<Arc<dyn Service>>;
}

impl<F> ErasedProvider for super::service::ServiceProvider<F>
where
    F: Fn(&ServiceContext) -> Result<Arc<dyn Service>> + Send + Sync + 'static,
{
    fn kind(&self) -> &ServiceKind {
        ServiceProvider::kind(self)
    }

    fn dependencies(&self) -> &'static [&'static str] {
        ServiceProvider::dependencies(self)
    }

    fn capabilities(&self) -> &'static [&'static str] {
        ServiceProvider::capabilities(self)
    }

    fn create(&self, ctx: &ServiceContext) -> Result<Arc<dyn Service>> {
        ServiceProvider::create(self, ctx)
    }
}

// 便捷函数
pub fn builder(ctx: ServiceContext) -> ServiceRuntimeBuilder {
    ServiceRuntimeBuilder::new(ctx)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::loader::build_runtime_with_defaults;
    use crate::Config;

    #[tokio::test]
    async fn bootstrap_registers_auth_service() {
        let mut config = Config::default();
        config.app_id = "test_app".into();
        config.app_secret = "test_secret".into();

        let runtime = build_runtime_with_defaults(config);

        runtime.bootstrap().await.expect("bootstrap should succeed");

        let report = runtime.health_report();
        assert_eq!(report.get("auth"), Some(&ServiceHealth::Ready));
    }
}

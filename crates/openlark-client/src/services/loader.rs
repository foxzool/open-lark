//! 默认服务注册器
//!
//! 将现有服务接入新的运行时。当前先以 AuthService 作为示例，后续逐步覆盖其他业务模块。

use std::sync::Arc;

use crate::Config;

use super::context::ServiceContext;
use super::runtime::{builder, ServiceRuntime, ServiceRuntimeBuilder};
use super::service::{ServiceKind, ServiceProvider};
use super::AuthService;

/// 构建带有默认服务的运行时
pub fn build_runtime_with_defaults(config: Config) -> ServiceRuntime {
    let ctx = ServiceContext::new(config);
    let mut b = builder(ctx.clone());

    register_auth(&mut b, ctx.clone());

    // TODO: 后续逐步接入 communication/docs/hr/... 模块

    b.build()
}

fn register_auth(builder: &mut ServiceRuntimeBuilder, _ctx: ServiceContext) {
    let provider = ServiceProvider::new(
        ServiceKind::new("auth", "v1"),
        &[],
        &["token", "oauth"],
        move |ctx: &ServiceContext| Ok(Arc::new(AuthService::new(&ctx.config))),
    );

    builder.register(provider);
}

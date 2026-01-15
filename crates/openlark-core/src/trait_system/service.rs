//! Service trait
//!
//! 业务服务的最小能力抽象：持有 Config，并提供服务标识信息。

use crate::config::Config;

/// 业务服务抽象
pub trait Service {
    /// 获取 SDK 配置
    fn config(&self) -> &Config;

    /// 服务名（用于日志/指标/路由等）
    fn service_name() -> &'static str
    where
        Self: Sized;

    /// 服务版本（默认 v1）
    fn service_version() -> &'static str
    where
        Self: Sized,
    {
        "v1"
    }
}


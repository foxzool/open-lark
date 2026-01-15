//! HTTP 客户端性能配置（精简版）
//!
//! 仅保留与 `ConfigBuilder` 相关的“reqwest Client 构建配置”，避免在 core 中混入基准测试/对比工具。

use reqwest::{Client, ClientBuilder};
use std::time::Duration;

/// HTTP 客户端性能优化配置
#[derive(Debug, Clone)]
pub struct OptimizedHttpConfig {
    /// 连接池最大空闲连接数 (默认: 90)
    pub pool_max_idle_per_host: usize,
    /// 连接池空闲连接超时 (默认: 90秒)
    pub pool_idle_timeout: Duration,
    /// TCP连接超时 (默认: 10秒)
    pub connect_timeout: Duration,
    /// 请求总超时 (默认: 30秒)
    pub request_timeout: Duration,
    /// 启用 gzip 压缩 (默认: true)
    pub gzip: bool,
    /// 启用 brotli 压缩 (默认: true)
    pub brotli: bool,
    /// User-Agent 字符串
    pub user_agent: String,
}

impl Default for OptimizedHttpConfig {
    fn default() -> Self {
        Self {
            pool_max_idle_per_host: 90,
            pool_idle_timeout: Duration::from_secs(90),
            connect_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
            gzip: true,
            brotli: true,
            user_agent: format!("open-lark/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

impl OptimizedHttpConfig {
    /// 创建生产环境优化配置
    pub fn production() -> Self {
        Self {
            pool_max_idle_per_host: 100,
            pool_idle_timeout: Duration::from_secs(120),
            connect_timeout: Duration::from_secs(5),
            request_timeout: Duration::from_secs(30),
            gzip: true,
            brotli: true,
            user_agent: format!("open-lark/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    /// 创建高吞吐量配置
    pub fn high_throughput() -> Self {
        Self {
            pool_max_idle_per_host: 200,
            pool_idle_timeout: Duration::from_secs(180),
            connect_timeout: Duration::from_secs(3),
            request_timeout: Duration::from_secs(15),
            gzip: true,
            brotli: true,
            user_agent: format!("open-lark/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    /// 创建低延迟配置
    pub fn low_latency() -> Self {
        Self {
            pool_max_idle_per_host: 50,
            pool_idle_timeout: Duration::from_secs(60),
            connect_timeout: Duration::from_secs(2),
            request_timeout: Duration::from_secs(10),
            gzip: false, // 减少CPU开销
            brotli: false,
            user_agent: format!("open-lark/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    /// 根据配置构建优化的 HTTP 客户端
    pub fn build_client(&self) -> Result<Client, reqwest::Error> {
        let mut builder = ClientBuilder::new()
            .pool_max_idle_per_host(self.pool_max_idle_per_host)
            .pool_idle_timeout(self.pool_idle_timeout)
            .connect_timeout(self.connect_timeout)
            .timeout(self.request_timeout)
            .user_agent(&self.user_agent);

        if !self.gzip {
            builder = builder.no_gzip();
        }
        if !self.brotli {
            builder = builder.no_brotli();
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_variants_build_client() {
        let configs = [
            OptimizedHttpConfig::default(),
            OptimizedHttpConfig::production(),
            OptimizedHttpConfig::high_throughput(),
            OptimizedHttpConfig::low_latency(),
        ];

        for cfg in configs {
            let client = cfg.build_client();
            assert!(client.is_ok());
        }
    }
}

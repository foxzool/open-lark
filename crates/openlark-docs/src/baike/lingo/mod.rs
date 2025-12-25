/// Lingo语言服务模块
///
/// 提供智能语言处理功能，包括草稿管理和词条管理。
use openlark_core::config::Config;

// 导出v1版本实现
pub mod v1;

// 重新导出服务
pub use v1::LingoV1Service;

/// Lingo语言服务（兼容旧接口）
#[derive(Debug, Clone)]
pub struct LingoService {
    /// 配置信息
    config: Config,
}

impl LingoService {
    /// 创建新的 Lingo 服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取lingo项目v1服务
    pub fn v1(&self) -> LingoV1Service<'_> {
        LingoV1Service::new(&self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lingo_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = LingoService::new(config.clone());

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }
}

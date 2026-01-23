use openlark_core::config::Config;

/// 基础服务（内部实现，通过 DocsClient 访问）
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct BaseService {
    config: Config,
}

impl BaseService {
    /// 创建新的基础服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// V2版本接口
    pub fn v2(&self) -> crate::base::base::v2::BaseV2Service {
        crate::base::base::v2::BaseV2Service::new(self.config.clone())
    }
}

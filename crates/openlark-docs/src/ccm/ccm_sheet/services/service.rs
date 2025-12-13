/// CCM Sheet 服务实现
///
/// 提供 Sheet 相关的服务功能。
use openlark_core::config::Config;

/// CCM Sheet 服务
#[derive(Clone)]
pub struct CcmSheetService {
    config: Config,
}

impl CcmSheetService {
    /// 创建新的 CCM Sheet 服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl openlark_core::trait_system::service::Service for CcmSheetService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ccmsheet"
    }
}

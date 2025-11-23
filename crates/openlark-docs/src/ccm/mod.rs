//! 云文档协同服务模块
//!
//! 提供文档、表格、知识库等协同编辑功能。

use openlark_core::config::Config;

/// 云文档协同服务
#[derive(Debug, Clone)]
pub struct CcmService {
    config: Config,
}

impl CcmService {
    /// 创建新的云文档协同服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 暂时简化sheets v3模块
pub mod sheets {
    use super::Config;

    pub mod v3 {
        use super::Config;

        pub struct V3 {
            config: Config,
        }

        impl V3 {
            pub fn new(config: Config) -> Self {
                Self { config }
            }

            pub fn config(&self) -> &Config {
                &self.config
            }
        }
    }
}

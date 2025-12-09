//! 云内容管理(ccm)模块
//!
//! 包含docs和docx两个子项目的API实现

use openlark_core::config::Config;

/// 云内容管理服务
#[derive(Debug, Clone)]
pub struct CcmService {
    config: Config,
}

impl CcmService {
    /// 创建新的云内容管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 导出docs和docx模块
pub mod docs;
pub mod docx;

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

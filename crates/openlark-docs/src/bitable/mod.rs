//! 多维表格服务模块
//!
//! 提供多维表格应用、数据表、视图管理等功能。

use openlark_core::config::Config;

/// 多维表格服务
#[derive(Debug, Clone)]
pub struct BitableService {
    config: Config,
}

impl BitableService {
    /// 创建新的多维表格服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 暂时简化v1模块
pub mod v1 {
    use super::Config;

    pub struct V1 {
        config: Config,
    }

    impl V1 {
        pub fn new(config: Config) -> Self {
            Self { config }
        }

        pub fn config(&self) -> &Config {
            &self.config
        }
    }
}

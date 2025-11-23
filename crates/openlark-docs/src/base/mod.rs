//! 基础服务模块
//!
//! 提供文件存储、基础操作等功能，包括文件上传、下载、云盘管理等。

use openlark_core::config::Config;

/// 基础服务
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
}

// 暂时简化v2模块
pub mod v2 {
    use super::Config;

    pub struct V2 {
        config: Config,
    }

    impl V2 {
        pub fn new(config: Config) -> Self {
            Self { config }
        }

        pub fn config(&self) -> &Config {
            &self.config
        }
    }
}

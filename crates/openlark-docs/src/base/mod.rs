/// 基础服务模块
///
/// 提供文件存储、基础操作等功能，包括文件上传、下载、云盘管理等。
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

    /// 获取V2版本服务
    pub fn v2(&self) -> v2::BaseV2Service {
        v2::BaseV2Service::new(self.config.clone())
    }
}

// 基础服务v2版本模块

// 导入v2版本的完整实现
pub mod v2;

pub use v2::*;

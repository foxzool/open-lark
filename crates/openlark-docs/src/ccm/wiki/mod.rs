/// Wiki 知识库服务模块
///
/// 提供企业知识库和Wiki管理功能。
use openlark_core::config::Config;

pub mod v1;
pub mod v2;

// 导出数据模型和请求类型
pub use v1::*;
pub use v2::*;

// 导出 V1 和 V2 服务
pub use v1::WikiV1Service;
pub use v2::WikiV2Service;

/// Wiki 知识库服务
///
/// 提供统一的 Wiki 知识库服务入口，支持多版本 API 访问。
#[derive(Debug, Clone)]
pub struct WikiService {
    config: Config,
}

impl WikiService {
    /// 创建新的 Wiki 服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 Wiki V1 服务
    pub fn v1(&self) -> WikiV1Service {
        WikiV1Service::new(self.config.clone())
    }

    /// 获取 Wiki V2 服务
    pub fn v2(&self) -> WikiV2Service {
        WikiV2Service::new(self.config.clone())
    }
}

//! Drive V2 API模块
//!
//! 提供Drive V2版本的API功能框架

use openlark_core::config::Config;

/// Drive V2 API服务
#[derive(Clone, Debug)]
pub struct DriveV2Service {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
}

impl DriveV2Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

// 注意：子模块暂时被禁用，因为需要进一步开发
// - explorer: 文件浏览器功能

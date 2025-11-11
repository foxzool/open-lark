// sessions - 会话管理服务
// 提供会话管理相关的功能
use config::Config;

/// 会话管理服务
#[derive(Debug, Clone)]
pub struct SessionsService {
    config: Config,
}

impl SessionsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

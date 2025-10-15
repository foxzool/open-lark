use crate::core::config::Config;

pub mod user;

/// 搜索服务 v1 版本
///
/// 提供基础搜索功能，包括用户搜索等核心搜索能力。
/// v1版本专注于简单易用的搜索体验。
pub struct V1 {
    /// 用户搜索服务
    pub user: user::UserService,
    /// 客户端配置
    config: Config,
}

impl V1 {
    /// 创建新的v1搜索服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的v1搜索服务实例
    pub fn new(config: Config) -> Self {
        Self {
            user: user::UserService::new(config.clone()),
            config: config,
        }
    }

    /// 获取客户端配置
    ///
    /// # 返回值
    /// 配置对象的引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}
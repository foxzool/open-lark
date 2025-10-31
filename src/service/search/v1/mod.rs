//! 搜索服务 V1 模块
//!
//! 提供搜索服务的V1版本API实现，包括用户搜索、文档搜索等功能。
//! 支持企业级的搜索需求，具备高性能和可扩展性。

use crate::core::config::Config;

// 导入用户搜索模块
pub mod user;

// 重新导出用户搜索相关的类型
pub use user::*;

/// 搜索服务 V1
///
/// 提供搜索服务的统一入口，整合了各种搜索功能。
#[derive(Debug, Clone)]
pub struct SearchV1Service {
    pub config: Config,
    /// 用户搜索服务
    pub user: UserService,
}

impl SearchV1Service {
    /// 创建搜索服务V1实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::search::v1::SearchV1Service;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = SearchV1Service::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            user: UserService::new(config.clone()),
            config,
        }
    }

    /// 获取客户端配置
    ///
    /// # 返回值
    ///
    /// 返回配置对象的引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

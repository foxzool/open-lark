/// Lingo草稿管理模块
use openlark_core::config::Config;

// 导出API实现
pub mod create;
pub mod update;

// 重新导出
// create 模块显式导出
pub use create::{
    CreateDraftRequest,
    CreateDraftResp,
    UpdateDraftRequest,
    UpdateDraftResp,
    execute,
    execute_with_options,
    new,
    repo_id,
    user_id_type,
};
// update 模块显式导出
pub use update::{
    CreateDraftRequest,
    CreateDraftResp,
    UpdateDraftRequest,
    UpdateDraftResp,
    execute,
    execute_with_options,
    new,
    repo_id,
    user_id_type,
};

/// Lingo草稿服务
#[derive(Debug, Clone)]
pub struct LingoDraftService {
    config: Config,
}

impl LingoDraftService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

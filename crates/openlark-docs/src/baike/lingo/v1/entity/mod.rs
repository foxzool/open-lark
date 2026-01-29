use openlark_core::config::Config;

pub mod create;
pub mod delete;
pub mod get;
pub mod highlight;
pub mod list;
pub mod r#match;
pub mod search;
pub mod update;

pub use create::{CreateEntityRequest, CreateEntityResp};
pub use delete::{DeleteEntityRequest, DeleteEntityResp};
pub use get::{GetEntityRequest, GetEntityResp};
pub use highlight::{HighlightEntityBody, HighlightEntityRequest, HighlightEntityResp, Phrase, Span};
pub use list::{ListEntityRequest, ListEntityResp};
pub use r#match::{MatchEntityBody, MatchEntityRequest, MatchEntityResp, MatchInfo, TermType};
pub use search::{ClassificationFilter, SearchEntityBody, SearchEntityRequest, SearchEntityResp};
pub use update::{UpdateEntityRequest, UpdateEntityResp};

/// Lingo词条管理服务
#[derive(Debug, Clone)]
pub struct LingoEntityService {
    config: Config,
}

impl LingoEntityService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Default for LingoEntityService {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

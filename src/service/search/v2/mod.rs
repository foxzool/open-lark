pub mod data_item;
pub mod data_source;
pub mod models;
pub mod schema;
pub mod suite_search;

use std::sync::Arc;

use crate::core::config::Config;

/// Search API v2版本服务
pub struct V2 {
    /// 套件搜索服务
    pub suite_search: suite_search::SuiteSearchService,
    /// 数据源服务
    pub data_source: data_source::DataSourceService,
    /// 数据项服务
    pub data_item: data_item::DataItemService,
    /// 数据范式服务
    pub schema: schema::SchemaService,
}

impl V2 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            suite_search: suite_search::SuiteSearchService::new(Arc::clone(&config)),
            data_source: data_source::DataSourceService::new(Arc::clone(&config)),
            data_item: data_item::DataItemService::new(Arc::clone(&config)),
            schema: schema::SchemaService::new(config),
        }
    }
}

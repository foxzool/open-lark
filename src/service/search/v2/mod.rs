pub mod data_item;
pub mod data_source;
pub mod models;
pub mod schema;
pub mod suite_search;

use crate::core::config::Config;

/// Search API v2版本服务
///
/// 提供增强版搜索功能，包括自定义数据源、高级搜索语法、
/// 多维度结果排序等企业级搜索能力。
pub struct V2 {
    /// 套件搜索服务
    pub suite_search: suite_search::SuiteSearchService,
    /// 数据源服务
    pub data_source: data_source::DataSourceService,
    /// 数据项服务
    pub data_item: data_item::DataItemService,
    /// 数据范式服务
    pub schema: schema::SchemaService,
    /// 客户端配置
    config: Config,
}

impl V2 {
    /// 创建新的v2搜索服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的v2搜索服务实例
    pub fn new(config: Config) -> Self {
        Self {
            suite_search: suite_search::SuiteSearchService::new(config.clone()),
            data_source: data_source::DataSourceService::new(config.clone()),
            data_item: data_item::DataItemService::new(config.clone()),
            schema: schema::SchemaService::new(config.clone()),
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

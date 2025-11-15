//! Bitable V2 API模块
//!
//! 提供多维表格 V2 版本的接口，包括高级功能和性能优化的接口：
//! - 高级搜索和过滤功能
//! - 增强的批量操作
//! - 优化的数据查询接口
//!
//! V2 API 提供了比 V1 更强大的功能和更好的性能表现。

use openlark_core::{
    config::Config,
};

// 导入子模块
pub mod advanced_search;
pub mod batch_operations;
pub mod enhanced_query;

// 重新导出主要类型
pub use advanced_search::{AdvancedSearchService, AdvancedSearchRequestBuilder};
pub use batch_operations::{BatchOperationsService, BulkOperationRequestBuilder};
pub use enhanced_query::{EnhancedQueryService, SmartQueryRequestBuilder};

/// Bitable V2 服务
#[derive(Debug, Clone)]
pub struct BitableV2Service {
    config: Config,
}

impl BitableV2Service {
    /// 创建 Bitable V2 服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取高级搜索服务
    pub fn advanced_search(&self) -> advanced_search::AdvancedSearchService {
        advanced_search::AdvancedSearchService::new(self.config.clone())
    }

    /// 获取批量操作服务
    pub fn batch_operations(&self) -> batch_operations::BatchOperationsService {
        batch_operations::BatchOperationsService::new(self.config.clone())
    }

    /// 获取增强查询服务
    pub fn enhanced_query(&self) -> enhanced_query::EnhancedQueryService {
        enhanced_query::EnhancedQueryService::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2_service_creation() {
        let config = Config::default();
        let service = BitableV2Service::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_v2_service_submodules() {
        let config = Config::default();
        let service = BitableV2Service::new(config);

        // 测试子模块访问
        let _search_service = service.advanced_search();
        let _batch_service = service.batch_operations();
        let _query_service = service.enhanced_query();
    }
}
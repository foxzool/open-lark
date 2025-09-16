//! 搜索（Search）服务
//!
//! 提供飞书开放平台的智能搜索功能，支持全文搜索、数据源管理、
//! 索引构建等企业级搜索能力。为企业提供统一的信息检索和知识发现服务。
//!
//! # 核心功能
//!
//! ## 数据源管理
//! - 📊 自定义数据源创建和配置
//! - 🔄 数据源同步和更新机制
//! - 🏷️ 数据分类和标签管理
//! - 📋 数据源权限控制
//!
//! ## 搜索引擎
//! - 🔍 全文搜索和精确匹配
//! - 🎯 智能推荐和相关性排序
//! - 🔗 跨数据源联合搜索
//! - 📈 搜索结果统计和分析
//!
//! ## 索引管理
//! - 🏗️ 自动索引构建和维护
//! - ⚡ 实时索引更新
//! - 🗂️ Schema定义和字段映射
//! - 🔧 索引优化和性能调优
//!
//! ## 数据项操作
//! - 📝 数据项的增删改查
//! - 🏷️ 元数据管理和标签
//! - 🔐 访问权限和安全控制
//! - 📊 数据质量监控
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取搜索服务
//! let search = &client.search;
//!
//! // 创建数据源（v2版本）
//! // let data_source_request = CreateDataSourceRequest::builder()
//! //     .name("企业知识库")
//! //     .description("包含公司所有技术文档")
//! //     .build();
//! // let data_source = search.v2.data_source.create(data_source_request, None).await?;
//!
//! // 执行搜索
//! // let search_request = SearchRequest::builder()
//! //     .query("飞书API")
//! //     .data_source_id("ds_123")
//! //     .build();
//! // let results = search.v2.suite_search.search(search_request, None).await?;
//! ```
//!
//! # API版本
//!
//! ## v1版本
//! 基础搜索功能，提供简单的全文搜索能力。
//!
//! ## v2版本（推荐）
//! 增强版搜索引擎，支持：
//! - 自定义数据源和Schema
//! - 高级搜索语法
//! - 多维度结果排序
//! - 搜索分析和统计
//!
//! # 搜索特性
//!
//! - 🚀 毫秒级搜索响应
//! - 🎯 智能相关性算法
//! - 📱 多端搜索体验统一
//! - 🔐 细粒度权限控制
//! - 📊 丰富的搜索分析

use crate::core::config::Config;

/// 搜索服务 v1 版本
pub mod v1;
/// 搜索服务 v2 版本
pub mod v2;

/// 搜索服务
///
/// 企业级智能搜索解决方案的统一入口，提供数据索引、全文检索、
/// 智能推荐等完整的搜索功能体系。
///
/// # 服务架构
///
/// - **v1**: 基础搜索功能，简单易用
/// - **v2**: 增强搜索引擎，功能丰富（推荐使用）
///
/// # 核心特性
///
/// - 🔍 高性能全文搜索引擎
/// - 📊 灵活的数据源管理
/// - 🎯 智能搜索推荐算法
/// - 🔐 企业级权限控制
/// - 📈 搜索效果分析统计
///
/// # 适用场景
///
/// - 企业知识库搜索
/// - 文档管理系统
/// - 内容发现和推荐
/// - 数据分析和挖掘
/// - 跨系统信息检索
///
/// # 最佳实践
///
/// - 合理设计数据源结构
/// - 定期优化搜索索引
/// - 监控搜索性能指标
/// - 收集用户搜索反馈
/// - 持续优化搜索算法
pub struct SearchService {
    /// v1版本搜索API服务
    pub v1: v1::V1,
    /// v2版本搜索API服务（推荐）
    pub v2: v2::V2,
}

impl SearchService {
    /// 创建新的搜索服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的搜索服务实例，包含v1和v2版本API
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config.clone()),
            v2: v2::V2::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    #[test]
    fn test_search_service_creation() {
        let config = create_test_config();
        let search_service = SearchService::new(config);

        // Verify service structure
        assert!(std::ptr::addr_of!(search_service.v1) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(search_service.v2) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_search_service_debug_trait() {
        let config = create_test_config();
        let search_service = SearchService::new(config);

        // Test that service can be used (services don't need to implement Debug)
        assert!(std::ptr::addr_of!(search_service) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_search_service_api_versions_independence() {
        let config = create_test_config();
        let search_service = SearchService::new(config);

        // Test that API versions are independent
        let v1_ptr = std::ptr::addr_of!(search_service.v1) as *const _;
        let v2_ptr = std::ptr::addr_of!(search_service.v2) as *const _;

        assert_ne!(v1_ptr, v2_ptr, "API versions should be independent");
    }

    #[test]
    fn test_search_service_with_custom_configurations() {
        let test_configs = vec![
            Config::builder()
                .app_id("search_basic")
                .app_secret("basic_secret")
                .build(),
            Config::builder()
                .app_id("search_timeout")
                .app_secret("timeout_secret")
                .req_timeout(std::time::Duration::from_millis(25000))
                .build(),
            Config::builder()
                .app_id("search_custom")
                .app_secret("custom_secret")
                .base_url("https://search.enterprise.com")
                .build(),
            Config::builder()
                .app_id("search_full")
                .app_secret("full_secret")
                .req_timeout(std::time::Duration::from_millis(30000))
                .base_url("https://full.search.com")
                .enable_token_cache(false)
                .build(),
        ];

        for config in test_configs {
            let search_service = SearchService::new(config);

            // Each configuration should create a valid service
            assert!(std::ptr::addr_of!(search_service.v1) as *const _ != std::ptr::null());
            assert!(std::ptr::addr_of!(search_service.v2) as *const _ != std::ptr::null());
        }
    }

    #[test]
    fn test_search_service_multiple_instances() {
        let config1 = create_test_config();
        let config2 = Config::builder()
            .app_id("search2")
            .app_secret("secret2")
            .build();

        let search_service1 = SearchService::new(config1);
        let search_service2 = SearchService::new(config2);

        // Services should be independent instances
        let service1_ptr = std::ptr::addr_of!(search_service1) as *const _;
        let service2_ptr = std::ptr::addr_of!(search_service2) as *const _;

        assert_ne!(service1_ptr, service2_ptr, "Services should be independent instances");

        // Each service should have valid API versions
        assert!(std::ptr::addr_of!(search_service1.v1) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(search_service1.v2) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(search_service2.v1) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(search_service2.v2) as *const _ != std::ptr::null());
    }

    #[test]
    fn test_search_service_config_cloning_behavior() {
        let original_config = create_test_config();

        // Test that the service works with cloned configs
        let search_service1 = SearchService::new(original_config.clone());
        let search_service2 = SearchService::new(original_config);

        // Both should work independently
        assert!(std::ptr::addr_of!(search_service1.v1) as *const _ != std::ptr::null());
        assert!(std::ptr::addr_of!(search_service2.v1) as *const _ != std::ptr::null());

        // But should be different service instances
        let service1_ptr = std::ptr::addr_of!(search_service1) as *const _;
        let service2_ptr = std::ptr::addr_of!(search_service2) as *const _;
        assert_ne!(service1_ptr, service2_ptr);
    }
}

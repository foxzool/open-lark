//! Bitable V2 API模块
//!
//! 提供多维表格 V2 版本的接口，包括高级功能和性能优化的接口：
//! - 高级搜索和过滤功能
//! - 增强的批量操作
//! - 优化的数据查询接口
//!
//! V2 API 提供了比 V1 更强大的功能和更好的性能表现。

use openlark_core::config::Config;

// 导入子模块
pub mod advanced_search;
pub mod batch_operations;
pub mod enhanced_query;
pub mod role_builders;
pub mod role_management;

// 重新导出主要类型
pub use advanced_search::{AdvancedSearchRequestBuilder, AdvancedSearchService};
pub use batch_operations::{BatchOperationsService, BulkOperationRequestBuilder};
pub use enhanced_query::{EnhancedQueryService, SmartQueryRequestBuilder};
pub use role_builders::{
    CreateRoleV2RequestBuilder, ListRolesV2RequestBuilder, UpdateRoleV2RequestBuilder,
};
pub use role_management::{
    CreateRoleV2Request, CreateRoleV2Response, ListRolesV2Response, RoleManagementV2Service,
    RoleV2, UpdateRoleV2Request, UpdateRoleV2Response,
};

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

    /// 获取角色管理 V2 服务
    pub fn role_management(&self) -> role_management::RoleManagementV2Service {
        role_management::RoleManagementV2Service::new(self.config.clone())
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
        let _role_service = service.role_management();
    }

    #[test]
    fn test_role_v2_creation() {
        let role = RoleV2 {
            role_id: "role_123".to_string(),
            name: "测试角色".to_string(),
            description: Some("这是一个测试角色".to_string()),
            permissions: vec!["read".to_string(), "write".to_string()],
            create_time: Some(1234567890),
            update_time: Some(1234567891),
        };

        assert_eq!(role.role_id, "role_123");
        assert_eq!(role.name, "测试角色");
        assert_eq!(role.permissions.len(), 2);
    }

    #[test]
    fn test_create_role_v2_request_validation() {
        let valid_request = CreateRoleV2Request {
            name: "编辑者".to_string(),
            description: Some("可以编辑数据的角色".to_string()),
            permissions: vec!["table:read".to_string(), "table:write".to_string()],
        };

        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateRoleV2Request {
            name: "".to_string(),
            description: None,
            permissions: vec![],
        };

        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_update_role_v2_request_validation() {
        let valid_request = UpdateRoleV2Request {
            name: Some("高级编辑者".to_string()),
            description: Some("具有高级权限".to_string()),
            permissions: vec![
                "table:read".to_string(),
                "table:write".to_string(),
                "table:delete".to_string(),
            ],
        };

        assert!(valid_request.validate().is_ok());

        let invalid_request = UpdateRoleV2Request {
            name: Some("".to_string()),
            description: None,
            permissions: vec![],
        };

        assert!(invalid_request.validate().is_err());
    }
}

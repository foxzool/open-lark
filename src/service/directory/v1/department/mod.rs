use config::Config;
use crate::service_trait::Service;
use crate::transport::Transport;
use std::sync::Arc;

pub mod create;
pub mod delete;
pub mod filter;
pub mod mget;
pub mod patch;
pub mod search;

// 重新导出所有请求和响应类型
pub use create::*;
pub use delete::*;
pub use filter::*;
pub use mget::*;
pub use patch::*;
pub use search::*;

/// 部门管理服务
///
/// 提供部门的创建、更新、删除、查询等管理功能
#[derive(Debug, Clone)]
pub struct DepartmentService {
    config: Config,
    transport: Arc<dyn Transport>,
}

impl DepartmentService {
    /// 创建新的部门管理服务
    ///
    /// # 参数
    /// * `config` - 配置信息
    /// * `transport` - 传输层实现
    pub fn new(config: Config, transport: Arc<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// 批量获取部门信息构建器
    ///
    /// 创建一个批量获取部门信息的构建器，支持链式调用和完整的错误处理
    ///
    /// # 参数
    /// * `request` - 批量获取部门信息请求，包含部门ID列表等参数
    ///
    /// # 返回
    /// 返回批量获取部门信息构建器，可用于执行查询操作
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::directory::v1::department::mget::{MGetDepartmentRequest, MGetDepartmentResponse};
    ///
    /// async fn mget_department_example(
    ///     service: Arc<DepartmentService>,
    /// ) -> Result<MGetDepartmentResponse, Box<dyn std::error::Error>> {
    ///     let request = MGetDepartmentRequest::builder()
    ///         .department_ids(vec!["od_123".to_string(), "od_456".to_string()])
    ///         .user_id_type("open_id")
    ///         .department_id_type("open_department_id")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .mget_department_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("成功获取{}个部门信息", response.department_count());
    ///     Ok(response)
    /// }
    /// ```
    pub fn mget_department_builder(&self, request: MGetDepartmentRequest) -> MGetDepartmentBuilder {
        MGetDepartmentBuilder::new(Arc::new(self.clone()), request)
    }

    /// 获取部门列表构建器
    ///
    /// 创建一个获取部门列表的构建器，支持链式调用和完整的错误处理
    ///
    /// # 参数
    /// * `request` - 获取部门列表请求，包含分页和过滤参数
    ///
    /// # 返回
    /// 返回获取部门列表构建器，可用于执行查询操作
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::directory::v1::department::filter::{FilterDepartmentRequest, FilterDepartmentResponse};
    ///
    /// async fn filter_department_example(
    ///     service: Arc<DepartmentService>,
    /// ) -> Result<FilterDepartmentResponse, Box<dyn std::error::Error>> {
    ///     let request = FilterDepartmentRequest::builder()
    ///         .page_size(20)
    ///         .parent_department_id("od_parent")
    ///         .user_id_type("open_id")
    ///         .department_id_type("open_department_id")
    ///         .department_status("active")
    ///         .sort_field("name")
    ///         .sort_direction("asc")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .filter_department_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("成功获取{}个部门信息", response.department_count());
    ///     Ok(response)
    /// }
    /// ```
    pub fn filter_department_builder(&self, request: FilterDepartmentRequest) -> FilterDepartmentBuilder {
        FilterDepartmentBuilder::new(Arc::new(self.clone()), request)
    }
}

impl Service for DepartmentService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn transport(&self) -> &dyn Transport {
        self.transport.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;

    fn create_test_service() -> Arc<DepartmentService> {
        let config = Config::new("test_app_id", "test_app_secret");
        let transport = Arc::new(MockTransport::new());
        Arc::new(DepartmentService::new(config, transport))
    }

    #[test]
    fn test_department_service_creation() {
        let config = Config::new("test_app_id", "test_app_secret");
        let transport = Arc::new(MockTransport::new());
        let service = DepartmentService::new(config, transport);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[tokio::test]
    async fn test_mget_department_builder() {
        let service = create_test_service();
        let request = MGetDepartmentRequest::builder()
            .department_ids(vec!["od_123".to_string(), "od_456".to_string()])
            .user_id_type("open_id")
            .department_id_type("open_department_id")
            .build()
            .unwrap();

        let builder = service.mget_department_builder(request);
        // 验证构建器创建成功，实际网络请求需要mock配置
        assert_eq!(builder.request.department_ids.len(), 2);
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
        assert_eq!(builder.request.department_id_type, Some("open_department_id".to_string()));
    }

    #[tokio::test]
    async fn test_filter_department_builder() {
        let service = create_test_service();
        let request = FilterDepartmentRequest::builder()
            .page_size(20)
            .parent_department_id("od_parent")
            .user_id_type("open_id")
            .department_id_type("open_department_id")
            .department_status("active")
            .sort_field("name")
            .sort_direction("asc")
            .build()
            .unwrap();

        let builder = service.filter_department_builder(request);
        // 验证构建器创建成功，实际网络请求需要mock配置
        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.parent_department_id, Some("od_parent".to_string()));
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
        assert_eq!(builder.request.department_id_type, Some("open_department_id".to_string()));
        assert_eq!(builder.request.department_status, Some("active".to_string()));
        assert_eq!(builder.request.sort_field, Some("name".to_string()));
        assert_eq!(builder.request.sort_direction, Some("asc".to_string()));
    }

    #[test]
    fn test_multiple_department_services() {
        let config1 = Config::new("app_id_1", "app_secret_1");
        let config2 = Config::new("app_id_2", "app_secret_2");
        let transport = Arc::new(MockTransport::new());

        let service1 = DepartmentService::new(config1.clone(), transport.clone());
        let service2 = DepartmentService::new(config2, transport);

        assert_eq!(service1.config().app_id(), "app_id_1");
        assert_eq!(service2.config().app_id(), "app_id_2");
        // 验证两个服务实例是独立的
        assert_ne!(service1.config().app_id(), service2.config().app_id());
    }

    #[test]
    fn test_department_service_clone() {
        let config = Config::new("test_app_id", "test_app_secret");
        let transport = Arc::new(MockTransport::new());
        let service = DepartmentService::new(config, transport);
        let cloned_service = service.clone();

        assert_eq!(service.config().app_id(), cloned_service.config().app_id());
        assert_eq!(service.config().app_secret(), cloned_service.config().app_secret());
    }

    #[test]
    fn test_service_trait_implementation() {
        let config = Config::new("test_app_id", "test_app_secret");
        let transport = Arc::new(MockTransport::new());
        let service = DepartmentService::new(config, transport);

        // 测试Service trait的实现
        let config_ref: &Config = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");

        let transport_ref: &dyn Transport = service.transport();
        // Transport trait测试需要具体的mock实现
        assert!(!std::ptr::eq(transport_ref, std::ptr::null()));
    }

    #[test]
    fn test_module_reexports() {
        // 验证所有模块都正确重新导出
        // 这个测试主要是编译时检查，确保所有公共类型都可以访问
        let _request: MGetDepartmentRequest = MGetDepartmentRequest::new(vec![]);
        let _response: MGetDepartmentResponse = MGetDepartmentResponse::new(vec![]);

        // 如果编译通过，说明所有导出都正确
        assert!(true);
    }
}
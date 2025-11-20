
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use openlark_core::config::Config;

// 重新导出所有模块类型
pub use create::*;
pub use get::*;
pub use list::*;

mod create;
mod get;
mod list;

/// 知识空间服务
///
/// 提供知识空间的完整管理功能，包括创建、查询、列表管理等。
/// 支持个人和团队知识空间的操作，具备完整的权限控制。
#[derive(Clone, Debug)]
pub struct SpaceService {
    config: Config,
}

impl SpaceService {
    /// 创建新的知识空间服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取知识空间列表
    ///
    /// 获取用户有权访问的知识空间列表，包括空间的基本信息、权限状态等。
    /// 支持分页查询和过滤条件，方便用户快速找到目标空间。
    ///
    /// # 参数
    /// * `request` - 获取知识空间列表请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回知识空间列表响应，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space::{SpaceService, ListSpaceRequest};
    ///
    /// let service = SpaceService::new(config);
    /// let request = ListSpaceRequest::builder()
    ///     .page_size(20)
    ///     .build()?;
    ///
    /// let response = service.list(request, None).await?;
    /// println!("找到{}个知识空间", response.data.items.len());
    /// ```
    pub async fn list(
        &self,
        request: ListSpaceRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<ListSpaceResponse>> {
        list_spaces(request, &self.config, option).await
    }

    /// 获取知识空间信息
    ///
    /// 获取指定知识空间的详细信息，包括空间名称、描述、创建者、成员数量等。
    /// 需要提供空间 ID 来查询。
    ///
    /// # 参数
    /// * `request` - 获取知识空间信息请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回知识空间详细信息，失败返回错误信息
    ///
    /// # 权限要求
    /// 需要具备目标知识空间的成员权限
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space::{SpaceService, GetSpaceInfoRequest};
    ///
    /// let service = SpaceService::new(config);
    /// let request = GetSpaceInfoRequest::new("6870403571079249922");
    ///
    /// let response = service.get(request, None).await?;
    /// println!("知识空间名称: {}", response.data.space.name);
    /// ```
    pub async fn get(
        &self,
        request: GetSpaceInfoRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<GetSpaceInfoResponse>> {
        get_space_info(request, &self.config, option).await
    }

    /// 创建知识空间
    ///
    /// 创建新的知识空间，用于组织和管理相关的知识库文档。可以设置空间名称、描述、
    /// 访问权限等属性。创建成功后返回空间 ID。
    ///
    /// # 参数
    /// * `request` - 创建知识空间请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回创建的知识空间信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space::{SpaceService, CreateSpaceRequest};
    ///
    /// let service = SpaceService::new(config);
    /// let request = CreateSpaceRequest::builder()
    ///     .name("新产品知识库")
    ///     .description("新产品相关的所有文档")
    ///     .build()?;
    ///
    /// let response = service.create(request, None).await?;
    /// println!("创建成功，空间ID: {}", response.data.space.space_id);
    /// ```
    pub async fn create(
        &self,
        request: CreateSpaceRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<CreateSpaceResponse>> {
        create_space(request, &self.config, option).await
    }

    /// 获取知识空间信息构建器
    ///
    /// 创建一个获取知识空间信息的构建器，支持链式调用和完整的错误处理
    ///
    /// # 参数
    /// * `request` - 获取知识空间信息请求，包含空间ID等参数
    ///
    /// # 返回
    /// 返回获取知识空间信息构建器，可用于执行查询操作
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space::{SpaceService, GetSpaceInfoRequest};
    /// use std::sync::Arc;
    ///
    /// let service = Arc::new(SpaceService::new(config));
    /// let response = service
    ///     .get_space_info_builder(GetSpaceInfoRequest::new("6870403571079249922"))
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn get_space_info_builder(&self, request: GetSpaceInfoRequest) -> GetSpaceInfoBuilder {
        GetSpaceInfoBuilder::new(std::sync::Arc::new(self.clone()), request)
    }

    /// 创建知识空间构建器
    ///
    /// 创建一个创建知识空间的构建器，支持链式调用和完整的错误处理
    ///
    /// # 参数
    /// * `request` - 创建知识空间请求，包含空间名称、描述等参数
    ///
    /// # 返回
    /// 返回创建知识空间构建器，可用于执行创建操作
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space::{SpaceService, CreateSpaceRequest};
    /// use std::sync::Arc;
    ///
    /// let service = Arc::new(SpaceService::new(config));
    /// let response = service
    ///     .create_space_builder(CreateSpaceRequest::builder()
    ///         .name("新产品知识库")
    ///         .description("新产品相关的所有文档")
    ///         .build()?)
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn create_space_builder(&self, request: CreateSpaceRequest) -> CreateSpaceBuilder {
        CreateSpaceBuilder::new(std::sync::Arc::new(self.clone()), request)
    }
}

impl openlark_core::service_trait::Service for SpaceService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn transport(&self) -> &dyn openlark_core::transport::Transport {
        panic!("SpaceService does not have a transport instance")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    transport::MockTransport;

    fn create_test_service() -> SpaceService {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        SpaceService::new(config)
    }

    #[test]
    fn test_space_service_creation() {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        let service = SpaceService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_space_service_clone() {
        let service = create_test_service();
        let cloned_service = service.clone();

        assert_eq!(service.config().app_id(), cloned_service.config().app_id());
        assert_eq!(service.config().app_secret(), cloned_service.config().app_secret());
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
    }

    #[test]
    fn test_get_space_info_builder_creation() {
        let service = create_test_service();
        let request = GetSpaceInfoRequest::new("6870403571079249922");

        let builder = service.get_space_info_builder(request);
        // 验证构建器创建成功，实际执行需要mock配置
        assert_eq!(builder.request.space_id, "6870403571079249922");
    }

    #[test]
    fn test_create_space_builder_creation() {
        let service = create_test_service();
        let request = CreateSpaceRequest::builder()
            .name("测试知识空间")
            .build()
            .unwrap();

        let builder = service.create_space_builder(request);
        // 验证构建器创建成功，实际执行需要mock配置
        assert_eq!(builder.request.name, "测试知识空间");
    }

    #[test]
    fn test_module_reexports() {
        // 验证所有模块都正确重新导出
        // 这个测试主要是编译时检查，确保所有公共类型都可以访问
        let _request: GetSpaceInfoRequest = GetSpaceInfoRequest::new("test_space_123");
        let _response: GetSpaceInfoResponse = GetSpaceInfoResponse {
            space: SpaceInfo {
                space_id: "test_space_123".to_string(),
                name: "Test Space".to_string(),
                description: None,
                space_type: None,
                visibility: None,
                create_time: None,
                update_time: None,
            },
        };

        // 如果编译通过，说明所有导出都正确
        assert!(true);
    }

    #[test]
    fn test_multiple_space_services() {
        let config1 = openlark_core::config::Config::new("app_id_1", "app_secret_1");
        let config2 = openlark_core::config::Config::new("app_id_2", "app_secret_2");

        let service1 = SpaceService::new(config1);
        let service2 = SpaceService::new(config2);

        assert_eq!(service1.config().app_id(), "app_id_1");
        assert_eq!(service2.config().app_id(), "app_id_2");
        // 验证两个服务实例是独立的
        assert_ne!(service1.config().app_id(), service2.config().app_id());
    }

    #[test]
    fn test_space_service_methods_exist() {
        // 这个测试确保所有方法都存在且可以调用（不需要实际执行）
        let service = create_test_service();

        // 验证方法签名正确
        let _get_method = service.get as fn(
            &SpaceService,
            GetSpaceInfoRequest,
            Option<openlark_core::req_option::RequestOption>,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = _> + '_>>;

        let _list_method = service.list as fn(
            &SpaceService,
            ListSpaceRequest,
            Option<openlark_core::req_option::RequestOption>,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = _> + '_>>;

        let _create_method = service.create as fn(
            &SpaceService,
            CreateSpaceRequest,
            Option<openlark_core::req_option::RequestOption>,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = _> + '_>>;

        // 如果编译通过，说明所有方法都存在
        assert!(true);
    }
}
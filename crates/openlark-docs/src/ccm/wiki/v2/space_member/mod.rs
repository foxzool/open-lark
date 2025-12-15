#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// 知识空间成员管理模块
///
/// 提供知识空间成员的完整管理功能，包括添加、删除、查询等。

use openlark_core::config::Config;
// 重新导出所有模块类型
pub use create::*;
pub use delete::*;
pub use list::*;

mod create;
mod delete;
mod list;

/// 知识空间成员服务
///
/// 提供知识空间成员的完整管理功能，包括成员添加、删除、列表查询等。
/// 支持不同权限角色的管理，具备完整的权限控制机制。
#[derive(Clone)]
pub struct SpaceMemberService {
    config: Config,
}

impl SpaceMemberService {
    /// 创建新的知识空间成员服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取知识空间成员列表
    ///
    /// 获取指定知识空间的成员列表，包括成员信息、权限角色、状态等。
    /// 支持分页查询，方便管理大量成员。
    ///
    /// # 参数
    /// * `request` - 获取知识空间成员列表请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回知识空间成员列表响应，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space_member::{SpaceMemberService, ListSpaceMembersRequest};
    ///
    /// let service = SpaceMemberService::new(config);
    /// let request = ListSpaceMembersRequest {
    ///     space_id: "6870403571079249922".to_string(),
    ///     page_size: Some(20),
    ///     page_token: None,
    /// };
    ///
    /// let response = service.list(request, None).await?;
    /// println!("找到{}个成员", response.data.unwrap().items.len());
    /// ```
    pub async fn list(
        &self,
        request: ListSpaceMembersRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<ListSpaceMembersResponse>> {
        list_space_members(request, &self.config, option).await
    }

    /// 创建知识空间成员
    ///
    /// 向指定知识空间添加新成员，支持设置不同的权限角色。
    /// 成功添加后，成员将收到邀请通知。
    ///
    /// # 参数
    /// * `request` - 创建知识空间成员请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回创建的成员信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space_member::{SpaceMemberService, CreateSpaceMemberRequest};
    ///
    /// let service = SpaceMemberService::new(config);
    /// let request = CreateSpaceMemberRequest::new(
    ///     "6870403571079249922",
    ///     "user",
    ///     "ou_123456789",
    ///     "edit_member"
    /// );
    ///
    /// let response = service.create(request, None).await?;
    /// println!("成员添加成功，ID: {}", response.data.unwrap().member_id);
    /// ```
    pub async fn create(
        &self,
        request: CreateSpaceMemberRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<CreateSpaceMemberResponse>> {
        create_space_member(request, &self.config, option).await
    }

    /// 删除知识空间成员
    ///
    /// 从指定知识空间中删除成员，删除后成员将失去访问权限。
    /// 管理员无法删除自己。
    ///
    /// # 参数
    /// * `request` - 删除知识空间成员请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回删除结果，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space_member::{SpaceMemberService, DeleteSpaceMemberRequest};
    ///
    /// let service = SpaceMemberService::new(config);
    /// let request = DeleteSpaceMemberRequest::new(
    ///     "6870403571079249922",
    ///     "ou_123456789",
    ///     "user"
    /// );
    ///
    /// let response = service.delete(request, None).await?;
    /// println!("删除成功: {}", response.data.unwrap().deleted);
    /// ```
    pub async fn delete(
        &self,
        request: DeleteSpaceMemberRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<DeleteSpaceMemberResponse>> {
        delete_space_member(request, &self.config, option).await
    }
}

impl openlark_core::trait_system::service::Service for SpaceMemberService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "spacemember"
    }

    }

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> SpaceMemberService {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        SpaceMemberService::new(config)
    }

    #[test]
    fn test_space_member_service_creation() {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        let service = SpaceMemberService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_space_member_service_clone() {
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
    fn test_multiple_space_member_services() {
        let config1 = openlark_core::config::Config::new("app_id_1", "app_secret_1");
        let config2 = openlark_core::config::Config::new("app_id_2", "app_secret_2");

        let service1 = SpaceMemberService::new(config1);
        let service2 = SpaceMemberService::new(config2);

        assert_eq!(service1.config().app_id(), "app_id_1");
        assert_eq!(service2.config().app_id(), "app_id_2");
        // 验证两个服务实例是独立的
        assert_ne!(service1.config().app_id(), service2.config().app_id());
    }

    #[test]
    fn test_module_reexports() {
        // 验证所有模块都正确重新导出
        // 这个测试主要是编译时检查，确保所有公共类型都可以访问
        let _list_request: ListSpaceMembersRequest = ListSpaceMembersRequest {
            space_id: "test_space_123".to_string(),
            page_size: Some(20),
            page_token: None,
        };

        let _create_request: CreateSpaceMemberRequest = CreateSpaceMemberRequest::new(
            "test_space_123",
            "user",
            "test_member_123",
            "edit_member"
        );

        let _delete_request: DeleteSpaceMemberRequest = DeleteSpaceMemberRequest::new(
            "test_space_123",
            "test_member_123",
            "user"
        );

        // 如果编译通过，说明所有导出都正确
        assert!(true);
}
}

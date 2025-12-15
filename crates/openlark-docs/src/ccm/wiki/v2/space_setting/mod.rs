#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// 知识空间设置管理模块
///
/// 提供知识空间设置的查询和管理功能，包括权限设置等。

use openlark_core::config::Config;
use serde::{Deserialize, Serialize};

// 重新导出所有模块类型
pub use get::*;
pub use update::*;

mod get;
mod update;

/// 知识空间设置服务
///
/// 提供知识空间设置的完整管理功能，包括设置查询、更新等。
/// 支持评论权限、复制权限等功能的配置管理。
#[derive(Clone)]
pub struct SpaceSettingService {
    config: Config,
}

impl SpaceSettingService {
    /// 创建新的知识空间设置服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取知识空间设置
    ///
    /// 获取指定知识空间的详细设置信息，包括权限配置、功能开关等。
    /// 注意：基础设置信息通过 get space info 接口获取。
    ///
    /// # 参数
    /// * `request` - 获取知识空间设置请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回知识空间设置信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space_setting::{SpaceSettingService, GetSpaceSettingRequest};
    ///
    /// let service = SpaceSettingService::new(config);
    /// let request = GetSpaceSettingRequest::new("6870403571079249922");
    ///
    /// let response = service.get(request, None).await?;
    /// println!("知识空间名称: {}", response.data.unwrap().name);
    /// ```
    pub async fn get(
        &self,
        request: GetSpaceSettingRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<GetSpaceSettingResponse>> {
        get_space_setting(request, &self.config, option).await
    }

    /// 更新知识空间设置
    ///
    /// 更新指定知识空间的设置，如评论权限、复制权限等功能开关。
    /// 需要管理员权限才能修改设置。
    ///
    /// # 参数
    /// * `request` - 更新知识空间设置请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回更新后的设置信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space_setting::{SpaceSettingService, UpdateSpaceSettingRequest};
    ///
    /// let service = SpaceSettingService::new(config);
    /// let request = UpdateSpaceSettingRequest::new("6870403571079249922")
    ///     .comment_enabled(true)
    ///     .copy_enabled(false);
    ///
    /// let response = service.update(request, None).await?;
    /// println!("设置更新成功");
    /// ```
    pub async fn update(
        &self,
        request: UpdateSpaceSettingRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<UpdateSpaceSettingResponse>> {
        update_space_setting(request, &self.config, option).await
    }
}

impl openlark_core::trait_system::service::Service for SpaceSettingService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "spacesetting"
    }

    }

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> SpaceSettingService {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        SpaceSettingService::new(config)
    }

    #[test]
    fn test_space_setting_service_creation() {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        let service = SpaceSettingService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_space_setting_service_clone() {
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
    fn test_multiple_space_setting_services() {
        let config1 = openlark_core::config::Config::new("app_id_1", "app_secret_1");
        let config2 = openlark_core::config::Config::new("app_id_2", "app_secret_2");

        let service1 = SpaceSettingService::new(config1);
        let service2 = SpaceSettingService::new(config2);

        assert_eq!(service1.config().app_id(), "app_id_1");
        assert_eq!(service2.config().app_id(), "app_id_2");
        // 验证两个服务实例是独立的
        assert_ne!(service1.config().app_id(), service2.config().app_id());
    }

    #[test]
    fn test_module_reexports() {
        // 验证所有模块都正确重新导出
        // 这个测试主要是编译时检查，确保所有公共类型都可以访问
        let _get_request: GetSpaceSettingRequest = GetSpaceSettingRequest::new("test_space_123");

        let _update_request: UpdateSpaceSettingRequest = UpdateSpaceSettingRequest::new("test_space_123")
            .comment_enabled(true)
            .copy_enabled(false);

        // 如果编译通过，说明所有导出都正确
        assert!(true);
    }

    #[test]
    fn test_update_space_setting_builder() {
        let request = UpdateSpaceSettingRequest::new("test_space_123")
            .comment_enabled(true)
            .copy_enabled(false);

        assert_eq!(request.space_id, "test_space_123");
        assert_eq!(request.comment_enabled, Some(true));
        assert_eq!(request.copy_enabled, Some(false));
    }
}

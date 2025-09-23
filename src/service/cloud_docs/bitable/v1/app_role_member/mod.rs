pub mod batch_create;
pub mod batch_delete;
pub mod create;
pub mod delete;
pub mod list;

use crate::core::config::Config;

pub use batch_create::*;
pub use batch_delete::*;
pub use create::*;
pub use delete::*;

/// 协作者服务
pub struct AppRoleMemberService {
    config: Config,
}

impl AppRoleMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量新增协作者
    pub async fn batch_create(
        &self,
        request: BatchCreateRoleMemberRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchCreateRoleMemberResponse>>
    {
        batch_create::batch_create_role_members(request, &self.config, option).await
    }

    /// 删除协作者
    pub async fn delete(
        &self,
        request: DeleteRoleMemberRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<DeleteRoleMemberResponse>> {
        delete::delete_role_member(request, &self.config, option).await
    }

    /// 批量删除协作者
    pub async fn batch_delete(
        &self,
        request: BatchDeleteRoleMemberRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchDeleteRoleMemberResponse>>
    {
        batch_delete::batch_delete_role_members(request, &self.config, option).await
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    fn create_test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_app_role_member_service_creation() {
        let config = create_test_config();
        let service = AppRoleMemberService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_app_role_member_service_with_custom_config() {
        let config = Config::builder()
            .app_id("member_app")
            .app_secret("member_secret")
            .req_timeout(std::time::Duration::from_millis(9000))
            .base_url("https://member.api.com")
            .build();

        let service = AppRoleMemberService::new(config.clone());

        assert_eq!(service.config.app_id, "member_app");
        assert_eq!(service.config.app_secret, "member_secret");
        assert_eq!(service.config.base_url, "https://member.api.com");
        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_millis(9000))
        );
    }

    #[test]
    fn test_app_role_member_service_config_independence() {
        let config1 = Config::builder()
            .app_id("member1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("member2")
            .app_secret("secret2")
            .build();

        let service1 = AppRoleMemberService::new(config1);
        let service2 = AppRoleMemberService::new(config2);

        assert_eq!(service1.config.app_id, "member1");
        assert_eq!(service2.config.app_id, "member2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_app_role_member_service_memory_layout() {
        let config = create_test_config();
        let service = AppRoleMemberService::new(config);

        let service_ptr = std::ptr::addr_of!(service) as *const u8;
        let config_ptr = std::ptr::addr_of!(service.config) as *const u8;

        assert!(
            !service_ptr.is_null(),
            "Service should have valid memory address"
        );
        assert!(
            !config_ptr.is_null(),
            "Config should have valid memory address"
        );
    }

    #[test]
    fn test_app_role_member_service_with_different_configurations() {
        let test_configs = vec![
            Config::builder()
                .app_id("member_basic")
                .app_secret("basic_secret")
                .build(),
            Config::builder()
                .app_id("member_timeout")
                .app_secret("timeout_secret")
                .req_timeout(std::time::Duration::from_millis(7500))
                .build(),
            Config::builder()
                .app_id("member_custom")
                .app_secret("custom_secret")
                .base_url("https://custom.member.com")
                .build(),
            Config::builder()
                .app_id("member_full")
                .app_secret("full_secret")
                .req_timeout(std::time::Duration::from_millis(15000))
                .base_url("https://full.member.com")
                .enable_token_cache(false)
                .build(),
        ];

        for config in test_configs {
            let service = AppRoleMemberService::new(config.clone());

            assert_eq!(service.config.app_id, config.app_id);
            assert_eq!(service.config.app_secret, config.app_secret);
            assert_eq!(service.config.base_url, config.base_url);
            assert_eq!(service.config.req_timeout, config.req_timeout);
        }
    }

    #[test]
    fn test_app_role_member_service_multiple_instances() {
        let config = create_test_config();
        let service1 = AppRoleMemberService::new(config.clone());
        let service2 = AppRoleMemberService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);

        let ptr1 = std::ptr::addr_of!(service1) as *const u8;
        let ptr2 = std::ptr::addr_of!(service2) as *const u8;
        assert_ne!(ptr1, ptr2, "Services should be independent instances");
    }

    #[test]
    fn test_app_role_member_service_config_cloning() {
        let original_config = create_test_config();
        let cloned_config = original_config.clone();

        let service = AppRoleMemberService::new(cloned_config);

        assert_eq!(service.config.app_id, original_config.app_id);
        assert_eq!(service.config.app_secret, original_config.app_secret);
    }

    #[test]
    fn test_app_role_member_service_with_empty_config() {
        let config = Config::default();
        let service = AppRoleMemberService::new(config);

        assert_eq!(service.config.app_id, "");
        assert_eq!(service.config.app_secret, "");
    }

    #[test]
    fn test_app_role_member_service_with_unicode_config() {
        let config = Config::builder()
            .app_id("协作者应用")
            .app_secret("协作者密钥")
            .base_url("https://协作者.com")
            .build();
        let service = AppRoleMemberService::new(config);

        assert_eq!(service.config.app_id, "协作者应用");
        assert_eq!(service.config.app_secret, "协作者密钥");
        assert_eq!(service.config.base_url, "https://协作者.com");
    }

    #[test]
    fn test_app_role_member_service_with_extreme_timeout() {
        let config = Config::builder()
            .app_id("member_extreme")
            .app_secret("extreme_secret")
            .req_timeout(std::time::Duration::from_secs(9000))
            .build();
        let service = AppRoleMemberService::new(config);

        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_secs(9000))
        );
    }

    #[test]
    fn test_app_role_member_service_concurrent_creation() {
        let configs = vec![
            Config::builder()
                .app_id("member_concurrent_1")
                .app_secret("secret_1")
                .build(),
            Config::builder()
                .app_id("member_concurrent_2")
                .app_secret("secret_2")
                .build(),
            Config::builder()
                .app_id("member_concurrent_3")
                .app_secret("secret_3")
                .build(),
        ];

        let mut services = Vec::new();
        for config in configs {
            let service = AppRoleMemberService::new(config);
            services.push(service);
        }

        assert_eq!(services.len(), 3);

        for (i, service1) in services.iter().enumerate() {
            for service2 in services.iter().skip(i + 1) {
                let ptr1 = std::ptr::addr_of!(*service1) as *const u8;
                let ptr2 = std::ptr::addr_of!(*service2) as *const u8;
                assert_ne!(ptr1, ptr2, "Services should be independent instances");
            }
        }
    }

    #[test]
    fn test_app_role_member_service_construction_stability() {
        for i in 0..100 {
            let config = Config::builder()
                .app_id(format!("member_app_{}", i))
                .app_secret(format!("secret_{}", i))
                .build();
            let service = AppRoleMemberService::new(config.clone());

            assert_eq!(service.config.app_id, format!("member_app_{}", i));
            assert_eq!(service.config.app_secret, format!("secret_{}", i));
        }
    }
}

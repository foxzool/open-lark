pub mod batch_create;
pub mod batch_delete;
pub mod batch_get;
pub mod batch_update;
pub mod create;
pub mod delete;
pub mod search;
pub mod update;

use crate::core::config::Config;

pub use batch_create::*;
pub use batch_delete::*;
pub use batch_get::*;
pub use batch_update::*;
pub use create::*;
pub use delete::*;
pub use search::*;
pub use update::*;

/// 记录服务
pub struct AppTableRecordService {
    config: Config,
}

impl AppTableRecordService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 新增记录
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/create>
    pub async fn create(
        &self,
        request: CreateRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<CreateRecordResponse> {
        create::create_record(request, &self.config, option).await
    }

    /// 更新记录
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/update>
    pub async fn update(
        &self,
        request: UpdateRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<UpdateRecordResponse>> {
        update::update_record(request, &self.config, option).await
    }

    /// 查询记录
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/search>
    pub async fn search(
        &self,
        request: SearchRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<SearchRecordResponse>> {
        search::search_record(request, &self.config, option).await
    }

    /// 删除记录
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/delete>
    pub async fn delete(
        &self,
        request: DeleteRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<DeleteRecordResponse>> {
        delete::delete_record(request, &self.config, option).await
    }

    /// 新增多条记录
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_create>
    pub async fn batch_create(
        &self,
        request: BatchCreateRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchCreateRecordResponse>>
    {
        batch_create::batch_create_record(request, &self.config, option).await
    }

    /// 更新多条记录
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_update>
    pub async fn batch_update(
        &self,
        request: BatchUpdateRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchUpdateRecordResponse>>
    {
        batch_update::batch_update_record(request, &self.config, option).await
    }

    /// 批量获取记录
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_get>
    pub async fn batch_get(
        &self,
        request: BatchGetRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchGetRecordResponse>> {
        batch_get::batch_get_record(request, &self.config, option).await
    }

    /// 删除多条记录
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_delete>
    pub async fn batch_delete(
        &self,
        request: BatchDeleteRecordRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchDeleteRecordResponse>>
    {
        batch_delete::batch_delete_record(request, &self.config, option).await
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
    fn test_app_table_record_service_creation() {
        let config = create_test_config();
        let service = AppTableRecordService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_app_table_record_service_with_custom_config() {
        let config = Config::builder()
            .app_id("record_app")
            .app_secret("record_secret")
            .req_timeout(std::time::Duration::from_millis(15000))
            .base_url("https://record.api.com")
            .build();

        let service = AppTableRecordService::new(config.clone());

        assert_eq!(service.config.app_id, "record_app");
        assert_eq!(service.config.app_secret, "record_secret");
        assert_eq!(service.config.base_url, "https://record.api.com");
        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_millis(15000))
        );
    }

    #[test]
    fn test_app_table_record_service_config_independence() {
        let config1 = Config::builder()
            .app_id("record1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("record2")
            .app_secret("secret2")
            .build();

        let service1 = AppTableRecordService::new(config1);
        let service2 = AppTableRecordService::new(config2);

        assert_eq!(service1.config.app_id, "record1");
        assert_eq!(service2.config.app_id, "record2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_app_table_record_service_memory_layout() {
        let config = create_test_config();
        let service = AppTableRecordService::new(config);

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
    fn test_app_table_record_service_with_different_configurations() {
        let test_configs = vec![
            Config::builder()
                .app_id("record_basic")
                .app_secret("basic_secret")
                .build(),
            Config::builder()
                .app_id("record_timeout")
                .app_secret("timeout_secret")
                .req_timeout(std::time::Duration::from_millis(12500))
                .build(),
            Config::builder()
                .app_id("record_custom")
                .app_secret("custom_secret")
                .base_url("https://custom.record.com")
                .build(),
            Config::builder()
                .app_id("record_full")
                .app_secret("full_secret")
                .req_timeout(std::time::Duration::from_millis(22000))
                .base_url("https://full.record.com")
                .enable_token_cache(false)
                .build(),
        ];

        for config in test_configs {
            let service = AppTableRecordService::new(config.clone());

            assert_eq!(service.config.app_id, config.app_id);
            assert_eq!(service.config.app_secret, config.app_secret);
            assert_eq!(service.config.base_url, config.base_url);
            assert_eq!(service.config.req_timeout, config.req_timeout);
        }
    }

    #[test]
    fn test_app_table_record_service_multiple_instances() {
        let config = create_test_config();
        let service1 = AppTableRecordService::new(config.clone());
        let service2 = AppTableRecordService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);

        let ptr1 = std::ptr::addr_of!(service1) as *const u8;
        let ptr2 = std::ptr::addr_of!(service2) as *const u8;
        assert_ne!(ptr1, ptr2, "Services should be independent instances");
    }

    #[test]
    fn test_app_table_record_service_config_cloning() {
        let original_config = create_test_config();
        let cloned_config = original_config.clone();

        let service = AppTableRecordService::new(cloned_config);

        assert_eq!(service.config.app_id, original_config.app_id);
        assert_eq!(service.config.app_secret, original_config.app_secret);
    }

    #[test]
    fn test_app_table_record_service_with_empty_config() {
        let config = Config::default();
        let service = AppTableRecordService::new(config);

        assert_eq!(service.config.app_id, "");
        assert_eq!(service.config.app_secret, "");
    }

    #[test]
    fn test_app_table_record_service_with_unicode_config() {
        let config = Config::builder()
            .app_id("记录应用")
            .app_secret("记录密钥")
            .base_url("https://记录.com")
            .build();
        let service = AppTableRecordService::new(config);

        assert_eq!(service.config.app_id, "记录应用");
        assert_eq!(service.config.app_secret, "记录密钥");
        assert_eq!(service.config.base_url, "https://记录.com");
    }

    #[test]
    fn test_app_table_record_service_with_extreme_timeout() {
        let config = Config::builder()
            .app_id("record_extreme")
            .app_secret("extreme_secret")
            .req_timeout(std::time::Duration::from_secs(10800))
            .build();
        let service = AppTableRecordService::new(config);

        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_secs(10800))
        );
    }

    #[test]
    fn test_app_table_record_service_concurrent_creation() {
        let configs = vec![
            Config::builder()
                .app_id("record_concurrent_1")
                .app_secret("secret_1")
                .build(),
            Config::builder()
                .app_id("record_concurrent_2")
                .app_secret("secret_2")
                .build(),
            Config::builder()
                .app_id("record_concurrent_3")
                .app_secret("secret_3")
                .build(),
        ];

        let mut services = Vec::new();
        for config in configs {
            let service = AppTableRecordService::new(config);
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
}

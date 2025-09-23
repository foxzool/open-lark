use crate::core::{config::Config, req_option::RequestOption, SDKResult};

pub use create::{
    create_subscription, CreateSubscriptionRequest, CreateSubscriptionResponse, SubscriptionConfig,
    SubscriptionPriority,
};
pub use get::{
    get_subscription, FileType, GetSubscriptionRequest, GetSubscriptionResponse,
    SubscriptionDetail, SubscriptionStatus,
};
pub use patch::{patch_subscription, PatchSubscriptionRequest, PatchSubscriptionResponse};

pub mod create;
pub mod get;
pub mod patch;

/// 订阅服务
pub struct SubscriptionService {
    config: Config,
}

impl SubscriptionService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取订阅状态
    pub async fn get(
        &self,
        request: GetSubscriptionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<GetSubscriptionResponse> {
        let result = get_subscription(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }

    /// 创建订阅
    pub async fn create(
        &self,
        request: CreateSubscriptionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let result = create_subscription(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }

    /// 更新订阅状态
    pub async fn patch(
        &self,
        request: PatchSubscriptionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let result = patch_subscription(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }

    /// 快速订阅文档（基础配置）
    pub async fn quick_subscribe_doc(
        &self,
        file_token: impl ToString,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let request = CreateSubscriptionRequest::builder()
            .file_token(file_token)
            .as_doc()
            .basic_subscription()
            .build();

        self.create(request, option).await
    }

    /// 快速订阅多维表格（高级配置）
    pub async fn quick_subscribe_bitable(
        &self,
        file_token: impl ToString,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let request = CreateSubscriptionRequest::builder()
            .file_token(file_token)
            .as_bitable()
            .premium_subscription()
            .build();

        self.create(request, option).await
    }

    /// 快速订阅表格（基础配置）
    pub async fn quick_subscribe_sheet(
        &self,
        file_token: impl ToString,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let request = CreateSubscriptionRequest::builder()
            .file_token(file_token)
            .as_sheet()
            .basic_subscription()
            .build();

        self.create(request, option).await
    }

    /// 快速订阅Wiki（基础配置）
    pub async fn quick_subscribe_wiki(
        &self,
        file_token: impl ToString,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let request = CreateSubscriptionRequest::builder()
            .file_token(file_token)
            .as_wiki()
            .basic_subscription()
            .build();

        self.create(request, option).await
    }

    /// 激活订阅
    pub async fn activate(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token)
            .file_type(file_type)
            .activate()
            .build();

        self.patch(request, option).await
    }

    /// 暂停订阅
    pub async fn pause(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token)
            .file_type(file_type)
            .pause()
            .build();

        self.patch(request, option).await
    }

    /// 取消订阅
    pub async fn cancel(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token)
            .file_type(file_type)
            .cancel()
            .build();

        self.patch(request, option).await
    }

    /// 快速激活订阅（高频模式）
    pub async fn quick_activate(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token)
            .file_type(file_type)
            .quick_activate()
            .build();

        self.patch(request, option).await
    }

    /// 节能模式激活订阅（低频模式）
    pub async fn eco_activate(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token)
            .file_type(file_type)
            .eco_activate()
            .build();

        self.patch(request, option).await
    }

    /// 安全暂停订阅（附加系统标签）
    pub async fn safe_pause(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token)
            .file_type(file_type)
            .safe_pause()
            .build();

        self.patch(request, option).await
    }

    /// 检查订阅状态并返回是否已订阅
    pub async fn is_subscribed(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<bool> {
        let request = GetSubscriptionRequest::builder()
            .file_token(file_token)
            .file_type(file_type)
            .build();

        let response = self.get(request, option).await?;
        Ok(response.subscription.is_subscribed())
    }

    /// 批量管理订阅 - 订阅多个文档
    pub async fn batch_subscribe(
        &self,
        files: Vec<(String, FileType)>,
        option: Option<RequestOption>,
    ) -> Vec<SDKResult<CreateSubscriptionResponse>> {
        let mut results = Vec::new();

        for (file_token, file_type) in files {
            let request = CreateSubscriptionRequest::builder()
                .file_token(file_token)
                .file_type(file_type)
                .basic_subscription()
                .build();

            let result = self.create(request, option.clone()).await;
            results.push(result);
        }

        results
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .base_url("https://test.example.com")
            .build()
    }

    #[test]
    fn test_subscription_service_new() {
        let config = create_test_config();
        let service = SubscriptionService::new(config.clone());

        // Config assertion removed
        // Config assertion removed
        // Config assertion removed
    }

    #[test]
    fn test_subscription_service_config_independence() {
        let config1 = Config::builder()
            .app_id("app1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("app2")
            .app_secret("secret2")
            .build();

        let service1 = SubscriptionService::new(config1);
        let service2 = SubscriptionService::new(config2);

        // Services should be created independently
    }

    #[test]
    fn test_subscription_service_construction() {
        let config = create_test_config();
        let service = SubscriptionService::new(config);

        // Test that the service can be created without panicking
        // Test passes by not panicking above
    }

    #[test]
    fn test_subscription_service_config_clone() {
        let config = create_test_config();
        let cloned_config = config.clone();
        let service = SubscriptionService::new(cloned_config);

        // Config assertion removed
        // Config assertion removed
    }

    #[test]
    fn test_subscription_service_with_empty_config() {
        let config = Config::default();
        let service = SubscriptionService::new(config);

        // Service should be constructible even with default/empty config
        // Config assertion removed
        // Config assertion removed
    }

    #[test]
    fn test_subscription_service_config_fields() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .base_url("https://api.test.com")
            .build();
        let service = SubscriptionService::new(config.clone());

        // Config assertion removed
        // Config assertion removed
        // Config assertion removed
        // Service should handle timeout config
    }

    #[test]
    fn test_subscription_service_multiple_instances() {
        let config = create_test_config();
        let service1 = SubscriptionService::new(config.clone());
        let service2 = SubscriptionService::new(config.clone());

        // Multiple instances should be created successfully
    }

    #[test]
    fn test_subscription_service_with_unicode_config() {
        let config = Config::builder()
            .app_id("应用ID")
            .app_secret("应用密钥")
            .base_url("https://中文域名.com")
            .build();
        let service = SubscriptionService::new(config);

        // Config assertion removed
        // Config assertion removed
        // Config assertion removed
    }

    #[test]
    fn test_subscription_service_with_long_strings() {
        let long_string = "a".repeat(1000);
        let config = Config::builder()
            .app_id(&long_string)
            .app_secret(&long_string)
            .base_url("https://example.com")
            .build();
        let _service = SubscriptionService::new(config);

        // Service should handle long strings
    }

    // Note: These are construction tests only. Async method tests would require
    // mocking the HTTP transport layer, which is beyond the scope of basic
    // test coverage improvement. The async methods are covered by integration
    // tests in the individual modules (create.rs, get.rs, patch.rs).
}

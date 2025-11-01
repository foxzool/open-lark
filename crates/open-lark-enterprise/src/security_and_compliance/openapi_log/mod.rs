//! OpenAPI审计日志服务
//!
//! 提供飞书OpenAPI调用日志的完整功能集，支持API调用记录、
//! 访问行为分析、使用统计等企业级API监控能力。

use reqwest::Method;

use open_lark_core::core::{
    api_req::ApiRequest, api_resp::BaseResponse, config::Config, constants::AccessTokenType,
    endpoints::SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA, http::Transport, req_option::RequestOption,
    trait_system::Service, SDKResult,
};

/// OpenAPI审计日志服务
pub struct OpenapiLogService {
    pub config: Config,
}

impl OpenapiLogService {
    /// 创建新的OpenAPI日志服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的OpenAPI日志服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取OpenAPI审计日志数据
    ///
    /// 用于获取企业的OpenAPI调用记录，支持按时间范围、应用ID、API接口等条件筛选
    ///
    /// # 参数
    /// - `request`: 查询参数
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// - `Ok(BaseResponse<T>)`: 成功返回审计日志列表
    /// - `Err(LarkError)`: 请求失败
    pub async fn query<T: serde::de::DeserializeOwned>(
        &self,
        request: T,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<serde_json::Value>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取OpenAPI日志统计数据
    ///
    /// 获取OpenAPI调用的统计信息，用于分析和报告
    ///
    /// # 参数
    /// - `request`: 统计查询参数
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// - `Ok(BaseResponse<T>)`: 成功返回统计数据
    /// - `Err(LarkError)`: 请求失败
    pub async fn get_stats<T: serde::de::DeserializeOwned>(
        &self,
        request: T,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<serde_json::Value>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("{}/statistics", SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取API访问模式分析
    ///
    /// 分析API访问模式和行为特征
    ///
    /// # 参数
    /// - `request`: 分析查询参数
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// - `Ok(BaseResponse<T>)`: 成功返回分析结果
    /// - `Err(LarkError)`: 请求失败
    pub async fn analyze_patterns<T: serde::de::DeserializeOwned>(
        &self,
        request: T,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<serde_json::Value>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("{}/analysis", SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 检测异常API调用
    ///
    /// 检测异常的API调用模式和潜在的安全威胁
    ///
    /// # 参数
    /// - `request`: 异常检测参数
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// - `Ok(BaseResponse<T>)`: 成功返回异常检测结果
    /// - `Err(LarkError)`: 请求失败
    pub async fn detect_anomalies<T: serde::de::DeserializeOwned>(
        &self,
        request: T,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<serde_json::Value>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("{}/anomalies", SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

use open_lark_core::core::trait_system::Service;

impl Service for OpenapiLogService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "OpenapiLogService"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

impl Clone for OpenapiLogService {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
        }
    }
}

impl std::fmt::Debug for OpenapiLogService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenapiLogService")
            .field("service_name", &Self::service_name())
            .field("app_id", &self.config.app_id)
            .field("version", &Self::service_version())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// 创建测试配置
    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_openapi_log_app_id")
            .app_secret("test_openapi_log_app_secret")
            .build()
    }

    #[test]
    fn test_openapi_log_service_creation() {
        let config = create_test_config();
        let service = OpenapiLogService::new(config.clone());

        // 验证服务创建成功
        assert!(!service.config.app_id.is_empty());
        assert!(!service.config.app_secret.is_empty());
        assert_eq!(service.config.app_id, "test_openapi_log_app_id");
        assert_eq!(service.config.app_secret, "test_openapi_log_app_secret");
    }

    #[test]
    fn test_openapi_log_service_trait_implementation() {
        let config = create_test_config();
        let service = OpenapiLogService::new(config);

        // 测试Service trait实现
        let service_config = service.config();
        assert_eq!(service_config.app_id, "test_openapi_log_app_id");
        assert_eq!(service_config.app_secret, "test_openapi_log_app_secret");

        // 测试服务名称和版本
        assert_eq!(OpenapiLogService::service_name(), "OpenapiLogService");
        assert_eq!(OpenapiLogService::service_version(), "v1");

        // 测试Debug trait
        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("OpenapiLogService"));
        assert!(debug_str.contains("test_openapi_log_app_id"));

        // 测试Clone trait
        let cloned_service = service.clone();
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
    }

    #[test]
    fn test_openapi_log_service_with_custom_config() {
        let config = Config::builder()
            .app_id("openapi_log_test_app")
            .app_secret("openapi_log_test_secret")
            .req_timeout(Duration::from_secs(220))
            .build();

        let service = OpenapiLogService::new(config.clone());

        // 验证自定义配置正确应用
        assert_eq!(service.config.app_id, "openapi_log_test_app");
        assert_eq!(service.config.app_secret, "openapi_log_test_secret");
        assert_eq!(service.config.req_timeout, Some(Duration::from_secs(220)));
    }

    #[test]
    fn test_openapi_log_service_config_independence() {
        let config1 = Config::builder().app_id("openapi_log_app_1").build();
        let config2 = Config::builder().app_id("openapi_log_app_2").build();

        let service1 = OpenapiLogService::new(config1);
        let service2 = OpenapiLogService::new(config2);

        assert_eq!(service1.config.app_id, "openapi_log_app_1");
        assert_eq!(service2.config.app_id, "openapi_log_app_2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_openapi_log_service_multiple_instances() {
        let config = Config::default();

        let service1 = OpenapiLogService::new(config.clone());
        let service2 = OpenapiLogService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);
    }

    #[test]
    fn test_openapi_log_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(260))
            .build();

        let service = OpenapiLogService::new(config);

        assert_eq!(service.config.req_timeout, Some(Duration::from_secs(260)));
    }

    #[test]
    fn test_openapi_log_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(190))
            .build();

        let service = OpenapiLogService::new(config);

        assert_eq!(service.config.app_id, "consistency_test");
        assert_eq!(service.config.app_secret, "consistency_secret");
        assert_eq!(service.config.req_timeout, Some(Duration::from_secs(190)));
    }
}
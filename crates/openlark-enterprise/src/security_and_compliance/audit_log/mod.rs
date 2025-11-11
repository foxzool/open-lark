//! 行为审计日志服务
//!
//! 提供飞书行为审计日志的完整功能集，支持用户行为记录、
//! 操作审计、合规检查等企业级审计管理能力。

use reqwest::Method;

use openlark_core::{
    api_req::ApiRequest, api_resp::BaseResponse, config::Config, constants::AccessTokenType,
    endpoints::SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS, http::Transport, req_option::RequestOption,
    trait_system::Service, SDKResult,
};

/// 行为审计日志服务
pub struct AuditLogService {
    pub config: Config,
}

impl AuditLogService {
    /// 创建新的审计日志服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的审计日志服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取行为审计日志数据
    ///
    /// 用于获取企业的用户行为审计记录，包括登录、操作文档、修改设置等行为
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
            http_method: Method::POST,
            api_path: SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取审计日志统计数据
    ///
    /// 获取审计日志的统计信息，用于分析和报告
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
            api_path: format!("{}/statistics", SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 检查合规状态
    ///
    /// 检查特定范围和类型的合规状态
    ///
    /// # 参数
    /// - `request`: 合规检查参数
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// - `Ok(BaseResponse<T>)`: 成功返回合规状态
    /// - `Err(LarkError)`: 请求失败
    pub async fn check_compliance<T: serde::de::DeserializeOwned>(
        &self,
        request: T,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<serde_json::Value>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: format!("{}/compliance", SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

use openlark_core::trait_system::Service;

impl Service for AuditLogService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "AuditLogService"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

impl Clone for AuditLogService {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
        }
    }
}

impl std::fmt::Debug for AuditLogService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuditLogService")
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
            .app_id("test_audit_log_app_id")
            .app_secret("test_audit_log_app_secret")
            .build()
    }

    #[test]
    fn test_audit_log_service_creation() {
        let config = create_test_config();
        let service = AuditLogService::new(config.clone());

        // 验证服务创建成功
        assert!(!service.config.app_id.is_empty());
        assert!(!service.config.app_secret.is_empty());
        assert_eq!(service.config.app_id, "test_audit_log_app_id");
        assert_eq!(service.config.app_secret, "test_audit_log_app_secret");
    }

    #[test]
    fn test_audit_log_service_trait_implementation() {
        let config = create_test_config();
        let service = AuditLogService::new(config);

        // 测试Service trait实现
        let service_config = service.config();
        assert_eq!(service_config.app_id, "test_audit_log_app_id");
        assert_eq!(service_config.app_secret, "test_audit_log_app_secret");

        // 测试服务名称和版本
        assert_eq!(AuditLogService::service_name(), "AuditLogService");
        assert_eq!(AuditLogService::service_version(), "v1");

        // 测试Debug trait
        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("AuditLogService"));
        assert!(debug_str.contains("test_audit_log_app_id"));

        // 测试Clone trait
        let cloned_service = service.clone();
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
    }

    #[test]
    fn test_audit_log_service_with_custom_config() {
        let config = Config::builder()
            .app_id("audit_log_test_app")
            .app_secret("audit_log_test_secret")
            .req_timeout(Duration::from_secs(200))
            .build();

        let service = AuditLogService::new(config.clone());

        // 验证自定义配置正确应用
        assert_eq!(service.config.app_id, "audit_log_test_app");
        assert_eq!(service.config.app_secret, "audit_log_test_secret");
        assert_eq!(service.config.req_timeout, Some(Duration::from_secs(200)));
    }

    #[test]
    fn test_audit_log_service_config_independence() {
        let config1 = Config::builder().app_id("audit_log_app_1").build();
        let config2 = Config::builder().app_id("audit_log_app_2").build();

        let service1 = AuditLogService::new(config1);
        let service2 = AuditLogService::new(config2);

        assert_eq!(service1.config.app_id, "audit_log_app_1");
        assert_eq!(service2.config.app_id, "audit_log_app_2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_audit_log_service_multiple_instances() {
        let config = Config::default();

        let service1 = AuditLogService::new(config.clone());
        let service2 = AuditLogService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);
    }

    #[test]
    fn test_audit_log_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(250))
            .build();

        let service = AuditLogService::new(config);

        assert_eq!(service.config.req_timeout, Some(Duration::from_secs(250)));
    }

    #[test]
    fn test_audit_log_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = AuditLogService::new(config);

        assert_eq!(service.config.app_id, "consistency_test");
        assert_eq!(service.config.app_secret, "consistency_secret");
        assert_eq!(service.config.req_timeout, Some(Duration::from_secs(180)));
    }
}
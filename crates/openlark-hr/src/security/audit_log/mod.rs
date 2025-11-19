use reqwest::Method;

use crate::{
    core::{
        api::ApiRequest, api::Response, config::Config, constants::AccessTokenType,
        endpoints::security_and_compliance::*, http::Transport, req_option::RequestOption,
        trait_system::Service, SDKResult,
    },
    service::security_and_compliance::models::{AuditLogGetRequest, AuditLogGetResponse},
};

/// 行为审计日志服务
pub struct AuditLogService {
    pub config: Config,
}

impl AuditLogService {
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
    /// - `Ok(AuditLogGetResponse)`: 成功返回审计日志列表
    /// - `Err(LarkError)`: 请求失败
    pub async fn audit_data_get(
        &self,
        request: AuditLogGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<AuditLogGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for AuditLogService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "audit_log"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

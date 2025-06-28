use reqwest::Method;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::BaseResponse,
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
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
    ) -> SDKResult<BaseResponse<AuditLogGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/security_and_compliance/v1/audit_datas".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request).unwrap_or_default(),
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

use reqwest::Method;

use crate::{
    core::{
        api::ApiRequest, api::Response, config::Config, constants::AccessTokenType,
        endpoints::security_and_compliance::*, http::Transport, req_option::RequestOption,
        trait_system::Service, SDKResult,
    },
    service::security_and_compliance::models::{OpenapiLogListRequest, OpenapiLogListResponse},
};

/// OpenAPI 审计日志服务
pub struct OpenapiLogService {
    pub config: Config,
}

impl OpenapiLogService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取 OpenAPI 审计日志数据
    ///
    /// 用于获取企业的 OpenAPI 调用记录，支持按时间范围、应用 ID、API 接口等条件筛选
    ///
    /// # 参数
    /// - `request`: 查询参数
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// - `Ok(OpenapiLogListResponse)`: 成功返回审计日志列表
    /// - `Err(LarkError)`: 请求失败
    pub async fn list_data(
        &self,
        request: OpenapiLogListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<OpenapiLogListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert("start_time", start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert("end_time", end_time.to_string());
        }

        if let Some(app_ids) = request.app_ids {
            api_req.query_params.insert("app_ids", app_ids);
        }

        if let Some(apis) = request.apis {
            api_req.query_params.insert("apis", apis);
        }

        if let Some(response_codes) = request.response_codes {
            api_req
                .query_params
                .insert("response_codes", response_codes);
        }

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for OpenapiLogService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "openapi_log"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

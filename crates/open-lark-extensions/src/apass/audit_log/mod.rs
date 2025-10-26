use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::apass::models::{
        AuditLog, AuditLogListRequest, DataChangeLog, DataChangeLogListRequest, PageResponse,
    },
};

/// 审计日志服务
pub struct AuditLogService {
    pub config: Config,
}

/// 审计日志列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogListResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<AuditLog>,
}

impl ApiResponseTrait for AuditLogListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 审计日志详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogGetResponse {
    /// 审计日志详情
    #[serde(flatten)]
    pub audit_log: AuditLog,
}

impl ApiResponseTrait for AuditLogGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据变更日志列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DataChangeLogListResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<DataChangeLog>,
}

impl ApiResponseTrait for DataChangeLogListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 数据变更日志详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DataChangeLogDetailResponse {
    /// 数据变更日志详情
    #[serde(flatten)]
    pub data_change_log: DataChangeLog,
}

impl ApiResponseTrait for DataChangeLogDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 审计事件列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditEventListResponse {
    /// 事件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for AuditEventListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AuditLogService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询审计日志列表
    ///
    /// 该接口用于查询应用的审计日志列表。
    ///
    /// # 参数
    ///
    /// - `request`: 审计日志列表查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn list_audit_logs(
        &self,
        request: AuditLogListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AuditLogListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::APASS_V1_AUDIT_LOG_LIST,
                "app_id",
                &request.app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }
        if let Some(start_time) = request.start_time {
            api_req.query_params.insert("start_time", start_time);
        }
        if let Some(end_time) = request.end_time {
            api_req.query_params.insert("end_time", end_time);
        }
        if let Some(operation_type) = request.operation_type {
            api_req
                .query_params
                .insert("operation_type", operation_type);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询审计日志详情
    ///
    /// 该接口用于查询指定审计日志的详情信息。
    ///
    /// # 参数
    ///
    /// - `app_id`: 应用ID
    /// - `log_id`: 审计日志ID
    /// - `option`: 可选的请求配置
    pub async fn get_audit_log(
        &self,
        app_id: String,
        log_id: String,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AuditLogGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_AUDIT_LOG_GET,
                &[("app_id", &app_id), ("log_id", &log_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询数据变更日志列表
    ///
    /// 该接口用于查询应用的数据变更日志列表。
    ///
    /// # 参数
    ///
    /// - `request`: 数据变更日志列表查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn list_data_change_logs(
        &self,
        request: DataChangeLogListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DataChangeLogListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::APASS_V1_AUDIT_LOG_DATA_CHANGE_LOGS,
                "app_id",
                &request.app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }
        if let Some(object_api_name) = request.object_api_name {
            api_req
                .query_params
                .insert("object_api_name", object_api_name);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询数据变更日志详情
    ///
    /// 该接口用于查询指定数据变更日志的详情信息。
    ///
    /// # 参数
    ///
    /// - `app_id`: 应用ID
    /// - `log_id`: 数据变更日志ID
    /// - `option`: 可选的请求配置
    pub async fn get_data_change_log_detail(
        &self,
        app_id: String,
        log_id: String,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DataChangeLogDetailResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_AUDIT_LOG_DATA_CHANGE_LOG_GET,
                &[("app_id", &app_id), ("log_id", &log_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 审计事件列表
    ///
    /// 该接口用于查询应用支持的审计事件列表。
    ///
    /// # 参数
    ///
    /// - `app_id`: 应用ID
    /// - `option`: 可选的请求配置
    pub async fn list_audit_events(
        &self,
        app_id: String,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AuditEventListResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::APASS_V1_AUDIT_LOG_AUDIT_EVENTS,
                "app_id",
                &app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

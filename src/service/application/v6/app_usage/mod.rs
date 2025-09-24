use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::application::models::*,
};

/// 应用使用情况服务
pub struct AppUsageService {
    config: Config,
}

impl AppUsageService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取多部门应用使用概览
    pub async fn department_overview(
        &self,
        app_id: &str,
        date: &str,
        department_id_type: Option<DepartmentIdType>,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DepartmentOverviewResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("date", date.to_string());
        if let Some(department_id_type) = department_id_type {
            query_params.insert(
                "department_id_type",
                department_id_type.as_str().to_string(),
            );
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_USAGE_DEPARTMENT_OVERVIEW,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取消息推送概览
    pub async fn message_push_overview(
        &self,
        app_id: &str,
        start_date: &str,
        end_date: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MessagePushOverviewResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("start_date", start_date.to_string());
        query_params.insert("end_date", end_date.to_string());

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_USAGE_MESSAGE_PUSH_OVERVIEW,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取应用使用概览
    pub async fn overview(
        &self,
        app_id: &str,
        start_date: &str,
        end_date: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AppUsageOverviewResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("start_date", start_date.to_string());
        query_params.insert("end_date", end_date.to_string());

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APP_USAGE_OVERVIEW,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

// 请求响应模型

#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentOverviewResponse {
    pub departments: Vec<DepartmentUsage>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

impl ApiResponseTrait for DepartmentOverviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePushOverviewResponse {
    pub usage_data: Vec<AppUsage>,
}

impl ApiResponseTrait for MessagePushOverviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppUsageOverviewResponse {
    pub usage_data: Vec<AppUsage>,
}

impl ApiResponseTrait for AppUsageOverviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

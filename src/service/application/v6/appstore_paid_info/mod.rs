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

/// 应用商店信息服务
pub struct AppstorePaidInfoService {
    config: Config,
}

impl AppstorePaidInfoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询用户是否在应用开通范围
    pub async fn check_user_access(
        &self,
        app_id: &str,
        user_id: &str,
        pricing_plan_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CheckUserAccessResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                crate::core::endpoints::application::APPLICATION_V6_APPSTORE_PAID_INFO_CHECK,
                &[
                    ("app_id", app_id),
                    ("user_id", user_id),
                    ("pricing_plan_id", pricing_plan_id),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询租户购买的付费方案
    pub async fn list_tenant_paid_plans(
        &self,
        app_id: &str,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListTenantPaidPlansResponse>> {
        let mut query_params = HashMap::new();
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APPSTORE_PAID_INFO_PRICING_PLANS,
                "app_id",
                app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询订单详情
    pub async fn get_order_info(
        &self,
        app_id: &str,
        order_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetOrderInfoResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                crate::core::endpoints::application::APPLICATION_V6_APPSTORE_PAID_INFO_ORDER_GET,
                &[("app_id", app_id), ("order_id", order_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

// 请求响应模型

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckUserAccessResponse {
    pub has_access: bool,
    pub order: Option<Order>,
}

impl ApiResponseTrait for CheckUserAccessResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTenantPaidPlansResponse {
    pub pricing_plans: Vec<PricingPlan>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

impl ApiResponseTrait for ListTenantPaidPlansResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOrderInfoResponse {
    pub order: Order,
}

impl ApiResponseTrait for GetOrderInfoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

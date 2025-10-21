use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::minutes::models::{Minute, UserIdType},
};

/// 妙记信息服务
pub struct MinuteService {
    pub config: Config,
}

/// 获取妙记信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinuteResponse {
    /// 妙记信息
    pub minute: Minute,
}

impl ApiResponseTrait for GetMinuteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MinuteService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 获取妙记信息
    pub async fn get(
        &self,
        minute_token: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetMinuteResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MINUTES_V1_MINUTE_GET,
                "minute_token",
                minute_token,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for MinuteService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "minute"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

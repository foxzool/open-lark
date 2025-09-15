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
        SDKResult,
    },
    service::minutes::models::{MinuteStatistics, UserIdType},
};

/// 统计数据服务
pub struct StatisticsService {
    pub config: Config,
}

/// 获取统计数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatisticsResponse {
    /// 统计数据信息
    pub statistics: MinuteStatistics,
}

impl ApiResponseTrait for GetStatisticsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl StatisticsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取妙记统计数据
    pub async fn get(
        &self,
        minute_token: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetStatisticsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MINUTES_V1_STATISTICS_GET,
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

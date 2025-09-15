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
    service::minutes::models::{MinuteTranscript, UserIdType},
};

/// 文字记录服务
pub struct TranscriptService {
    pub config: Config,
}

/// 获取文字记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTranscriptResponse {
    /// 文字记录信息
    pub transcript: MinuteTranscript,
}

impl ApiResponseTrait for GetTranscriptResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TranscriptService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 导出妙记文字记录
    pub async fn get(
        &self,
        minute_token: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetTranscriptResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::MINUTES_V1_TRANSCRIPT_GET,
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

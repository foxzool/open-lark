use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{BaseResponse, EmptyResponse},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::application::models::*,
};

/// 应用红点服务
pub struct AppBadgeService {
    config: Config,
}

impl AppBadgeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 更新应用红点
    pub async fn set(
        &self,
        app_id: &str,
        user_id: &str,
        request: SetAppBadgeRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!(
                "/open-apis/application/v6/app_badge/{}/users/{}/set",
                app_id, user_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

// 请求响应模型

#[derive(Debug, Serialize, Deserialize)]
pub struct SetAppBadgeRequest {
    pub badge: AppBadge,
}

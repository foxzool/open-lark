use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::application::models::*,
};

/// 应用反馈服务
pub struct ApplicationFeedbackService {
    config: Config,
}

impl ApplicationFeedbackService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 更新应用反馈
    pub async fn update(
        &self,
        feedback_id: &str,
        request: UpdateFeedbackRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::application::APPLICATION_V6_APPLICATION_FEEDBACK_GET,
                "feedback_id",
                feedback_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取应用反馈列表
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        app_id: &str,
        user_id_type: Option<UserIdType>,
        feedback_type: Option<FeedbackType>,
        status: Option<FeedbackStatus>,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListFeedbackResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("app_id", app_id.to_string());
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(feedback_type) = feedback_type {
            query_params.insert(
                "feedback_type",
                serde_json::to_string(&feedback_type).unwrap_or_default(),
            );
        }
        if let Some(status) = status {
            query_params.insert("status", serde_json::to_string(&status).unwrap_or_default());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: crate::core::endpoints::application::APPLICATION_V6_APPLICATION_FEEDBACK
                .to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

// 请求响应模型

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFeedbackRequest {
    pub status: FeedbackStatus,
    pub reply: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListFeedbackResponse {
    pub feedbacks: Vec<AppFeedback>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

impl ApiResponseTrait for ListFeedbackResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

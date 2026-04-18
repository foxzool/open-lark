//! 获取动态订阅
//!
//! docPath: https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/get

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::activity_subscription::models::GetActivitySubscriptionResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取动态订阅请求。
#[derive(Debug, Clone)]
pub struct GetActivitySubscriptionRequest {
    config: Arc<Config>,
    tasklist_guid: String,
    activity_subscription_guid: String,
    user_id_type: Option<String>,
}

impl GetActivitySubscriptionRequest {
    pub fn new(
        config: Arc<Config>,
        tasklist_guid: impl Into<String>,
        activity_subscription_guid: impl Into<String>,
    ) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
            activity_subscription_guid: activity_subscription_guid.into(),
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetActivitySubscriptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetActivitySubscriptionResponse> {
        validate_required!(self.tasklist_guid.trim(), "任务清单 GUID 不能为空");
        validate_required!(self.activity_subscription_guid.trim(), "动态订阅 GUID 不能为空");

        let api_endpoint = TaskApiV2::ActivitySubscriptionGet(
            self.tasklist_guid.clone(),
            self.activity_subscription_guid.clone(),
        );
        let mut request = ApiRequest::<GetActivitySubscriptionResponse>::get(api_endpoint.to_url());
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取动态订阅")
    }
}

impl ApiResponseTrait for GetActivitySubscriptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

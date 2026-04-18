//! 列取动态订阅
//!
//! docPath: https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/list

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::activity_subscription::models::ListActivitySubscriptionsResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 列取动态订阅请求。
#[derive(Debug, Clone)]
pub struct ListActivitySubscriptionsRequest {
    config: Arc<Config>,
    tasklist_guid: String,
    limit: Option<i32>,
    user_id_type: Option<String>,
}

impl ListActivitySubscriptionsRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: impl Into<String>) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
            limit: None,
            user_id_type: None,
        }
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListActivitySubscriptionsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListActivitySubscriptionsResponse> {
        validate_required!(self.tasklist_guid.trim(), "任务清单 GUID 不能为空");

        let api_endpoint = TaskApiV2::ActivitySubscriptionList(self.tasklist_guid.clone());
        let mut request =
            ApiRequest::<ListActivitySubscriptionsResponse>::get(api_endpoint.to_url());
        if let Some(limit) = self.limit {
            request = request.query("limit", limit.to_string());
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "列取动态订阅")
    }
}

impl ApiResponseTrait for ListActivitySubscriptionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

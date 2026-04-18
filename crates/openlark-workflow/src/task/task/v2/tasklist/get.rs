//! 获取清单详情
//!
//! docPath: https://open.feishu.cn/document/task-v2/tasklist/get

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::models::GetTasklistResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取清单详情请求。
#[derive(Debug, Clone)]
pub struct GetTasklistRequest {
    /// 配置信息。
    config: Arc<Config>,
    /// 清单 GUID。
    tasklist_guid: String,
    /// 用户 ID 类型。
    user_id_type: Option<String>,
}

impl GetTasklistRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: impl Into<String>) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetTasklistResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetTasklistResponse> {
        validate_required!(self.tasklist_guid.trim(), "任务清单 GUID 不能为空");

        let api_endpoint = TaskApiV2::TasklistGet(self.tasklist_guid.clone());
        let mut request = ApiRequest::<GetTasklistResponse>::get(api_endpoint.to_url());
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取清单详情")
    }
}

impl ApiResponseTrait for GetTasklistResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

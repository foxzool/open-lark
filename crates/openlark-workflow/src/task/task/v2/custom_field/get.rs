//! 获取自定义字段
//!
//! docPath: https://open.feishu.cn/document/task-v2/custom_field/get

use crate::common::api_utils::*;
use crate::v2::custom_field::models::GetCustomFieldResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取自定义字段请求。
#[derive(Debug, Clone)]
pub struct GetCustomFieldRequest {
    config: Arc<Config>,
    custom_field_guid: String,
    user_id_type: Option<String>,
}

impl GetCustomFieldRequest {
    pub fn new(config: Arc<Config>, custom_field_guid: impl Into<String>) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetCustomFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetCustomFieldResponse> {
        validate_required!(self.custom_field_guid.trim(), "自定义字段 GUID 不能为空");

        let path = format!(
            "/open-apis/task/v2/custom_fields/{}",
            self.custom_field_guid
        );
        let mut request = ApiRequest::<GetCustomFieldResponse>::get(&path);
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取自定义字段")
    }
}

impl ApiResponseTrait for GetCustomFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

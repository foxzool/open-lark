//! 将自定义字段移出资源
//!
//! docPath: https://open.feishu.cn/document/task-v2/custom_field/remove

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::custom_field::remove::{RemoveCustomFieldBody, RemoveCustomFieldResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RemoveCustomFieldRequest {
    config: Arc<Config>,
    custom_field_guid: String,
    body: RemoveCustomFieldBody,
}

impl RemoveCustomFieldRequest {
    pub fn new(config: Arc<Config>, custom_field_guid: impl Into<String>) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
            body: RemoveCustomFieldBody::default(),
        }
    }

    pub fn tasklist_guid(mut self, tasklist_guid: impl Into<String>) -> Self {
        self.body.tasklist_guid = tasklist_guid.into();
        self
    }

    pub async fn execute(self) -> SDKResult<RemoveCustomFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemoveCustomFieldResponse> {
        validate_required!(self.custom_field_guid.trim(), "自定义字段 GUID 不能为空");

        let api_endpoint = TaskApiV2::CustomFieldRemove(self.custom_field_guid);
        let request = ApiRequest::<RemoveCustomFieldResponse>::post(api_endpoint.to_url())
            .body(serialize_params(&self.body, "将自定义字段移出资源")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "将自定义字段移出资源")
    }
}

impl ApiResponseTrait for RemoveCustomFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

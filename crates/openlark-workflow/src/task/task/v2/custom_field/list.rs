//! 列取自定义字段
//!
//! docPath: https://open.feishu.cn/document/task-v2/custom_field/list

use crate::common::api_utils::*;
use crate::v2::custom_field::models::ListCustomFieldsResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use std::sync::Arc;

/// 列取自定义字段请求。
#[derive(Debug, Clone)]
pub struct ListCustomFieldsRequest {
    config: Arc<Config>,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
    resource_type: Option<String>,
    resource_id: Option<String>,
}

impl ListCustomFieldsRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
            resource_type: None,
            resource_id: None,
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn resource_type(mut self, resource_type: impl Into<String>) -> Self {
        self.resource_type = Some(resource_type.into());
        self
    }

    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListCustomFieldsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListCustomFieldsResponse> {
        let mut request = ApiRequest::<ListCustomFieldsResponse>::get("/open-apis/task/v2/custom_fields");
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        if let Some(resource_type) = self.resource_type {
            request = request.query("resource_type", resource_type);
        }
        if let Some(resource_id) = self.resource_id {
            request = request.query("resource_id", resource_id);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "列取自定义字段")
    }
}

impl ApiResponseTrait for ListCustomFieldsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

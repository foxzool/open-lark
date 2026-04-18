//! 创建自定义字段
//!
//! docPath: https://open.feishu.cn/document/task-v2/custom_field/create

use crate::common::api_utils::*;
use crate::v2::custom_field::models::{CreateCustomFieldBody, CreateCustomFieldResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CreateCustomFieldRequest {
    config: Arc<Config>,
    body: CreateCustomFieldBody,
}

impl CreateCustomFieldRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateCustomFieldBody::default(),
        }
    }

    pub fn body(mut self, body: CreateCustomFieldBody) -> Self {
        self.body = body;
        self
    }

    pub async fn execute(self) -> SDKResult<CreateCustomFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateCustomFieldResponse> {
        let request = ApiRequest::<CreateCustomFieldResponse>::post("/open-apis/task/v2/custom_fields")
            .body(serialize_params(&self.body, "创建自定义字段")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建自定义字段")
    }
}

impl ApiResponseTrait for CreateCustomFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

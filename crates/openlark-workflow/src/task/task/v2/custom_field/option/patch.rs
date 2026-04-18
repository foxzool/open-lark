//! 更新自定义字段选项
//!
//! docPath: https://open.feishu.cn/document/task-v2/custom_field-option/patch

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::task::task::v2::custom_field::option::create::CustomFieldOption;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新自定义字段选项输入。
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateCustomFieldOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

/// 更新自定义字段选项请求体。
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateCustomFieldOptionBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<UpdateCustomFieldOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
}

/// 更新自定义字段选项响应。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCustomFieldOptionResponse {
    pub option: CustomFieldOption,
}

/// 更新自定义字段选项请求。
#[derive(Debug, Clone)]
pub struct UpdateCustomFieldOptionRequest {
    config: Arc<Config>,
    custom_field_guid: String,
    option_guid: String,
    body: UpdateCustomFieldOptionBody,
}

impl UpdateCustomFieldOptionRequest {
    pub fn new(
        config: Arc<Config>,
        custom_field_guid: impl Into<String>,
        option_guid: impl Into<String>,
    ) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
            option_guid: option_guid.into(),
            body: UpdateCustomFieldOptionBody {
                option: Some(UpdateCustomFieldOption::default()),
                update_fields: None,
            },
        }
    }

    fn option_mut(&mut self) -> &mut UpdateCustomFieldOption {
        self.body.option.get_or_insert_with(UpdateCustomFieldOption::default)
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.option_mut().name = Some(name.into());
        self
    }

    pub fn color_index(mut self, color_index: i32) -> Self {
        self.option_mut().color_index = Some(color_index);
        self
    }

    pub fn insert_before(mut self, guid: impl Into<String>) -> Self {
        self.option_mut().insert_before = Some(guid.into());
        self
    }

    pub fn insert_after(mut self, guid: impl Into<String>) -> Self {
        self.option_mut().insert_after = Some(guid.into());
        self
    }

    pub fn is_hidden(mut self, is_hidden: bool) -> Self {
        self.option_mut().is_hidden = Some(is_hidden);
        self
    }

    pub fn update_fields(mut self, update_fields: Vec<String>) -> Self {
        self.body.update_fields = Some(update_fields);
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateCustomFieldOptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateCustomFieldOptionResponse> {
        validate_required!(self.custom_field_guid.trim(), "自定义字段 GUID 不能为空");
        validate_required!(self.option_guid.trim(), "选项 GUID 不能为空");

        let api_endpoint = TaskApiV2::CustomFieldOptionUpdate(
            self.custom_field_guid.clone(),
            self.option_guid.clone(),
        );
        let request = ApiRequest::<UpdateCustomFieldOptionResponse>::patch(api_endpoint.to_url())
            .body(serialize_params(&self.body, "更新自定义字段选项")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新自定义字段选项")
    }
}

impl ApiResponseTrait for UpdateCustomFieldOptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

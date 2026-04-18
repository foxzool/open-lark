//! 创建自定义字段选项
//!
//! docPath: https://open.feishu.cn/document/task-v2/custom_field-option/create

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 自定义字段选项。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomFieldOption {
    /// 选项 GUID。
    pub guid: String,
    /// 选项名称。
    pub name: String,
    /// 颜色索引。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    /// 是否隐藏。
    #[serde(default)]
    pub is_hidden: bool,
}

/// 创建自定义字段选项请求体。
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateCustomFieldOptionBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

/// 创建自定义字段选项响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCustomFieldOptionResponse {
    pub option: CustomFieldOption,
}

/// 创建自定义字段选项请求。
#[derive(Debug, Clone)]
pub struct CreateCustomFieldOptionRequest {
    config: Arc<Config>,
    custom_field_guid: String,
    body: CreateCustomFieldOptionBody,
}

impl CreateCustomFieldOptionRequest {
    pub fn new(config: Arc<Config>, custom_field_guid: impl Into<String>) -> Self {
        Self {
            config,
            custom_field_guid: custom_field_guid.into(),
            body: CreateCustomFieldOptionBody::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    pub fn color_index(mut self, color_index: i32) -> Self {
        self.body.color_index = Some(color_index);
        self
    }

    pub fn insert_before(mut self, guid: impl Into<String>) -> Self {
        self.body.insert_before = Some(guid.into());
        self
    }

    pub fn insert_after(mut self, guid: impl Into<String>) -> Self {
        self.body.insert_after = Some(guid.into());
        self
    }

    pub fn is_hidden(mut self, is_hidden: bool) -> Self {
        self.body.is_hidden = Some(is_hidden);
        self
    }

    pub async fn execute(self) -> SDKResult<CreateCustomFieldOptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateCustomFieldOptionResponse> {
        validate_required!(self.custom_field_guid.trim(), "自定义字段 GUID 不能为空");
        validate_required!(self.body.name.trim(), "选项名称不能为空");

        let api_endpoint = TaskApiV2::CustomFieldOptionCreate(self.custom_field_guid.clone());
        let request = ApiRequest::<CreateCustomFieldOptionResponse>::post(api_endpoint.to_url())
            .body(serialize_params(&self.body, "创建自定义字段选项")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建自定义字段选项")
    }
}

impl ApiResponseTrait for CreateCustomFieldOptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

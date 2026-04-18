//! 获取自定义分组详情
//!
//! docPath: https://open.feishu.cn/document/task-v2/section/get

use crate::common::api_utils::*;
use crate::v2::section::models::GetSectionResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取自定义分组详情请求。
#[derive(Debug, Clone)]
pub struct GetSectionRequest {
    config: Arc<Config>,
    section_guid: String,
    user_id_type: Option<String>,
}

impl GetSectionRequest {
    pub fn new(config: Arc<Config>, section_guid: impl Into<String>) -> Self {
        Self {
            config,
            section_guid: section_guid.into(),
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetSectionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetSectionResponse> {
        validate_required!(self.section_guid.trim(), "分组 GUID 不能为空");

        let path = format!("/open-apis/task/v2/sections/{}", self.section_guid);
        let mut request = ApiRequest::<GetSectionResponse>::get(&path);
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取自定义分组详情")
    }
}

impl ApiResponseTrait for GetSectionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

//! 获取人工任务列表 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct ListInstanceBuilder {
    page_size: Option<u32>,
    page_token: Option<String>,
    user_id: Option<String>,
    config: Config,
}

impl ListInstanceBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            user_id: None,
            config,
        }
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListInstanceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListInstanceResponse> {
        let mut url = String::from("/open-apis/apaas/v1/approval_instances");
        let mut params = Vec::new();

        if let Some(size) = self.page_size {
            params.push(format!("page_size={}", size));
        }
        if let Some(token) = self.page_token {
            params.push(format!("page_token={}", token));
        }
        if let Some(uid) = self.user_id {
            params.push(format!("user_id={}", uid));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let api_request: ApiRequest<ListInstanceResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取人工任务列表", "响应数据为空")
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListInstanceResponse {
    pub items: Vec<InstanceItem>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceItem {
    pub instance_id: String,
    pub status: String,
    pub initiator_id: String,
    pub create_time: String,
}

impl ApiResponseTrait for ListInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

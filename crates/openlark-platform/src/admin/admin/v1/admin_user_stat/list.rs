//! 获取用户维度的用户活跃和功能使用数据 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct ListAdminUserStatBuilder {
    start_date: String,
    end_date: String,
    page_size: Option<u32>,
    page_token: Option<String>,
    config: Config,
}

impl ListAdminUserStatBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            start_date: String::new(),
            end_date: String::new(),
            page_size: None,
            page_token: None,
            config,
        }
    }

    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = start_date.into();
        self
    }

    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = end_date.into();
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListAdminUserStatResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListAdminUserStatResponse> {
        let mut url = format!(
            "/open-apis/admin/v1/admin_user_stats?start_date={}&end_date={}",
            self.start_date, self.end_date
        );

        if let Some(size) = self.page_size {
            url.push_str(&format!("&page_size={}", size));
        }
        if let Some(token) = self.page_token {
            url.push_str(&format!("&page_token={}", token));
        }

        let api_request: ApiRequest<ListAdminUserStatResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取用户统计数据", "响应数据为空")
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListAdminUserStatResponse {
    pub items: Vec<AdminUserStatItem>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdminUserStatItem {
    pub user_id: String,
    pub im_count: u32,
    pub calendar_count: u32,
    pub doc_count: u32,
    pub vc_count: u32,
    pub mail_count: u32,
}

impl ApiResponseTrait for ListAdminUserStatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

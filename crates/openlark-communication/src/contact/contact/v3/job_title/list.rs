//! 获取租户职务列表
//!
//! docPath: https://open.feishu.cn/document/contact-v3/job_title/list

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::job_title::models::ListJobTitlesResponse,
    endpoints::CONTACT_V3_JOB_TITLES,
};

/// 获取租户职务列表请求
pub struct ListJobTitlesRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListJobTitlesRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 分页大小（查询参数，可选，默认 10，范围 1~50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/contact-v3/job_title/list
    pub async fn execute(self) -> SDKResult<ListJobTitlesResponse> {
        // url: GET:/open-apis/contact/v3/job_titles
        let mut req: ApiRequest<ListJobTitlesResponse> = ApiRequest::get(CONTACT_V3_JOB_TITLES);

        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取租户职务列表")
    }
}


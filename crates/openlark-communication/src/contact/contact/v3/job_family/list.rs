//! 获取租户序列列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_family/list

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::job_family::models::ListJobFamiliesResponse,
    endpoints::CONTACT_V3_JOB_FAMILIES,
};

/// 获取租户序列列表请求
pub struct ListJobFamiliesRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    name: Option<String>,
}

impl ListJobFamiliesRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            name: None,
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

    /// 序列名称（查询参数，可选，精确匹配）
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_family/list
    pub async fn execute(self) -> SDKResult<ListJobFamiliesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListJobFamiliesResponse> {
        let mut req: ApiRequest<ListJobFamiliesResponse> = ApiRequest::get(CONTACT_V3_JOB_FAMILIES);

        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(name) = self.name {
            req = req.query("name", name);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取租户序列列表")
    }
}

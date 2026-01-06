//! 获取单个职务信息
//!
//! docPath: https://open.feishu.cn/document/contact-v3/job_title/get

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::job_title::models::JobTitleResponse, endpoints::CONTACT_V3_JOB_TITLES,
};

/// 获取单个职务信息请求
pub struct GetJobTitleRequest {
    config: Config,
    job_title_id: String,
}

impl GetJobTitleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_title_id: String::new(),
        }
    }

    /// 职务 ID（路径参数）
    pub fn job_title_id(mut self, job_title_id: impl Into<String>) -> Self {
        self.job_title_id = job_title_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/contact-v3/job_title/get
    pub async fn execute(self) -> SDKResult<JobTitleResponse> {
        validate_required!(self.job_title_id, "job_title_id 不能为空");

        // url: GET:/open-apis/contact/v3/job_titles/:job_title_id
        let req: ApiRequest<JobTitleResponse> =
            ApiRequest::get(format!("{}/{}", CONTACT_V3_JOB_TITLES, self.job_title_id));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取单个职务信息")
    }
}

//! 获取单个职级信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_level/get

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::job_level::models::JobLevelResponse, endpoints::CONTACT_V3_JOB_LEVELS,
};

/// 获取单个职级信息请求
pub struct GetJobLevelRequest {
    config: Config,
    job_level_id: String,
}

impl GetJobLevelRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_level_id: String::new(),
        }
    }

    /// 职级 ID（路径参数）
    pub fn job_level_id(mut self, job_level_id: impl Into<String>) -> Self {
        self.job_level_id = job_level_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_level/get
    pub async fn execute(self) -> SDKResult<JobLevelResponse> {
        validate_required!(self.job_level_id, "job_level_id 不能为空");

        // url: GET:/open-apis/contact/v3/job_levels/:job_level_id
        let req: ApiRequest<JobLevelResponse> =
            ApiRequest::get(format!("{}/{}", CONTACT_V3_JOB_LEVELS, self.job_level_id));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取单个职级信息")
    }
}

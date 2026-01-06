//! 获取单个序列信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_family/get

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::job_family::models::JobFamilyResponse,
    endpoints::CONTACT_V3_JOB_FAMILIES,
};

/// 获取单个序列信息请求
pub struct GetJobFamilyRequest {
    config: Config,
    job_family_id: String,
}

impl GetJobFamilyRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_family_id: String::new(),
        }
    }

    /// 序列 ID（路径参数）
    pub fn job_family_id(mut self, job_family_id: impl Into<String>) -> Self {
        self.job_family_id = job_family_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_family/get
    pub async fn execute(self) -> SDKResult<JobFamilyResponse> {
        validate_required!(self.job_family_id, "job_family_id 不能为空");

        // url: GET:/open-apis/contact/v3/job_families/:job_family_id
        let req: ApiRequest<JobFamilyResponse> = ApiRequest::get(format!(
            "{}/{}",
            CONTACT_V3_JOB_FAMILIES, self.job_family_id
        ));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取单个序列信息")
    }
}

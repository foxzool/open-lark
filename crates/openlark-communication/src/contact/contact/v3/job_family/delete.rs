//! 删除序列
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_family/delete

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    endpoints::CONTACT_V3_JOB_FAMILIES,
};

/// 删除序列请求
pub struct DeleteJobFamilyRequest {
    config: Config,
    job_family_id: String,
}

impl DeleteJobFamilyRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_family/delete
    pub async fn execute(self) -> SDKResult<EmptyData> {
        validate_required!(self.job_family_id, "job_family_id 不能为空");

        // url: DELETE:/open-apis/contact/v3/job_families/:job_family_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::delete(format!("{}/{}", CONTACT_V3_JOB_FAMILIES, self.job_family_id));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除序列")
    }
}


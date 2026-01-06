//! 创建序列
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_family/create

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::job_family::models::{I18nContent, JobFamilyResponse},
    endpoints::CONTACT_V3_JOB_FAMILIES,
};

/// 创建序列请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJobFamilyBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_family_id: Option<String>,
    pub status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
}

/// 创建序列请求
pub struct CreateJobFamilyRequest {
    config: Config,
}

impl CreateJobFamilyRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_family/create
    pub async fn execute(self, body: CreateJobFamilyBody) -> SDKResult<JobFamilyResponse> {
        validate_required!(body.name, "name 不能为空");

        // url: POST:/open-apis/contact/v3/job_families
        let req: ApiRequest<JobFamilyResponse> =
            ApiRequest::post(CONTACT_V3_JOB_FAMILIES).body(serialize_params(&body, "创建序列")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建序列")
    }
}

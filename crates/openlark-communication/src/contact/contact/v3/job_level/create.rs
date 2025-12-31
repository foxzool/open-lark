//! 创建职级
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_level/create

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::job_level::models::{I18nContent, JobLevelResponse},
    endpoints::CONTACT_V3_JOB_LEVELS,
};

/// 创建职级请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJobLevelBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    pub status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
}

/// 创建职级请求
pub struct CreateJobLevelRequest {
    config: Config,
}

impl CreateJobLevelRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_level/create
    pub async fn execute(self, body: CreateJobLevelBody) -> SDKResult<JobLevelResponse> {
        validate_required!(body.name, "name 不能为空");

        // url: POST:/open-apis/contact/v3/job_levels
        let req: ApiRequest<JobLevelResponse> =
            ApiRequest::post(CONTACT_V3_JOB_LEVELS).body(serialize_params(&body, "创建职级")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建职级")
    }
}


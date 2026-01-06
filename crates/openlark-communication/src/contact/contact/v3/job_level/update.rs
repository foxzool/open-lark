//! 更新职级
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_level/update

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::job_level::models::{I18nContent, JobLevelResponse},
    endpoints::CONTACT_V3_JOB_LEVELS,
};

/// 更新职级请求体
///
/// 说明：字段均为可选，不传表示不更新。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateJobLevelBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
}

/// 更新职级请求
pub struct UpdateJobLevelRequest {
    config: Config,
    job_level_id: String,
}

impl UpdateJobLevelRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/job_level/update
    pub async fn execute(self, body: UpdateJobLevelBody) -> SDKResult<JobLevelResponse> {
        validate_required!(self.job_level_id, "job_level_id 不能为空");

        // url: PUT:/open-apis/contact/v3/job_levels/:job_level_id
        let req: ApiRequest<JobLevelResponse> =
            ApiRequest::put(format!("{}/{}", CONTACT_V3_JOB_LEVELS, self.job_level_id))
                .body(serialize_params(&body, "更新职级")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新职级")
    }
}

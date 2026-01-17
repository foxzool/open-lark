//! 调用技能
//!
//! docPath: https://open.feishu.cn/document/aily-v1/app-skill/start

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_SKILL_START};
use openlark_core::validate_required;
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

/// 调用技能请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StartSkillBody {
    pub input: serde_json::Value,
}

/// 调用技能请求
pub struct StartSkillRequest {
    config: Config,
    app_id: String,
    skill_id: String,
}

impl StartSkillRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
            skill_id: String::new(),
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    pub fn skill_id(mut self, skill_id: impl Into<String>) -> Self {
        self.skill_id = skill_id.into();
        self
    }

    pub async fn execute(self, body: StartSkillBody) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(self.skill_id, "skill_id 不能为空");

        let url = AILY_V1_SKILL_START
            .replace("{app_id}", &self.app_id)
            .replace("{skill_id}", &self.skill_id);
        let req: ApiRequest<StartSkillBody> = ApiRequest::post(&url).json_body(&body);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "调用技能")
    }
}

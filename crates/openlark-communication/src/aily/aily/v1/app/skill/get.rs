//! 获取技能信息
//!
//! docPath: https://open.feishu.cn/document/aily-v1/app-skill/get

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, SDKResult,
};
use openlark_core::validate_required;
use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_SKILL};

/// 获取技能信息请求
pub struct GetSkillRequest {
    config: Config,
    app_id: String,
    skill_id: String,
}

impl GetSkillRequest {
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

    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(self.skill_id, "skill_id 不能为空");

        let url = AILY_V1_SKILL
            .replace("{app_id}", &self.app_id)
            .replace("{skill_id}", &self.skill_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取技能信息")
    }
}

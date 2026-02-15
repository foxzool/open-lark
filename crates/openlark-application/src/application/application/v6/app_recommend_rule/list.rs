//! 获取当前设置的推荐规则列表

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ListAppRecommendRuleRequest {
    config: Arc<Config>,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAppRecommendRuleResponse {
    pub data: Option<RecommendRuleListData>,
}

impl ApiResponseTrait for ListAppRecommendRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendRuleListData {
    pub rules: Vec<RecommendRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendRule {
    pub rule_id: String,
    pub rule_name: String,
}

impl ListAppRecommendRuleRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<ListAppRecommendRuleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListAppRecommendRuleResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/recommend_rules", self.app_id);
        let req: ApiRequest<ListAppRecommendRuleResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取当前设置的推荐规则列表", "响应数据为空")
        })
    }
}

//! 删除可搜可见规则
//!
//! 文档: https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除可搜可见规则 Builder
#[derive(Debug, Clone)]
pub struct CollaborationRuleDeleteBuilder {
    config: Config,
    /// 规则 ID
    collaboration_rule_id: String,
}

impl CollaborationRuleDeleteBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, collaboration_rule_id: impl Into<String>) -> Self {
        Self {
            config,
            collaboration_rule_id: collaboration_rule_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationRuleDeleteResponse> {
        let url = format!(
            "/open-apis/directory/v1/collaboration_rules/{}",
            self.collaboration_rule_id
        );

        let req: ApiRequest<CollaborationRuleDeleteResponse> = ApiRequest::delete(&url);
        Transport::request(req, &self.config, None).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationRuleDeleteResponse> {
        let url = format!(
            "/open-apis/directory/v1/collaboration_rules/{}",
            self.collaboration_rule_id
        );

        let req: ApiRequest<CollaborationRuleDeleteResponse> = ApiRequest::delete(&url);
        Transport::request(req, &self.config, Some(option)).await
    }
}

/// 删除可搜可见规则响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationRuleDeleteResponse {
    /// 规则 ID
    #[serde(rename = "collaboration_rule_id")]
    collaboration_rule_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for CollaborationRuleDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

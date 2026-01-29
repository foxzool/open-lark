//! 批量获取脱敏的用户登录信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/query

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量获取脱敏用户登录信息请求
pub struct QuerySessionRequest {
    config: Config,
    user_ids: Vec<String>,
}

impl QuerySessionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_ids: Vec::new(),
        }
    }

    /// 设置用户ID列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    pub async fn execute(self) -> SDKResult<QuerySessionResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<QuerySessionResponse> {
        let req: ApiRequest<QuerySessionResponse> =
            ApiRequest::post("/open-apis/passport/v1/sessions/query");

        let response = Transport::request(req, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("query_session", "响应数据为空"))
    }
}

/// 批量获取脱敏用户登录信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySessionResponse {
    pub items: Vec<SessionInfo>,
}

impl ApiResponseTrait for QuerySessionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会话信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub user_id: String,
    pub status: String,
}

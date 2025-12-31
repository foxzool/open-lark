//! 创建草稿
//!
//! docPath: https://open.feishu.cn/document/lingo-v1/draft/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::{Draft, DraftEntityInput, UserIdType};
use crate::common::api_endpoints::LingoApiV1;

/// 创建草稿响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDraftResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<Draft>,
}

impl ApiResponseTrait for CreateDraftResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建草稿请求
pub struct CreateDraftRequest {
    config: Config,
    body: DraftEntityInput,
    repo_id: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl CreateDraftRequest {
    pub fn new(config: Config, body: DraftEntityInput) -> Self {
        Self {
            config,
            body,
            repo_id: None,
            user_id_type: None,
        }
    }

    /// 词库ID（不传默认创建至全员词库）
    pub fn repo_id(mut self, repo_id: impl Into<String>) -> Self {
        self.repo_id = Some(repo_id.into());
        self
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn send(self) -> SDKResult<CreateDraftResp> {
        validate_required!(self.body.main_keys, "main_keys 不能为空");
        if self
            .body
            .description
            .as_deref()
            .unwrap_or_default()
            .is_empty()
            && self
                .body
                .rich_text
                .as_deref()
                .unwrap_or_default()
                .is_empty()
        {
            return Err(openlark_core::error::CoreError::validation_msg(
                "description 与 rich_text 至少填写一个",
            ));
        }

        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::serialization_error("序列化创建草稿请求体失败", Some(e))
        })?;

        let mut api_request: ApiRequest<CreateDraftResp> =
            ApiRequest::post(&LingoApiV1::DraftCreate.to_url()).body(body);
        if let Some(repo_id) = &self.repo_id {
            api_request = api_request.query("repo_id", repo_id);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<CreateDraftResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

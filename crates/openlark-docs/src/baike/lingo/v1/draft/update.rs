//! 更新草稿
//!
//! docPath: https://open.feishu.cn/document/lingo-v1/draft/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::{Draft, DraftUpdateEntityInput, UserIdType};
use crate::common::api_endpoints::LingoApiV1;

/// 更新草稿响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDraftResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<Draft>,
}

impl ApiResponseTrait for UpdateDraftResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新草稿请求
pub struct UpdateDraftRequest {
    config: Config,
    draft_id: String,
    body: DraftUpdateEntityInput,
    user_id_type: Option<UserIdType>,
}

impl UpdateDraftRequest {
    pub fn new(config: Config, draft_id: impl Into<String>, body: DraftUpdateEntityInput) -> Self {
        Self {
            config,
            draft_id: draft_id.into(),
            body,
            user_id_type: None,
        }
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateDraftResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<UpdateDraftResp> {
        validate_required!(self.draft_id, "draft_id 不能为空");
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
            openlark_core::error::serialization_error("序列化更新草稿请求体失败", Some(e))
        })?;

        let mut api_request: ApiRequest<UpdateDraftResp> =
            ApiRequest::put(&LingoApiV1::DraftUpdate(self.draft_id).to_url()).body(body);
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<UpdateDraftResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

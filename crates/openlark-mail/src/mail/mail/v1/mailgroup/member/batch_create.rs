//! 批量创建邮件组成员

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
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
pub struct BatchCreateMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchCreateMailGroupMemberBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateMailGroupMemberBody {
    pub members: Vec<MailGroupMemberItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMemberItem {
    pub member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupMemberResponse {
    pub data: Option<BatchCreateMailGroupMemberData>,
}

impl ApiResponseTrait for BatchCreateMailGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupMemberData {
    pub results: Vec<MailGroupMemberResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMemberResult {
    pub member_id: String,
    pub status: String,
}

impl BatchCreateMailGroupMemberRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchCreateMailGroupMemberBody::default(),
        }
    }

    pub fn members(mut self, members: Vec<MailGroupMemberItem>) -> Self {
        self.body.members = members;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchCreateMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreateMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/batch_create",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchCreateMailGroupMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "批量创建邮件组成员")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量创建邮件组成员", "响应数据为空")
        })
    }
}

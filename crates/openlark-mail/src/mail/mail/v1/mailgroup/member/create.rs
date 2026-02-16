//! 创建邮件组成员

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
pub struct CreateMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: CreateMailGroupMemberBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMailGroupMemberBody {
    pub member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailGroupMemberResponse {
    pub data: Option<MailGroupMemberData>,
}

impl ApiResponseTrait for CreateMailGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMemberData {
    pub member_id: String,
}

impl CreateMailGroupMemberRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: CreateMailGroupMemberBody::default(),
        }
    }

    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.body.member_id = member_id.into();
        self
    }

    pub fn member_type(mut self, member_type: impl Into<String>) -> Self {
        self.body.member_type = Some(member_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<CreateMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members",
            self.mailgroup_id
        );
        let req: ApiRequest<CreateMailGroupMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "创建邮件组成员")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("创建邮件组成员", "响应数据为空"))
    }
}

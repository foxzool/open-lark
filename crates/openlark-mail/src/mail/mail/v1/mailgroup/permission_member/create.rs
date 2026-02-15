//! 创建邮件组权限成员

use crate::common::api_utils::serialize_params;
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
pub struct CreateMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: CreateMailGroupPermissionMemberBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMailGroupPermissionMemberBody {
    pub member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailGroupPermissionMemberResponse {
    pub data: Option<PermissionMemberData>,
}

impl ApiResponseTrait for CreateMailGroupPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMemberData {
    pub permission_member_id: String,
}

impl CreateMailGroupPermissionMemberRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: CreateMailGroupPermissionMemberBody::default(),
        }
    }

    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.body.member_id = member_id.into();
        self
    }

    pub async fn execute(self) -> SDKResult<CreateMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateMailGroupPermissionMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members",
            self.mailgroup_id
        );
        let req: ApiRequest<CreateMailGroupPermissionMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "请求")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("创建邮件组权限成员", "响应数据为空")
        })
    }
}

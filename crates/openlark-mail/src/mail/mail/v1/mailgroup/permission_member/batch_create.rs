//! 批量创建邮件组权限成员

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
pub struct BatchCreateMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchCreateMailGroupPermissionMemberBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateMailGroupPermissionMemberBody {
    pub members: Vec<PermissionMemberItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMemberItem {
    pub member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupPermissionMemberResponse {
    pub data: Option<BatchCreatePermissionMemberData>,
}

impl ApiResponseTrait for BatchCreateMailGroupPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreatePermissionMemberData {
    pub results: Vec<PermissionMemberResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMemberResult {
    pub member_id: String,
    pub status: String,
}

impl BatchCreateMailGroupPermissionMemberRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchCreateMailGroupPermissionMemberBody::default(),
        }
    }

    pub fn members(mut self, members: Vec<PermissionMemberItem>) -> Self {
        self.body.members = members;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchCreateMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreateMailGroupPermissionMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members/batch_create",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchCreateMailGroupPermissionMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "请求")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量创建邮件组权限成员", "响应数据为空")
        })
    }
}

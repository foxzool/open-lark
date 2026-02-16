//! 批量删除邮件组权限成员

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
pub struct BatchDeleteMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchDeleteMailGroupPermissionMemberBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteMailGroupPermissionMemberBody {
    pub permission_member_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteMailGroupPermissionMemberResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchDeleteMailGroupPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchDeleteMailGroupPermissionMemberRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchDeleteMailGroupPermissionMemberBody::default(),
        }
    }

    pub fn permission_member_ids(mut self, ids: Vec<String>) -> Self {
        self.body.permission_member_ids = ids;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchDeleteMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchDeleteMailGroupPermissionMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members/batch_delete",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchDeleteMailGroupPermissionMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "批量删除邮件组权限成员")?);

        let _resp: openlark_core::api::Response<BatchDeleteMailGroupPermissionMemberResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(BatchDeleteMailGroupPermissionMemberResponse { data: None })
    }
}

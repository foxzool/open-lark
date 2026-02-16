//! 批量获取邮件组权限成员

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
pub struct ListMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMailGroupPermissionMemberResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListMailGroupPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListMailGroupPermissionMemberRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<ListMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListMailGroupPermissionMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members",
            self.mailgroup_id
        );
        let req: ApiRequest<ListMailGroupPermissionMemberResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量获取邮件组权限成员", "响应数据为空")
        })
    }
}

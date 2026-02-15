//! 批量获取邮件组管理员

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
pub struct ListMailGroupManagerRequest {
    config: Arc<Config>,
    mailgroup_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMailGroupManagerResponse {
    pub data: Option<ListMailGroupManagerData>,
}

impl ApiResponseTrait for ListMailGroupManagerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMailGroupManagerData {
    pub managers: Vec<MailGroupManager>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupManager {
    pub manager_id: String,
    pub manager_email: String,
}

impl ListMailGroupManagerRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<ListMailGroupManagerResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListMailGroupManagerResponse> {
        let path = format!("/open-apis/mail/v1/mailgroups/{}/managers", self.mailgroup_id);
        let req: ApiRequest<ListMailGroupManagerResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量获取邮件组管理员", "响应数据为空")
        })
    }
}

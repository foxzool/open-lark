//! 删除访问控制
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar-acl/delete

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_CALENDARS};

/// 删除访问控制请求
pub struct DeleteCalendarAclRequest {
    config: Config,
    calendar_id: String,
    acl_id: String,
}

impl DeleteCalendarAclRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
            acl_id: String::new(),
        }
    }

    /// 日历 ID（路径参数）
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.calendar_id = calendar_id.into();
        self
    }

    /// ACL ID（路径参数）
    pub fn acl_id(mut self, acl_id: impl Into<String>) -> Self {
        self.acl_id = acl_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar-acl/delete
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");
        validate_required!(self.acl_id, "acl_id 不能为空");

        // url: DELETE:/open-apis/calendar/v4/calendars/:calendar_id/acls/:acl_id
        let req: ApiRequest<serde_json::Value> = ApiRequest::delete(format!(
            "{}/{}/acls/{}",
            CALENDAR_V4_CALENDARS, self.calendar_id, self.acl_id
        ));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除访问控制")
    }
}


//! 取消订阅日历访问控制变更事件
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar-acl/unsubscription

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_CALENDARS};

/// 取消订阅日历访问控制变更事件请求
pub struct CalendarAclUnsubscriptionRequest {
    config: Config,
    calendar_id: String,
}

impl CalendarAclUnsubscriptionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
        }
    }

    /// 日历 ID（路径参数）
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.calendar_id = calendar_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar-acl/unsubscription
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");

        // url: POST:/open-apis/calendar/v4/calendars/:calendar_id/acls/unsubscription
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(format!(
            "{}/{}/acls/unsubscription",
            CALENDAR_V4_CALENDARS, self.calendar_id
        ));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "取消订阅日历访问控制变更事件")
    }
}


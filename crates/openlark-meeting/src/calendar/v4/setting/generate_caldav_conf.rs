//! 生成 CalDAV 配置
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/setting/generate_caldav_conf

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_SETTINGS};

/// 生成 CalDAV 配置请求
pub struct GenerateCaldavConfRequest {
    config: Config,
}

impl GenerateCaldavConfRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/setting/generate_caldav_conf
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/settings/generate_caldav_conf
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/generate_caldav_conf", CALENDAR_V4_SETTINGS));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "生成 CalDAV 配置")
    }
}

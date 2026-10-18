//! 获取机器人信息。
//!
//! `GET /open-apis/bot/v3/info`，只接受 tenant access token。
//! 响应里的机器人对象在 `bot`，少数回包放在 `data`。

use std::sync::Arc;

use openlark_core::SDKResult;
use openlark_core::api::{ApiRequest, ApiResponseTrait, ResponseFormat};
use openlark_core::config::Config;
use openlark_core::constants::AccessTokenType;
use openlark_core::http::Transport;
use openlark_core::req_option::RequestOption;
use serde::Deserialize;

const BOT_V3_INFO: &str = "/open-apis/bot/v3/info";

/// 获取当前应用机器人信息。
#[derive(Debug, Clone)]
pub struct GetBotInfoRequest {
    config: Arc<Config>,
}

/// 获取机器人信息的响应。
///
/// 官方回包把机器人放在 `bot`。`data` 留下是为了兼容把同一对象包在 `data` 里的回包。
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GetBotInfoResponse {
    /// 机器人信息。
    #[serde(default)]
    pub bot: Option<BotInfo>,
    /// 兼容字段。`bot` 没有可用 `open_id` 时再读这里。
    #[serde(default)]
    pub data: Option<BotInfo>,
}

impl ApiResponseTrait for GetBotInfoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

/// 机器人信息。
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct BotInfo {
    /// 机器人激活状态。
    #[serde(default)]
    pub activate_status: Option<i32>,
    /// 应用名称。
    #[serde(default)]
    pub app_name: Option<String>,
    /// 头像地址。
    #[serde(default)]
    pub avatar_url: Option<String>,
    /// IP 白名单。
    #[serde(default)]
    pub ip_white_list: Option<Vec<String>>,
    /// 机器人 `open_id`。
    #[serde(default)]
    pub open_id: String,
}

impl GetBotInfoResponse {
    /// 先取 `bot.open_id`，没有再取 `data.open_id`。空白视为没有。
    pub fn open_id(&self) -> Option<&str> {
        non_blank_open_id(self.bot.as_ref()).or_else(|| non_blank_open_id(self.data.as_ref()))
    }
}

impl GetBotInfoRequest {
    /// 用这份配置创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetBotInfoResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetBotInfoResponse> {
        Transport::request_typed(
            self.build_request(),
            &self.config,
            Some(option),
            "获取机器人信息",
        )
        .await
    }

    pub(crate) fn build_request(&self) -> ApiRequest<GetBotInfoResponse> {
        ApiRequest::get(BOT_V3_INFO)
            .with_supported_access_token_types(vec![AccessTokenType::Tenant])
    }
}

fn non_blank_open_id(info: Option<&BotInfo>) -> Option<&str> {
    info.map(|bot| bot.open_id.trim())
        .filter(|id| !id.is_empty())
}

#[cfg(test)]
mod tests {
    use openlark_core::api::HttpMethod;
    use openlark_core::config::Config;

    use super::*;
    use crate::BotService;

    #[test]
    fn bot_info_request_is_tenant_get() {
        let request = BotService::new(Config::default()).bot_info();
        let api = request.build_request();
        assert_eq!(api.api_path(), "/open-apis/bot/v3/info");
        assert_eq!(api.method(), &HttpMethod::Get);
        assert_eq!(
            api.supported_access_token_types(),
            vec![AccessTokenType::Tenant]
        );
    }

    #[test]
    fn bot_info_open_id_prefers_bot_then_data() {
        let from_bot: GetBotInfoResponse = serde_json::from_str(
            r#"{"code":0,"msg":"ok","bot":{"activate_status":2,"app_name":"助手","avatar_url":"https://example.com/a","ip_white_list":[],"open_id":"ou_bot"},"data":{"open_id":"ou_data"}}"#,
        )
        .expect("bot payload");
        assert_eq!(from_bot.open_id(), Some("ou_bot"));
        assert_eq!(
            from_bot
                .bot
                .as_ref()
                .and_then(|bot| bot.app_name.as_deref()),
            Some("助手")
        );

        let from_data: GetBotInfoResponse =
            serde_json::from_str(r#"{"code":0,"msg":"ok","data":{"open_id":"ou_data"}}"#)
                .expect("data payload");
        assert_eq!(from_data.open_id(), Some("ou_data"));

        let blank_bot: GetBotInfoResponse = serde_json::from_str(
            r#"{"code":0,"msg":"ok","bot":{"open_id":"  "},"data":{"open_id":"ou_data"}}"#,
        )
        .expect("blank bot");
        assert_eq!(blank_bot.open_id(), Some("ou_data"));

        let neither: GetBotInfoResponse =
            serde_json::from_str(r#"{"code":0,"msg":"ok","bot":{"open_id":""}}"#).expect("empty");
        assert_eq!(neither.open_id(), None);
    }
}

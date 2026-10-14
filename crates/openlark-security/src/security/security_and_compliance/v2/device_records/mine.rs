//! 通过授权获取设备信息
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v2/device_record/mine>
//!
//! 权限点 `security_and_compliance:device_record:read`。Authorization **仅** `user_access_token`。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 通过授权获取设备信息。
///
/// GET `/open-apis/security_and_compliance/v2/device_records/mine`
pub const DEVICE_RECORDS_MINE: &str = "/open-apis/security_and_compliance/v2/device_records/mine";

/// 通过授权获取设备信息的响应 `data`。
///
/// `device_ownership`：`0` 未知、`1` 个人设备、`2` 企业设备。
/// `device_status`：`0` 未知、`1` 信任设备、`2` 非信任设备。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MineDeviceRecordResponse {
    /// 设备认证编码。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_record_id: Option<String>,
    /// 设备归属。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ownership: Option<i32>,
    /// 可信状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status: Option<i32>,
}

impl ApiResponseTrait for MineDeviceRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 通过授权获取设备信息请求。
#[derive(Debug, Clone)]
pub struct MineDeviceRecordRequest {
    /// 配置信息。
    config: Config,
}

impl MineDeviceRecordRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<MineDeviceRecordResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<MineDeviceRecordResponse> {
        let req: ApiRequest<MineDeviceRecordResponse> = ApiRequest::get(DEVICE_RECORDS_MINE)
            .with_supported_access_token_types(vec![AccessTokenType::User]);
        Transport::request_typed(req, &self.config, Some(option), "通过授权获取设备信息").await
    }
}

/// 旧名。2026-10-12 起该接口返回单台授权设备，请改用 [`MineDeviceRecordRequest`]。
#[deprecated(note = "请改用 MineDeviceRecordRequest；分页查询参数已从官方接口移除")]
pub type GetMyDeviceRecordsRequest = MineDeviceRecordRequest;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn user_option() -> RequestOption {
        RequestOption::builder()
            .user_access_token("user-token")
            .build()
    }

    #[test]
    fn mine_endpoint_constant() {
        assert_eq!(
            DEVICE_RECORDS_MINE,
            "/open-apis/security_and_compliance/v2/device_records/mine"
        );
    }

    #[test]
    fn mine_response_roundtrip() {
        let raw = json!({
            "device_record_id": "7089353870308032531",
            "device_ownership": 1,
            "device_status": 2
        });
        let parsed: MineDeviceRecordResponse = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(
            parsed.device_record_id.as_deref(),
            Some("7089353870308032531")
        );
        assert_eq!(parsed.device_ownership, Some(1));
        assert_eq!(parsed.device_status, Some(2));
        assert_eq!(serde_json::to_value(&parsed).unwrap(), raw);
    }

    /// 端到端：GET .../device_records/mine + user token + 单设备响应。
    #[tokio::test]
    async fn test_mine_device_record_returns_authorized_device() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/security_and_compliance/v2/device_records/mine",
            ))
            .and(header("authorization", "Bearer user-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "device_record_id": "7089353870308032531",
                    "device_ownership": 0,
                    "device_status": 1
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = MineDeviceRecordRequest::new(config)
            .execute_with_options(user_option())
            .await
            .expect("通过授权获取设备信息应成功");
        assert_eq!(
            data.device_record_id.as_deref(),
            Some("7089353870308032531")
        );
        assert_eq!(data.device_ownership, Some(0));
        assert_eq!(data.device_status, Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert!(received[0].url.query().unwrap_or("").is_empty());
    }
}

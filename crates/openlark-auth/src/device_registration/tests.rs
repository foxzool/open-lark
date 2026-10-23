//! device_registration 测试（替身 HTTP，不访问真实飞书，#695 AC4）

use super::{AppCredentials, DeviceRegistration, PollOutcome, RegistrationError};
use openlark_core::config::Config;
use openlark_core::constants::{AppType, FEISHU_BASE_URL};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string_contains, method, path},
};

fn registration_config(accounts_origin: impl Into<String>) -> Config {
    Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .app_type(AppType::SelfBuild)
        .oauth_base_url(accounts_origin)
        .build()
}

fn endpoint_mock(form_contains: &str, body: serde_json::Value) -> Mock {
    Mock::given(method("POST"))
        .and(path("/oauth/v1/app/registration"))
        .and(body_string_contains(form_contains))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
}

fn begin_body() -> serde_json::Value {
    json!({
        "device_code": "dc-1",
        "verification_uri_complete": "https://accounts.feishu.cn/qr?ticket=t1",
        "interval": 7,
        "expire_in": 300
    })
}

async fn forms_of(server: &MockServer) -> Vec<String> {
    server
        .received_requests()
        .await
        .unwrap_or_default()
        .iter()
        .map(|req| String::from_utf8_lossy(&req.body).to_string())
        .collect()
}

/// 端点解析三分支：显式 oauth_base_url / 默认 feishu 域 / 自定义 base_url 无覆盖。
#[test]
fn endpoint_resolution_branches() {
    assert!(DeviceRegistration::new(&registration_config("http://127.0.0.1:9")).is_ok());

    let default_config = Config::builder()
        .app_id("a")
        .app_secret("s")
        .base_url(FEISHU_BASE_URL)
        .build();
    assert!(DeviceRegistration::new(&default_config).is_ok());

    let custom = Config::builder()
        .app_id("a")
        .app_secret("s")
        .allow_custom_base_url(true)
        .base_url("https://my.proxy.example")
        .build();
    assert!(matches!(
        DeviceRegistration::new(&custom),
        Err(RegistrationError::Configuration { .. })
    ));
}

/// AC1：begin 发起设备授权——请求体带固定 begin 参数，返回句柄带
/// 扫码 URL（追加 from/tp/source 标识并保留原 query）与归一化的间隔/有效期。
#[tokio::test]
async fn begin_returns_qr_url_and_handle() {
    let server = MockServer::start().await;
    server
        .register(endpoint_mock("action=begin", begin_body()))
        .await;

    let reg = DeviceRegistration::new(&registration_config(server.uri())).expect("构造");
    let auth = reg.begin().await.expect("begin");

    assert_eq!(auth.device_code, "dc-1");
    assert_eq!(auth.interval_secs, 7);
    assert_eq!(auth.expire_in_secs, 300);
    assert!(
        auth.qr_url
            .starts_with("https://accounts.feishu.cn/qr?ticket=t1&"),
        "应保留原 query，got {}",
        auth.qr_url
    );
    assert!(auth.qr_url.contains("from=sdk"), "{}", auth.qr_url);
    assert!(auth.qr_url.contains("tp=sdk"), "{}", auth.qr_url);
    assert!(auth.qr_url.contains("source=openlark"), "{}", auth.qr_url);

    let forms = forms_of(&server).await;
    assert_eq!(forms.len(), 1);
    for part in [
        "action=begin",
        "archetype=PersonalAgent",
        "auth_method=client_secret",
        "request_user_info=open_id",
    ] {
        assert!(
            forms[0].contains(part),
            "begin form 缺 {part}: {}",
            forms[0]
        );
    }
}

/// QR URL 已带 from/tp/source 同名键时覆盖而非重复追加。
#[tokio::test]
async fn begin_qr_decoration_overrides_existing_keys() {
    let server = MockServer::start().await;
    server
        .register(endpoint_mock(
            "action=begin",
            json!({
                "device_code": "dc-3",
                "verification_uri_complete":
                    "https://accounts.feishu.cn/qr?ticket=t2&source=other&keep=1"
            }),
        ))
        .await;

    let reg = DeviceRegistration::new(&registration_config(server.uri())).expect("构造");
    let auth = reg.begin().await.expect("begin");
    assert!(auth.qr_url.contains("ticket=t2"), "{}", auth.qr_url);
    assert!(
        auth.qr_url.contains("keep=1"),
        "无关 query 应保留: {}",
        auth.qr_url
    );
    assert!(auth.qr_url.contains("source=openlark"), "{}", auth.qr_url);
    assert!(
        !auth.qr_url.contains("source=other"),
        "同名键应被覆盖: {}",
        auth.qr_url
    );
    assert_eq!(
        auth.qr_url.matches("source=").count(),
        1,
        "source 键应恰好一个: {}",
        auth.qr_url
    );
}

/// begin 缺 interval/expire_in 时用默认值兜底。
#[tokio::test]
async fn begin_normalizes_missing_intervals() {
    let server = MockServer::start().await;
    server
        .register(endpoint_mock(
            "action=begin",
            json!({
                "device_code": "dc-2",
                "verification_uri_complete": "https://accounts.feishu.cn/qr"
            }),
        ))
        .await;

    let reg = DeviceRegistration::new(&registration_config(server.uri())).expect("构造");
    let auth = reg.begin().await.expect("begin");
    assert_eq!(auth.interval_secs, 5);
    assert_eq!(auth.expire_in_secs, 600);
}

/// begin 响应缺 device_code → InvalidResponse（不访问后续流程）。
#[tokio::test]
async fn begin_missing_device_code_is_invalid_response() {
    let server = MockServer::start().await;
    server
        .register(endpoint_mock(
            "action=begin",
            json!({"verification_uri_complete": "https://x"}),
        ))
        .await;

    let reg = DeviceRegistration::new(&registration_config(server.uri())).expect("构造");
    assert!(matches!(
        reg.begin().await,
        Err(RegistrationError::InvalidResponse { .. })
    ));
}

/// AC1（后半）：授权完成后 poll 返回凭证（含扫码用户信息）。
#[tokio::test]
async fn poll_ready_returns_credentials() {
    let server = MockServer::start().await;
    server
        .register(endpoint_mock("action=begin", begin_body()))
        .await;
    server
        .register(endpoint_mock(
            "action=poll",
            json!({
                "client_id": "cli_new",
                "client_secret": "sec_new",
                "user_info": {"open_id": "ou_scanner", "tenant_brand": "feishu"}
            }),
        ))
        .await;

    let reg = DeviceRegistration::new(&registration_config(server.uri())).expect("构造");
    let mut auth = reg.begin().await.expect("begin");
    let outcome = reg.poll(&mut auth).await.expect("poll");
    assert_eq!(
        outcome,
        PollOutcome::Ready(AppCredentials {
            client_id: "cli_new".to_string(),
            client_secret: "sec_new".to_string(),
            user_open_id: Some("ou_scanner".to_string()),
            tenant_brand: Some("feishu".to_string()),
        })
    );

    let forms = forms_of(&server).await;
    assert_eq!(forms.len(), 2);
    assert!(forms[1].contains("action=poll"), "{}", forms[1]);
    assert!(forms[1].contains("device_code=dc-1"), "{}", forms[1]);
}

/// AC2：续态与终态可区分——pending / slow_down / 无 error 无凭证。
#[tokio::test]
async fn poll_distinguishes_pending_and_slow_down() {
    let server = MockServer::start().await;
    server
        .register(endpoint_mock("action=begin", begin_body()))
        .await;
    server
        .register(endpoint_mock(
            "action=poll",
            json!({"error": "authorization_pending"}),
        ))
        .await;

    let reg = DeviceRegistration::new(&registration_config(server.uri())).expect("构造");
    let mut auth = reg.begin().await.expect("begin");
    assert_eq!(
        reg.poll(&mut auth).await.expect("pending"),
        PollOutcome::Pending
    );

    server.reset().await;
    server
        .register(endpoint_mock("action=poll", json!({"error": "slow_down"})))
        .await;
    assert_eq!(
        reg.poll(&mut auth).await.expect("slow_down #1"),
        PollOutcome::SlowDown { interval_secs: 12 },
        "建议间隔应为句柄 interval(7) + 5"
    );
    assert_eq!(auth.interval_secs, 12, "句柄应累计降速间隔");
    assert_eq!(
        reg.poll(&mut auth).await.expect("slow_down #2"),
        PollOutcome::SlowDown { interval_secs: 17 },
        "连续 slow_down 应继续递增（对齐官方 7→12→17）"
    );
    assert_eq!(auth.interval_secs, 17);

    server.reset().await;
    server
        .register(endpoint_mock("action=poll", json!({})))
        .await;
    assert_eq!(
        reg.poll(&mut auth).await.expect("空响应按 Node 行为续轮询"),
        PollOutcome::Pending
    );
}

/// AC2：终态错误独立变体——access_denied / expired_token / 未知码。
#[tokio::test]
async fn poll_terminal_errors_are_distinct_variants() {
    let server = MockServer::start().await;
    server
        .register(endpoint_mock("action=begin", begin_body()))
        .await;
    server
        .register(endpoint_mock(
            "action=poll",
            json!({"error": "access_denied", "error_description": "user rejected"}),
        ))
        .await;

    let reg = DeviceRegistration::new(&registration_config(server.uri())).expect("构造");
    let mut auth = reg.begin().await.expect("begin");
    match reg.poll(&mut auth).await {
        Err(RegistrationError::AccessDenied { description }) => {
            assert_eq!(description, "user rejected");
        }
        other => panic!("expected AccessDenied, got {other:?}"),
    }

    server.reset().await;
    server
        .register(endpoint_mock(
            "action=poll",
            json!({"error": "expired_token"}),
        ))
        .await;
    assert!(matches!(
        reg.poll(&mut auth).await,
        Err(RegistrationError::ExpiredToken { .. })
    ));

    server.reset().await;
    server
        .register(endpoint_mock(
            "action=poll",
            json!({"error": "server_on_moon"}),
        ))
        .await;
    assert!(matches!(
        reg.poll(&mut auth).await,
        Err(RegistrationError::InvalidResponse { .. })
    ));
}

/// 非 2xx / 空 body / 非法 JSON → InvalidResponse。
#[tokio::test]
async fn malformed_responses_are_invalid_response() {
    let server = MockServer::start().await;
    server
        .register(endpoint_mock("action=begin", begin_body()))
        .await;
    server
        .register(
            Mock::given(method("POST"))
                .and(path("/oauth/v1/app/registration"))
                .and(body_string_contains("action=poll"))
                .respond_with(ResponseTemplate::new(500)),
        )
        .await;

    let reg = DeviceRegistration::new(&registration_config(server.uri())).expect("构造");
    let mut auth = reg.begin().await.expect("begin");
    assert!(matches!(
        reg.poll(&mut auth).await,
        Err(RegistrationError::InvalidResponse { .. })
    ));
}

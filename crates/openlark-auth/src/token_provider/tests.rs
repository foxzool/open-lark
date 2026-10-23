use super::AuthTokenProvider;
use openlark_core::{
    auth::{TokenProvider, TokenRequest},
    config::Config,
    constants::{AccessTokenType, AppType},
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path},
};

fn marketplace_config(base_url: impl Into<String>) -> Config {
    Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .app_type(AppType::Marketplace)
        .base_url(base_url)
        .build()
}

#[tokio::test]
async fn tenant_token_fetch_no_longer_uses_noop_provider() {
    let config = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url("http://127.0.0.1:9")
        .build();

    let provider = AuthTokenProvider::new(config);
    let err = provider
        .get_token(TokenRequest::tenant())
        .await
        .expect_err("should fail on unreachable test endpoint");

    assert!(!err.to_string().contains("NoOpTokenProvider"));
}

#[tokio::test]
async fn tenant_cache_key_should_include_tenant_context() {
    let request = TokenRequest::tenant().tenant_key("tenant_key_001");

    let key = AuthTokenProvider::cache_key(&AccessTokenType::Tenant, &AppType::SelfBuild, &request);

    assert!(key.starts_with("Tenant_SelfBuild_ctx_"));
    assert!(!key.contains("tenant_key_001"));
    assert_eq!(
        key,
        AuthTokenProvider::cache_key(&AccessTokenType::Tenant, &AppType::SelfBuild, &request)
    );
    for different_request in [
        TokenRequest::tenant().tenant_key("tenant_key_002"),
        TokenRequest::tenant()
            .tenant_key("tenant_key_001")
            .app_ticket("ticket_001"),
    ] {
        assert_ne!(
            key,
            AuthTokenProvider::cache_key(
                &AccessTokenType::Tenant,
                &AppType::SelfBuild,
                &different_request
            )
        );
    }
    assert_ne!(
        AuthTokenProvider::cache_key(
            &AccessTokenType::Tenant,
            &AppType::Marketplace,
            &TokenRequest::tenant()
        ),
        AuthTokenProvider::cache_key(
            &AccessTokenType::Tenant,
            &AppType::Marketplace,
            &TokenRequest::tenant().tenant_key("default")
        )
    );
}

#[tokio::test]
async fn app_cache_key_should_include_app_ticket_for_marketplace() {
    let request = TokenRequest::app().app_ticket("ticket_001");

    let key = AuthTokenProvider::cache_key(&AccessTokenType::App, &AppType::Marketplace, &request);

    assert!(key.starts_with("App_Marketplace_"));
    assert!(!key.contains("ticket_001"));
    assert_eq!(
        key,
        AuthTokenProvider::cache_key(&AccessTokenType::App, &AppType::Marketplace, &request)
    );
    assert_ne!(
        key,
        AuthTokenProvider::cache_key(
            &AccessTokenType::App,
            &AppType::Marketplace,
            &TokenRequest::app().app_ticket("ticket_002")
        )
    );
}

#[tokio::test]
async fn self_build_app_cache_key_should_remain_stable() {
    assert_eq!(
        AuthTokenProvider::cache_key(
            &AccessTokenType::App,
            &AppType::SelfBuild,
            &TokenRequest::app()
        ),
        AuthTokenProvider::cache_key(
            &AccessTokenType::App,
            &AppType::SelfBuild,
            &TokenRequest::app()
                .tenant_key("tenant_001")
                .app_ticket("ticket_001")
        )
    );
}

#[tokio::test]
async fn marketplace_cache_isolates_tenants_and_ticket_rotation() {
    let server = MockServer::start().await;
    for (ticket, app_token) in [("ticket_a", "app-token-a"), ("ticket_b", "app-token-b")] {
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token"))
            .and(body_json(json!({
                "app_id": "test_app_id",
                "app_secret": "test_app_secret",
                "app_ticket": ticket
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "app_access_token": app_token, "expire": 7200
            })))
            .expect(1)
            .mount(&server)
            .await;
    }
    let contexts = [
        ("tenant_a", "ticket_a", "app-token-a", "tenant-token-a"),
        ("tenant_b", "ticket_a", "app-token-a", "tenant-token-b"),
        (
            "tenant_a",
            "ticket_b",
            "app-token-b",
            "tenant-token-rotated",
        ),
    ];
    for (tenant, _, app_token, tenant_token) in contexts {
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/tenant_access_token"))
            .and(body_json(json!({
                "app_access_token": app_token, "tenant_key": tenant
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "tenant_access_token": tenant_token, "expire": 7200
            })))
            .expect(1)
            .mount(&server)
            .await;
    }
    let provider = AuthTokenProvider::new(marketplace_config(server.uri()));
    for _ in 0..2 {
        for (tenant, ticket, _, tenant_token) in contexts {
            let token = provider
                .get_token(TokenRequest::tenant().tenant_key(tenant).app_ticket(ticket))
                .await
                .expect("tenant token should be fetched or reused within its own context");
            assert_eq!(token, tenant_token);
        }
    }
    let debug_output = format!("{provider:?}");
    for sensitive in [
        "tenant_a",
        "tenant_b",
        "ticket_a",
        "ticket_b",
        "app-token-a",
        "app-token-b",
        "tenant-token-a",
        "tenant-token-b",
        "tenant-token-rotated",
    ] {
        assert!(!debug_output.contains(sensitive));
    }
}

#[tokio::test]
async fn debug_output_should_not_include_marketplace_app_ticket() {
    let provider = AuthTokenProvider::new(marketplace_config("http://127.0.0.1:9"));
    let _ = provider
        .get_token(TokenRequest::app().app_ticket("ticket_001"))
        .await;

    let debug_output = format!("{provider:?}");

    assert!(!debug_output.contains("ticket_001"));
}

#[tokio::test]
async fn marketplace_app_token_requires_app_ticket() {
    let provider = AuthTokenProvider::new(marketplace_config("http://127.0.0.1:9"));

    let err = provider
        .get_token(TokenRequest::app())
        .await
        .expect_err("marketplace app token without app_ticket should fail before network");

    assert!(err.to_string().contains("app_ticket"));
}

#[tokio::test]
async fn marketplace_tenant_token_requires_tenant_key() {
    let provider = AuthTokenProvider::new(marketplace_config("http://127.0.0.1:9"));

    let err = provider
        .get_token(TokenRequest::tenant().app_ticket("ticket_001"))
        .await
        .expect_err("marketplace tenant token without tenant_key should fail before network");

    assert!(err.to_string().contains("tenant_key"));
}

#[tokio::test]
async fn marketplace_tenant_token_uses_app_token_and_tenant_key_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/open-apis/auth/v3/app_access_token"))
        .and(body_json(json!({
            "app_id": "test_app_id",
            "app_secret": "test_app_secret",
            "app_ticket": "ticket_001"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "app_access_token": "market-app-token",
            "expire": 7200
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/open-apis/auth/v3/tenant_access_token"))
        .and(body_json(json!({
            "app_access_token": "market-app-token",
            "tenant_key": "tenant_key_001"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "tenant_access_token": "tenant-token",
            "expire": 7200
        })))
        .mount(&server)
        .await;

    let provider = AuthTokenProvider::new(marketplace_config(server.uri()));

    let token = provider
        .get_token(
            TokenRequest::tenant()
                .tenant_key("tenant_key_001")
                .app_ticket("ticket_001"),
        )
        .await
        .expect("marketplace tenant token should be fetched through app token");

    assert_eq!(token, "tenant-token");

    let received_requests = server.received_requests().await.unwrap_or_default();
    assert_eq!(received_requests.len(), 2);
    assert!(
        received_requests
            .iter()
            .all(|request| !request.headers.contains_key("authorization"))
    );
}

mod client_assertion_exchange {
    use super::super::AuthTokenProvider;
    use crate::token_provider::client_assertion::{
        self, assertion_cache_key, resolve_oauth_endpoint,
    };
    use openlark_core::auth::TokenRequest;
    use openlark_core::auth::{
        CLIENT_ASSERTION_APP_TOKEN_UNAVAILABLE, CLIENT_ASSERTION_CUSTOM_BASE_URL,
        CLIENT_ASSERTION_EMPTY, CLIENT_ASSERTION_MARKETPLACE_UNSUPPORTED,
        CLIENT_ASSERTION_OAUTH_BASE_INVALID, CLIENT_ASSERTION_OAUTH_FAILED_PREFIX,
        CLIENT_ASSERTION_PROVIDER_FAILED_PREFIX, ClientAssertionProvider, TokenProvider,
    };
    use openlark_core::config::Config;
    use openlark_core::constants::{AccessTokenType, AppType};
    use openlark_core::constants::{FEISHU_BASE_URL, LARK_BASE_URL};
    use openlark_core::error::{CoreError, network_error};
    use serde_json::json;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:jwt-bearer";
    const ASSERTION_TYPE: &str = "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";
    const OLD_GRANT_PATHS: [&str; 4] = [
        "/open-apis/auth/v3/tenant_access_token/internal",
        "/open-apis/auth/v3/app_access_token/internal",
        "/open-apis/auth/v3/tenant_access_token",
        "/open-apis/auth/v3/app_access_token",
    ];

    #[derive(Debug)]
    struct FixedAssertion {
        value: String,
        fail: Option<&'static str>,
        calls: Arc<AtomicUsize>,
        auds: Arc<Mutex<Vec<String>>>,
    }

    impl FixedAssertion {
        fn ok(value: impl Into<String>) -> (Self, Arc<AtomicUsize>, Arc<Mutex<Vec<String>>>) {
            let calls = Arc::new(AtomicUsize::new(0));
            let auds = Arc::new(Mutex::new(Vec::new()));
            (
                Self {
                    value: value.into(),
                    fail: None,
                    calls: Arc::clone(&calls),
                    auds: Arc::clone(&auds),
                },
                calls,
                auds,
            )
        }

        fn fail(message: &'static str) -> (Self, Arc<AtomicUsize>) {
            let calls = Arc::new(AtomicUsize::new(0));
            (
                Self {
                    value: String::new(),
                    fail: Some(message),
                    calls: Arc::clone(&calls),
                    auds: Arc::new(Mutex::new(Vec::new())),
                },
                calls,
            )
        }
    }

    impl ClientAssertionProvider for FixedAssertion {
        fn retrieve_assertion<'a>(
            &'a self,
            aud: &'a str,
        ) -> Pin<Box<dyn Future<Output = openlark_core::SDKResult<String>> + Send + 'a>> {
            Box::pin(async move {
                self.calls.fetch_add(1, Ordering::SeqCst);
                self.auds.lock().expect("aud lock").push(aud.to_string());
                if let Some(message) = self.fail {
                    return Err(network_error(message));
                }
                Ok(self.value.clone())
            })
        }
    }

    fn aud_of(uri: &str) -> String {
        let url = url::Url::parse(uri).expect("mock uri");
        match url.port() {
            Some(port) => format!("{}:{port}", url.host_str().expect("host")),
            None => url.host_str().expect("host").to_string(),
        }
    }

    fn assertion_ready_config(
        uri: &str,
        provider: FixedAssertion,
        app_secret: Option<&str>,
    ) -> Config {
        let mut builder = Config::builder()
            .app_id("cli_demo")
            .app_type(AppType::SelfBuild)
            .base_url(uri)
            .oauth_base_url(uri)
            .allow_custom_base_url(true)
            .client_assertion_provider(provider);
        if let Some(secret) = app_secret {
            builder = builder.app_secret(secret);
        }
        builder.build()
    }

    async fn mount_absent_grants(server: &MockServer) {
        for grant_path in OLD_GRANT_PATHS {
            Mock::given(method("POST"))
                .and(path(grant_path))
                .respond_with(ResponseTemplate::new(500))
                .expect(0)
                .mount(server)
                .await;
        }
    }

    async fn mount_oauth(server: &MockServer, status: u16, body: serde_json::Value, times: u64) {
        Mock::given(method("POST"))
            .and(path("/oauth/v3/token"))
            .and(body_json(json!({
                "grant_type": GRANT_TYPE,
                "client_assertion_type": ASSERTION_TYPE,
                "client_assertion": "fake-assertion",
                "client_id": "cli_demo"
            })))
            .respond_with(ResponseTemplate::new(status).set_body_json(body))
            .expect(times)
            .mount(server)
            .await;
    }

    fn error_chain(err: &CoreError) -> String {
        let mut text = err.to_string();
        let mut source = match err {
            CoreError::Network(net) => net
                .source
                .as_ref()
                .map(|item| item.as_ref() as &dyn std::error::Error),
            _ => None,
        };
        while let Some(inner) = source {
            text.push(' ');
            text.push_str(&inner.to_string());
            source = inner.source();
        }
        text
    }

    fn assert_configuration(err: &CoreError, sentence: &str) {
        assert!(
            matches!(err, CoreError::Configuration { .. }),
            "应变体是 Configuration，实际 {err:?}"
        );
        assert!(
            err.to_string().contains(sentence),
            "to_string 应包含 `{sentence}`，实际 {}",
            err
        );
    }

    fn oauth_bodies(server_requests: &[wiremock::Request]) -> Vec<serde_json::Value> {
        server_requests
            .iter()
            .filter(|request| request.url.path() == "/oauth/v3/token")
            .map(|request| serde_json::from_slice(&request.body).expect("oauth body 应是 JSON"))
            .collect()
    }

    #[tokio::test]
    async fn self_build_exchanges_exact_jwt_bearer_body_and_caches() {
        let server = MockServer::start().await;
        mount_absent_grants(&server).await;
        mount_oauth(
            &server,
            200,
            json!({"access_token": "tat_fake", "expires_in": 7200}),
            1,
        )
        .await;
        let (provider, calls, auds) = FixedAssertion::ok("fake-assertion");
        let token_provider =
            AuthTokenProvider::new(assertion_ready_config(&server.uri(), provider, None));

        let first = token_provider
            .get_token(TokenRequest::tenant())
            .await
            .expect("第一次换票应成功");
        let second = token_provider
            .get_token(TokenRequest::tenant())
            .await
            .expect("第二次应命中缓存");

        assert_eq!(first, "tat_fake");
        assert_eq!(second, "tat_fake");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            auds.lock().expect("aud").as_slice(),
            [aud_of(&server.uri())]
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert!(
            received
                .iter()
                .all(|request| request.url.path() == "/oauth/v3/token")
        );
        assert!(
            received
                .iter()
                .all(|request| !request.headers.contains_key("authorization"))
        );
        let content_type = received[0]
            .headers
            .get("content-type")
            .expect("content-type")
            .to_str()
            .expect("content-type str");
        assert!(
            content_type.starts_with("application/json"),
            "{content_type}"
        );
        let body = &oauth_bodies(&received)[0];
        let keys: std::collections::BTreeSet<_> =
            body.as_object().expect("object").keys().cloned().collect();
        assert_eq!(
            keys.into_iter().collect::<Vec<_>>(),
            vec![
                "client_assertion",
                "client_assertion_type",
                "client_id",
                "grant_type"
            ]
        );
        assert!(body.get("client_secret").is_none());
        assert!(body.get("app_secret").is_none());
    }

    #[tokio::test]
    async fn secret_plus_provider_still_uses_oauth_not_internal_grant() {
        let server = MockServer::start().await;
        mount_absent_grants(&server).await;
        mount_oauth(
            &server,
            200,
            json!({"access_token": "tat_fake", "expires_in": 7200}),
            1,
        )
        .await;
        let (provider, calls, _) = FixedAssertion::ok("fake-assertion");
        let token_provider = AuthTokenProvider::new(assertion_ready_config(
            &server.uri(),
            provider,
            Some("should-not-be-sent"),
        ));

        let token = token_provider
            .get_token(TokenRequest::tenant())
            .await
            .expect("有 secret 时仍应换票");
        assert_eq!(token, "tat_fake");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        let received = server.received_requests().await.unwrap_or_default();
        assert!(
            received
                .iter()
                .all(|request| request.url.path() == "/oauth/v3/token")
        );
        assert!(oauth_bodies(&received)[0].get("app_secret").is_none());
    }

    #[tokio::test]
    async fn trimmed_assertion_is_what_gets_posted() {
        let server = MockServer::start().await;
        mount_oauth(
            &server,
            200,
            json!({"access_token": "tat_fake", "expires_in": 7200}),
            1,
        )
        .await;
        let (provider, _, _) = FixedAssertion::ok("  fake-assertion\n");
        let token_provider =
            AuthTokenProvider::new(assertion_ready_config(&server.uri(), provider, None));
        let token = token_provider
            .get_token(TokenRequest::tenant())
            .await
            .expect("trim 后应换票");
        assert_eq!(token, "tat_fake");
    }

    #[tokio::test]
    async fn oauth_success_fields_win_over_odd_status_and_failure_is_not_cached() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/oauth/v3/token"))
            .respond_with(ResponseTemplate::new(418).set_body_json(json!({
                "access_token": "tat_fake",
                "expires_in": 7200
            })))
            .expect(1)
            .mount(&server)
            .await;
        let (provider, _, _) = FixedAssertion::ok("fake-assertion");
        let token_provider =
            AuthTokenProvider::new(assertion_ready_config(&server.uri(), provider, None));
        let token = token_provider
            .get_token(TokenRequest::tenant())
            .await
            .expect("有 access_token 与 expires_in 时应成功");
        assert_eq!(token, "tat_fake");

        let fail_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/oauth/v3/token"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": "invalid_grant",
                "error_description": "bad assertion"
            })))
            .expect(2)
            .mount(&fail_server)
            .await;
        let (provider, calls, _) = FixedAssertion::ok("fake-assertion");
        let token_provider =
            AuthTokenProvider::new(assertion_ready_config(&fail_server.uri(), provider, None));
        for _ in 0..2 {
            let err = token_provider
                .get_token(TokenRequest::tenant())
                .await
                .expect_err("坏响应不应缓存");
            assert_configuration(
                &err,
                &format!("{CLIENT_ASSERTION_OAUTH_FAILED_PREFIX}bad assertion"),
            );
            assert!(!matches!(err, CoreError::Api(_)));
        }
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn empty_or_failing_assertion_does_not_call_http() {
        for value in ["", "   \n"] {
            let server = MockServer::start().await;
            mount_absent_grants(&server).await;
            Mock::given(method("POST"))
                .and(path("/oauth/v3/token"))
                .respond_with(ResponseTemplate::new(500))
                .expect(0)
                .mount(&server)
                .await;
            let (provider, calls, _) = FixedAssertion::ok(value);
            let token_provider =
                AuthTokenProvider::new(assertion_ready_config(&server.uri(), provider, None));
            let err = token_provider
                .get_token(TokenRequest::tenant())
                .await
                .expect_err("空白 assertion 应失败");
            assert_configuration(&err, CLIENT_ASSERTION_EMPTY);
            assert_eq!(calls.load(Ordering::SeqCst), 1);
            assert!(
                server
                    .received_requests()
                    .await
                    .unwrap_or_default()
                    .is_empty()
            );
        }

        let server = MockServer::start().await;
        mount_absent_grants(&server).await;
        Mock::given(method("POST"))
            .and(path("/oauth/v3/token"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&server)
            .await;
        let (provider, calls) = FixedAssertion::fail("signer down");
        let token_provider =
            AuthTokenProvider::new(assertion_ready_config(&server.uri(), provider, None));
        let err = token_provider
            .get_token(TokenRequest::tenant())
            .await
            .expect_err("供给失败应包装");
        assert_configuration(
            &err,
            &format!("{CLIENT_ASSERTION_PROVIDER_FAILED_PREFIX}网络错误: signer down"),
        );
        assert!(!matches!(err, CoreError::Network(_)));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn app_token_and_marketplace_reject_before_http() {
        let server = MockServer::start().await;
        mount_absent_grants(&server).await;
        Mock::given(method("POST"))
            .and(path("/oauth/v3/token"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&server)
            .await;
        let (provider, calls, _) = FixedAssertion::ok("fake-assertion");
        let token_provider =
            AuthTokenProvider::new(assertion_ready_config(&server.uri(), provider, None));
        let err = token_provider
            .get_token(TokenRequest::app())
            .await
            .expect_err("app token 应被拒绝");
        assert_configuration(&err, CLIENT_ASSERTION_APP_TOKEN_UNAVAILABLE);
        assert_eq!(calls.load(Ordering::SeqCst), 0);

        let (provider, calls, _) = FixedAssertion::ok("fake-assertion");
        let config = Config::builder()
            .app_id("cli_demo")
            .app_type(AppType::Marketplace)
            .base_url(server.uri())
            .oauth_base_url(server.uri())
            .allow_custom_base_url(true)
            .client_assertion_provider(provider)
            .build();
        let token_provider = AuthTokenProvider::new(config);
        let err = token_provider
            .get_token(TokenRequest::tenant())
            .await
            .expect_err("marketplace 应被拒绝");
        assert_configuration(&err, CLIENT_ASSERTION_MARKETPLACE_UNSUPPORTED);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn user_and_none_messages_stay_unchanged_with_provider() {
        let (provider, calls, _) = FixedAssertion::ok("fake-assertion");
        let token_provider = AuthTokenProvider::new(
            Config::builder()
                .app_id("cli_demo")
                .client_assertion_provider(provider)
                .build(),
        );
        let user_err = token_provider
            .get_token(TokenRequest::user())
            .await
            .expect_err("user");
        // #693：User 路径改为缓存+刷新；无种子 refresh token 时仍报配置错误
        assert!(user_err.to_string().contains("user_refresh_token"));
        let none_err = token_provider
            .get_token(TokenRequest {
                token_type: AccessTokenType::None,
                ..TokenRequest::default()
            })
            .await
            .expect_err("none");
        assert!(
            none_err
                .to_string()
                .contains("AccessTokenType::None 不应触发 token 获取")
        );
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn self_build_without_provider_keeps_internal_grants() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/tenant_access_token/internal"))
            .and(body_json(
                json!({"app_id": "cli_demo", "app_secret": "secret_demo"}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "tenant_access_token": "tat_internal",
                "expire": 7200
            })))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/open-apis/auth/v3/app_access_token/internal"))
            .and(body_json(
                json!({"app_id": "cli_demo", "app_secret": "secret_demo"}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "app_access_token": "aat_internal",
                "expire": 7200
            })))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/oauth/v3/token"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("cli_demo")
            .app_secret("secret_demo")
            .base_url(server.uri())
            .allow_custom_base_url(true)
            .build();
        let token_provider = AuthTokenProvider::new(config);
        assert_eq!(
            token_provider
                .get_token(TokenRequest::tenant())
                .await
                .expect("internal tenant"),
            "tat_internal"
        );
        assert_eq!(
            token_provider
                .get_token(TokenRequest::app())
                .await
                .expect("internal app"),
            "aat_internal"
        );
    }

    #[test]
    fn resolve_maps_only_official_hosts_and_explicit_origin() {
        client_assertion::ensure_client_assertion_endpoint(FEISHU_BASE_URL, None)
            .expect("ensure 包装应复用同一解析");
        let feishu = resolve_oauth_endpoint(FEISHU_BASE_URL, None).expect("feishu");
        assert_eq!(
            feishu.token_url,
            "https://accounts.feishu.cn/oauth/v3/token"
        );
        assert_eq!(feishu.aud, "accounts.feishu.cn");

        let lark = resolve_oauth_endpoint(&format!("{LARK_BASE_URL}/"), None).expect("lark");
        assert_eq!(
            lark.token_url,
            "https://accounts.larksuite.com/oauth/v3/token"
        );
        assert_eq!(lark.aud, "accounts.larksuite.com");

        let larkoffice = resolve_oauth_endpoint("https://open.larkoffice.com", None)
            .expect_err("larkoffice 不猜测");
        assert_configuration(&larkoffice, CLIENT_ASSERTION_CUSTOM_BASE_URL);

        let overridden =
            resolve_oauth_endpoint(FEISHU_BASE_URL, Some("https://accounts.example.test:8443"))
                .expect("显式原点覆盖官方主机");
        assert_eq!(overridden.aud, "accounts.example.test:8443");
        assert_eq!(
            overridden.token_url,
            "https://accounts.example.test:8443/oauth/v3/token"
        );

        let blank = resolve_oauth_endpoint(FEISHU_BASE_URL, Some("  ")).expect("空白等于未设置");
        assert_eq!(blank.aud, "accounts.feishu.cn");

        for invalid in [
            "https://user:pass@accounts.example.test",
            "https://accounts.example.test/oauth",
            "https://accounts.example.test/?q=1",
            "https://accounts.example.test/#frag",
            "ftp://accounts.example.test",
        ] {
            let err = resolve_oauth_endpoint(FEISHU_BASE_URL, Some(invalid)).expect_err(invalid);
            assert_configuration(&err, CLIENT_ASSERTION_OAUTH_BASE_INVALID);
        }
    }

    #[test]
    fn assertion_cache_key_does_not_collide_with_grant_keys() {
        let assertion_key = assertion_cache_key("accounts.feishu.cn");
        assert_eq!(assertion_key, "client_assertion:accounts.feishu.cn");
        let requests = [
            TokenRequest::tenant(),
            TokenRequest::tenant().tenant_key("tenant"),
            TokenRequest::tenant().app_ticket("ticket"),
            TokenRequest::tenant().tenant_key("").app_ticket(""),
            TokenRequest::app(),
            TokenRequest::app().app_ticket("ticket"),
            TokenRequest::app()
                .tenant_key("tenant")
                .app_ticket("ticket"),
        ];
        for app_type in [AppType::SelfBuild, AppType::Marketplace] {
            for token_type in [
                AccessTokenType::Tenant,
                AccessTokenType::App,
                AccessTokenType::User,
                AccessTokenType::None,
            ] {
                for request in &requests {
                    let grant_key = AuthTokenProvider::cache_key(&token_type, &app_type, request);
                    assert_ne!(assertion_key, grant_key);
                }
            }
        }
    }

    fn limited_config(
        uri: &str,
        provider: FixedAssertion,
        timeout: Option<Duration>,
        max_response_size: u64,
    ) -> Config {
        let mut builder = Config::builder()
            .app_id("cli_demo")
            .app_type(AppType::SelfBuild)
            .base_url(uri)
            .oauth_base_url(uri)
            .allow_custom_base_url(true)
            .max_response_size(max_response_size)
            .client_assertion_provider(provider);
        if let Some(timeout) = timeout {
            builder = builder.req_timeout(timeout);
        }
        builder.build()
    }

    #[tokio::test]
    async fn exchange_applies_req_timeout_and_still_returns_token() {
        let server = MockServer::start().await;
        mount_absent_grants(&server).await;
        Mock::given(method("POST"))
            .and(path("/oauth/v3/token"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_delay(Duration::from_secs(3))
                    .set_body_json(json!({"access_token": "tat_late", "expires_in": 7200})),
            )
            .mount(&server)
            .await;

        let (provider, _, _) = FixedAssertion::ok("fake-assertion");
        let started = Instant::now();
        let err = AuthTokenProvider::new(limited_config(
            &server.uri(),
            provider,
            Some(Duration::from_millis(200)),
            1024,
        ))
        .get_token(TokenRequest::tenant())
        .await
        .expect_err("短于响应延迟的 req_timeout 应失败");
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "换票不应等到延迟结束，实际 {:?}",
            started.elapsed()
        );
        let message = error_chain(&err).to_ascii_lowercase();
        assert!(
            matches!(err, CoreError::Network(_))
                && (message.contains("timed out") || message.contains("timeout")),
            "应是请求超时，实际 {message}"
        );

        let fast = MockServer::start().await;
        mount_absent_grants(&fast).await;
        mount_oauth(
            &fast,
            200,
            json!({"access_token": "tat_fake", "expires_in": 7200}),
            1,
        )
        .await;
        let (provider, _, _) = FixedAssertion::ok("fake-assertion");
        let token = AuthTokenProvider::new(limited_config(
            &fast.uri(),
            provider,
            Some(Duration::from_secs(2)),
            1024,
        ))
        .get_token(TokenRequest::tenant())
        .await
        .expect("未超时且 body 未超限时应换到 token");
        assert_eq!(token, "tat_fake");
    }

    #[tokio::test]
    async fn exchange_rejects_body_over_max_response_size() {
        let server = MockServer::start().await;
        mount_absent_grants(&server).await;
        let payload = "x".repeat(256);
        Mock::given(method("POST"))
            .and(path("/oauth/v3/token"))
            .respond_with(ResponseTemplate::new(200).set_body_string(&payload))
            .expect(1)
            .mount(&server)
            .await;

        let (provider, _, _) = FixedAssertion::ok("fake-assertion");
        let err = AuthTokenProvider::new(limited_config(
            &server.uri(),
            provider,
            Some(Duration::from_secs(2)),
            32,
        ))
        .get_token(TokenRequest::tenant())
        .await
        .expect_err("超过 max_response_size 的换票响应应被拒绝");
        assert!(
            matches!(
                err,
                CoreError::ResponseTooLarge {
                    limit: 32,
                    actual: 256,
                    ..
                }
            ),
            "应按 Content-Length 提前拒绝，实际 {err:?}"
        );
    }

    #[tokio::test]
    async fn exchange_stops_chunked_body_before_reading_the_rest() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind");
        let origin = format!("http://{}", listener.local_addr().expect("addr"));
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept");
            let mut received = Vec::new();
            let mut buf = [0_u8; 2048];
            loop {
                let n = tokio::io::AsyncReadExt::read(&mut socket, &mut buf)
                    .await
                    .expect("read headers");
                if n == 0 {
                    return;
                }
                received.extend_from_slice(&buf[..n]);
                if received.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }
            let chunk = "x".repeat(128);
            let head = format!(
                "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n80\r\n{chunk}\r\n"
            );
            tokio::io::AsyncWriteExt::write_all(&mut socket, head.as_bytes())
                .await
                .expect("write chunk");
            tokio::io::AsyncWriteExt::flush(&mut socket)
                .await
                .expect("flush");
            tokio::time::sleep(Duration::from_secs(30)).await;
        });

        let (provider, _, _) = FixedAssertion::ok("fake-assertion");
        let token_provider = AuthTokenProvider::new(limited_config(
            &origin,
            provider,
            Some(Duration::from_secs(10)),
            32,
        ));
        let started = Instant::now();
        let result = tokio::time::timeout(
            Duration::from_secs(2),
            token_provider.get_token(TokenRequest::tenant()),
        )
        .await;
        server.abort();
        let err = result
            .expect("超限后应立刻返回，而不是把剩余 body 读完")
            .expect_err("分块超过上限应失败");
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "实际 {:?}",
            started.elapsed()
        );
        match err {
            CoreError::ResponseTooLarge { limit, actual, .. } => {
                assert_eq!(limit, 32);
                assert!(
                    actual > 32 && actual <= 128,
                    "应停在第一块，actual={actual}"
                );
            }
            other => panic!("应是 ResponseTooLarge，实际 {other:?}"),
        }
    }
}

// === #693：user access token 自动缓存与刷新（替身 HTTP，不访问真实飞书） ===

fn user_token_config(base_url: impl Into<String>, refresh_seed: Option<&str>) -> Config {
    let mut builder = Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url(base_url);
    if let Some(seed) = refresh_seed {
        builder = builder.user_refresh_token(seed);
    }
    builder.build()
}

fn app_token_internal_mock() -> Mock {
    Mock::given(method("POST"))
        .and(path("/open-apis/auth/v3/app_access_token/internal"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0, "msg": "success",
            "app_access_token": "app-token-for-refresh", "expire": 7200
        })))
}

fn oidc_refresh_mock(access: &str, new_refresh: &str, expires_in: u64) -> Mock {
    Mock::given(method("POST"))
        .and(path("/open-apis/authen/v1/oidc/refresh_access_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0, "msg": "ok",
            "data": {
                "access_token": access,
                "refresh_token": new_refresh,
                "expires_in": expires_in,
                "token_type": "Bearer"
            }
        })))
}

async fn refresh_request_bodies(server: &MockServer) -> Vec<String> {
    server
        .received_requests()
        .await
        .unwrap_or_default()
        .iter()
        .filter(|req| req.url.path().ends_with("oidc/refresh_access_token"))
        .filter_map(|req| {
            let body: serde_json::Value = serde_json::from_slice(&req.body).ok()?;
            body.get("refresh_token")?.as_str().map(str::to_string)
        })
        .collect()
}

/// AC1：配置 refresh token 后，User 类型可从缓存取 token；命中缓存不重复请求。
#[tokio::test]
async fn user_token_hits_cache_after_first_refresh() {
    let server = MockServer::start().await;
    server.register(app_token_internal_mock()).await;
    server
        .register(oidc_refresh_mock("ut-access-1", "rt-2", 7200))
        .await;

    let provider = AuthTokenProvider::new(user_token_config(server.uri(), Some("rt-1")));
    let first = provider
        .get_token(TokenRequest::user())
        .await
        .expect("首次应走刷新接口");
    assert_eq!(first, "ut-access-1");

    let second = provider
        .get_token(TokenRequest::user())
        .await
        .expect("第二次应命中缓存");
    assert_eq!(second, "ut-access-1");

    let bodies = refresh_request_bodies(&server).await;
    assert_eq!(bodies.len(), 1, "缓存命中后不应再刷新");
    assert_eq!(bodies[0], "rt-1", "首次刷新应使用配置的种子 refresh token");
}

/// AC2：进入提前刷新窗口（expires_in=60 → 有效期 0，立即过期）后自动刷新，
/// 且写回响应中的新 refresh token（轮转），后续继续命中缓存。
#[tokio::test]
async fn user_token_refreshes_with_rotation_in_refresh_window() {
    let server = MockServer::start().await;
    server.register(app_token_internal_mock()).await;
    server
        .register(oidc_refresh_mock("ut-access-1", "rt-2", 60))
        .await;

    let provider = AuthTokenProvider::new(user_token_config(server.uri(), Some("rt-1")));
    let first = provider
        .get_token(TokenRequest::user())
        .await
        .expect("首次刷新");
    assert_eq!(first, "ut-access-1");

    // 换桩：第二次刷新返回新 access + 再轮转的 refresh，有效期恢复正常
    let first_bodies = refresh_request_bodies(&server).await;
    server.reset().await;
    server.register(app_token_internal_mock()).await;
    server
        .register(oidc_refresh_mock("ut-access-2", "rt-3", 7200))
        .await;

    let second = provider
        .get_token(TokenRequest::user())
        .await
        .expect("过期后应自动刷新");
    assert_eq!(second, "ut-access-2", "应返回刷新后的新 token");
    assert_eq!(first_bodies[0], "rt-1");

    let bodies = refresh_request_bodies(&server).await;
    assert_eq!(bodies.len(), 1, "本阶段恰好一次刷新");
    assert_eq!(
        bodies[0], "rt-2",
        "刷新应使用上次响应写回的轮转 refresh token"
    );

    let third = provider
        .get_token(TokenRequest::user())
        .await
        .expect("刷新后应命中缓存");
    assert_eq!(third, "ut-access-2");
    assert_eq!(
        refresh_request_bodies(&server).await.len(),
        1,
        "缓存有效期内不应再刷新"
    );
}

/// AC4：无 refresh token 种子且缓存为空 → 配置错误（不访问网络）。
#[tokio::test]
async fn user_token_without_refresh_seed_is_configuration_error() {
    let provider = AuthTokenProvider::new(user_token_config("http://127.0.0.1:9", None));
    let err = provider
        .get_token(TokenRequest::user())
        .await
        .expect_err("缺 refresh token 应报配置错误");
    let msg = err.to_string();
    assert!(
        msg.contains("user_access_token") || msg.contains("refresh"),
        "错误应指引配置 refresh token 或显式传入 user_access_token，got: {msg}"
    );
}

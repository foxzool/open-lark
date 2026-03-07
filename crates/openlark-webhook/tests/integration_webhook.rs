use openlark_webhook::common::error::WebhookError;
use openlark_webhook::robot::v1::send::SendWebhookMessageRequest;
use serde_json::json;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn webhook_url(server: &MockServer) -> String {
    format!("{}/webhook", server.uri())
}

#[tokio::test]
async fn test_send_text_message_success_200() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .and(body_json(json!({
            "msg_type": "text",
            "content": {
                "text": "hello webhook"
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "ok"
        })))
        .mount(&server)
        .await;

    let resp = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("hello webhook".to_string())
        .execute()
        .await
        .expect("text message should be sent successfully");

    assert_eq!(resp.code, 0);
    assert_eq!(resp.msg, "ok");
}

#[tokio::test]
async fn test_send_all_message_types_request_format() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .and(body_json(json!({
            "msg_type": "text",
            "content": {
                "text": "type-text"
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0, "msg": "ok" })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .and(body_json(json!({
            "msg_type": "post",
            "content": {
                "post": "{\"zh_cn\":{\"title\":\"t\",\"content\":[[{\"tag\":\"text\",\"text\":\"body\"}]]}}"
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0, "msg": "ok" })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .and(body_json(json!({
            "msg_type": "image",
            "content": {
                "image_key": "img_v2_test_key"
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0, "msg": "ok" })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .and(body_json(json!({
            "msg_type": "file",
            "content": {
                "file_key": "file_v2_test_key"
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "code": 0, "msg": "ok" })))
        .mount(&server)
        .await;

    let url = webhook_url(&server);
    SendWebhookMessageRequest::new(url.clone())
        .text("type-text".to_string())
        .execute()
        .await
        .expect("text request body should match Feishu webhook format");

    SendWebhookMessageRequest::new(url.clone())
        .post("{\"zh_cn\":{\"title\":\"t\",\"content\":[[{\"tag\":\"text\",\"text\":\"body\"}]]}}".to_string())
        .execute()
        .await
        .expect("post request body should match Feishu webhook format");

    SendWebhookMessageRequest::new(url.clone())
        .image("img_v2_test_key".to_string())
        .execute()
        .await
        .expect("image request body should match Feishu webhook format");

    SendWebhookMessageRequest::new(url)
        .file("file_v2_test_key".to_string())
        .execute()
        .await
        .expect("file request body should match Feishu webhook format");
}

#[cfg(feature = "card")]
#[tokio::test]
async fn test_send_interactive_card_request_format() {
    let server = MockServer::start().await;

    let card = json!({
        "config": { "wide_screen_mode": true },
        "header": { "title": { "tag": "plain_text", "content": "Integration" } },
        "elements": [{ "tag": "div", "text": { "tag": "lark_md", "content": "card body" } }]
    });

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .and(body_json(json!({
            "msg_type": "interactive",
            "content": {
                "card": card
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "ok"
        })))
        .mount(&server)
        .await;

    let _ = SendWebhookMessageRequest::new(webhook_url(&server))
        .card(json!({
            "config": { "wide_screen_mode": true },
            "header": { "title": { "tag": "plain_text", "content": "Integration" } },
            "elements": [{ "tag": "div", "text": { "tag": "lark_md", "content": "card body" } }]
        }))
        .execute()
        .await
        .expect("interactive card request body should match Feishu webhook format");
}

#[tokio::test]
async fn test_error_response_400_returns_webhook_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(
            ResponseTemplate::new(400).set_body_json(json!({ "code": 19001, "msg": "invalid request" })),
        )
        .mount(&server)
        .await;

    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("bad request".to_string())
        .execute()
        .await
        .expect_err("400 response should return error");

    match err {
        WebhookError::Http(message) => assert!(message.contains("400")),
        _ => panic!("expected WebhookError::Http for 400"),
    }
}

#[tokio::test]
async fn test_rate_limit_429_returns_webhook_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(
            ResponseTemplate::new(429).set_body_json(json!({ "code": 19020, "msg": "rate limit" })),
        )
        .mount(&server)
        .await;

    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("rate limit".to_string())
        .execute()
        .await
        .expect_err("429 response should return rate limit error");

    match err {
        WebhookError::Http(message) => assert!(message.contains("429")),
        _ => panic!("expected WebhookError::Http for 429"),
    }
}

#[tokio::test]
async fn test_error_response_500_returns_webhook_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(
            ResponseTemplate::new(500).set_body_json(json!({ "code": 20000, "msg": "internal error" })),
        )
        .mount(&server)
        .await;

    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("server error".to_string())
        .execute()
        .await
        .expect_err("500 response should return server error");

    match err {
        WebhookError::Http(message) => assert!(message.contains("500")),
        _ => panic!("expected WebhookError::Http for 500"),
    }
}

#[tokio::test]
async fn test_response_parsing_success() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 12345,
            "msg": "accepted"
        })))
        .mount(&server)
        .await;

    let resp = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("parse response".to_string())
        .execute()
        .await
        .expect("200 response should be parsed");

    assert_eq!(resp.code, 12345);
    assert_eq!(resp.msg, "accepted");
}

#[tokio::test]
async fn test_response_parsing_failure_returns_serialization_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .mount(&server)
        .await;

    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("parse fail".to_string())
        .execute()
        .await
        .expect_err("invalid response body should return serialization error");

    match err {
        WebhookError::Serialization(_) => {}
        _ => panic!("expected WebhookError::Serialization for invalid JSON"),
    }
}

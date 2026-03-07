use openlark_webhook::common::error::WebhookError;
use openlark_webhook::robot::v1::send::SendWebhookMessageRequest;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn webhook_url(server: &MockServer) -> String {
    format!("{}/webhook", server.uri())
}

async fn mock_success(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code": 0, "msg": "ok"})))
        .mount(server)
        .await;
}

async fn mock_error(server: &MockServer, status: u16, code: i32, msg: &str) {
    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(status).set_body_json(json!({"code": code, "msg": msg})))
        .mount(server)
        .await;
}

#[tokio::test]
async fn test_send_text_message_success() {
    let server = MockServer::start().await;
    mock_success(&server).await;
    
    let resp = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("hello webhook".to_string())
        .execute()
        .await
        .expect("text message should be sent successfully");
    
    assert_eq!(resp.code, 0);
    assert_eq!(resp.msg, "ok");
}

#[tokio::test]
async fn test_send_all_message_types() {
    let server = MockServer::start().await;
    mock_success(&server).await;
    let url = webhook_url(&server);
    
    SendWebhookMessageRequest::new(url.clone())
        .text("type-text".to_string())
        .execute()
        .await
        .expect("text should succeed");
    
    SendWebhookMessageRequest::new(url.clone())
        .post("{\"zh_cn\":{\"title\":\"t\",\"content\":[[{\"tag\":\"text\",\"text\":\"body\"}]]}}".to_string())
        .execute()
        .await
        .expect("post should succeed");
    
    SendWebhookMessageRequest::new(url.clone())
        .image("img_v2_test_key".to_string())
        .execute()
        .await
        .expect("image should succeed");
    
    SendWebhookMessageRequest::new(url)
        .file("file_v2_test_key".to_string())
        .execute()
        .await
        .expect("file should succeed");
}

#[cfg(feature = "card")]
#[tokio::test]
async fn test_send_interactive_card() {
    let server = MockServer::start().await;
    mock_success(&server).await;
    
    let _ = SendWebhookMessageRequest::new(webhook_url(&server))
        .card(json!({
            "config": { "wide_screen_mode": true },
            "header": { "title": { "tag": "plain_text", "content": "Integration" } },
            "elements": [{ "tag": "div", "text": { "tag": "lark_md", "content": "card body" } }]
        }))
        .execute()
        .await
        .expect("card should succeed");
}

#[tokio::test]
async fn test_error_400() {
    let server = MockServer::start().await;
    mock_error(&server, 400, 19001, "invalid request").await;
    
    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("test".to_string())
        .execute()
        .await
        .expect_err("should fail");
    
    match err {
        WebhookError::Http(message) => assert!(message.contains("400")),
        _ => panic!("expected Http error"),
    }
}

#[tokio::test]
async fn test_error_429() {
    let server = MockServer::start().await;
    mock_error(&server, 429, 19020, "rate limit").await;
    
    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("test".to_string())
        .execute()
        .await
        .expect_err("should fail");
    
    match err {
        WebhookError::Http(message) => assert!(message.contains("429")),
        _ => panic!("expected Http error"),
    }
}

#[tokio::test]
async fn test_error_500() {
    let server = MockServer::start().await;
    mock_error(&server, 500, 20000, "internal error").await;
    
    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("test".to_string())
        .execute()
        .await
        .expect_err("should fail");
    
    match err {
        WebhookError::Http(message) => assert!(message.contains("500")),
        _ => panic!("expected Http error"),
    }
}

#[tokio::test]
async fn test_response_parsing_success() {
    let server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code": 12345, "msg": "accepted"})))
        .mount(&server)
        .await;
    
    let resp = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("test".to_string())
        .execute()
        .await
        .expect("should succeed");
    
    assert_eq!(resp.code, 12345);
    assert_eq!(resp.msg, "accepted");
}

#[tokio::test]
async fn test_response_parsing_failure() {
    let server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/webhook"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .mount(&server)
        .await;
    
    let err = SendWebhookMessageRequest::new(webhook_url(&server))
        .text("test".to_string())
        .execute()
        .await
        .expect_err("should fail");
    
    match err {
        WebhookError::Serialization(_) => {}
        _ => panic!("expected Serialization error"),
    }
}

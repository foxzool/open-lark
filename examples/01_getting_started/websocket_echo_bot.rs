use std::sync::Arc;
use std::time::Duration;

use openlark_client::ws_client::{EventDispatcherHandler, LarkWsClient};
use openlark_communication::endpoints::IM_V1_MESSAGES;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let runtime_config = RuntimeConfig::from_env()?;
    println!("🚀 启动 WebSocket Echo Bot");
    println!("🌐 API Base URL: {}", runtime_config.base_url);

    let ws_config = openlark_client::Config::builder()
        .app_id(runtime_config.app_id.clone())
        .app_secret(runtime_config.app_secret.clone())
        .base_url(runtime_config.base_url.clone())
        .timeout(Duration::from_secs(runtime_config.timeout_secs))
        .build()
        .map_err(|e| format!("构建 WebSocket 配置失败: {e}"))?;

    let (payload_tx, payload_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    tokio::spawn(process_payload_loop(payload_rx, runtime_config.clone()));

    let event_handler = EventDispatcherHandler::builder()
        .payload_sender(payload_tx)
        .build();

    println!("🔌 正在建立飞书长连接...");
    LarkWsClient::open(Arc::new(ws_config), event_handler).await?;
    Ok(())
}

#[derive(Debug, Clone)]
struct RuntimeConfig {
    app_id: String,
    app_secret: String,
    base_url: String,
    timeout_secs: u64,
}

impl RuntimeConfig {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let app_id = std::env::var("OPENLARK_APP_ID")
            .map_err(|_| "未找到环境变量 OPENLARK_APP_ID")?;
        let app_secret = std::env::var("OPENLARK_APP_SECRET")
            .map_err(|_| "未找到环境变量 OPENLARK_APP_SECRET")?;
        let base_url = std::env::var("OPENLARK_BASE_URL")
            .unwrap_or_else(|_| "https://open.feishu.cn".to_string());
        let timeout_secs = std::env::var("OPENLARK_TIMEOUT")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(30);

        Ok(Self {
            app_id,
            app_secret,
            base_url,
            timeout_secs,
        })
    }
}

async fn process_payload_loop(
    mut payload_rx: mpsc::UnboundedReceiver<Vec<u8>>,
    runtime_config: RuntimeConfig,
) {
    while let Some(payload) = payload_rx.recv().await {
        if let Err(err) = handle_payload(&runtime_config, &payload).await {
            eprintln!("❌ 处理事件失败: {err}");
        }
    }
}

async fn handle_payload(
    runtime_config: &RuntimeConfig,
    payload: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let envelope: EventEnvelope = match serde_json::from_slice(payload) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("⚠️ 忽略无法解析的事件载荷: {err}");
            return Ok(());
        }
    };

    if envelope.header.event_type != "im.message.receive_v1" {
        return Ok(());
    }

    if envelope.event.message.message_type != "text" {
        println!("ℹ️ 跳过非文本消息: {}", envelope.event.message.message_type);
        return Ok(());
    }

    let text = extract_text(&envelope.event.message.content)?;
    if text.trim().is_empty() {
        println!("ℹ️ 跳过空文本消息");
        return Ok(());
    }

    let (receive_id, receive_id_type) = resolve_receive_target(&envelope.event)?;
    send_echo_message(runtime_config, &receive_id, receive_id_type, &text).await?;

    println!("✅ Echo 成功: receive_id_type={receive_id_type}, receive_id={receive_id}");
    Ok(())
}

fn extract_text(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content_json: TextContent = serde_json::from_str(content)
        .map_err(|e| format!("解析文本消息 content 失败: {e}"))?;

    Ok(content_json.text)
}

fn resolve_receive_target(event: &EventBody) -> Result<(String, &'static str), Box<dyn std::error::Error>> {
    if event.message.chat_type == "p2p" {
        let open_id = event.sender.sender_id.open_id.clone();
        if open_id.is_empty() {
            return Err("p2p 消息缺少 sender.open_id".into());
        }
        return Ok((open_id, "open_id"));
    }

    if let Some(chat) = &event.chat {
        if !chat.chat_id.is_empty() {
            return Ok((chat.chat_id.clone(), "chat_id"));
        }
    }

    if let Some(chat_id) = &event.message.chat_id {
        if !chat_id.is_empty() {
            return Ok((chat_id.clone(), "chat_id"));
        }
    }

    Err("群聊消息缺少 chat_id".into())
}

async fn send_echo_message(
    runtime_config: &RuntimeConfig,
    receive_id: &str,
    receive_id_type: &str,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let app_access_token = fetch_app_access_token(runtime_config).await?;

    let body = json!({
        "receive_id": receive_id,
        "msg_type": "text",
        "content": json!({ "text": text }).to_string()
    });

    let base_url = runtime_config.base_url.trim_end_matches('/');
    let url = format!("{base_url}{IM_V1_MESSAGES}");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(runtime_config.timeout_secs))
        .build()?;

    let response = client
        .post(url)
        .query(&[("receive_id_type", receive_id_type)])
        .header("Authorization", format!("Bearer {app_access_token}"))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let response_text = response.text().await.unwrap_or_default();
        return Err(format!("发送 Echo 失败: {status}, body={response_text}").into());
    }

    Ok(())
}

async fn fetch_app_access_token(
    runtime_config: &RuntimeConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let core_config = openlark_core::config::Config::builder()
        .app_id(runtime_config.app_id.clone())
        .app_secret(runtime_config.app_secret.clone())
        .base_url(runtime_config.base_url.clone())
        .req_timeout(Duration::from_secs(runtime_config.timeout_secs))
        .build();
    let auth_service = openlark_auth::AuthService::new(core_config);
    let token_response = auth_service
        .v3()
        .app_access_token_internal()
        .app_id(runtime_config.app_id.clone())
        .app_secret(runtime_config.app_secret.clone())
        .execute()
        .await
        .map_err(|e| format!("获取 app_access_token 失败: {e}"))?;

    Ok(token_response.data.app_access_token)
}

#[derive(Debug, Deserialize)]
struct EventEnvelope {
    header: EventHeader,
    event: EventBody,
}

#[derive(Debug, Deserialize)]
struct EventHeader {
    event_type: String,
}

#[derive(Debug, Deserialize)]
struct EventBody {
    sender: Sender,
    message: Message,
    #[serde(default)]
    chat: Option<Chat>,
}

#[derive(Debug, Deserialize)]
struct Sender {
    sender_id: SenderId,
}

#[derive(Debug, Deserialize)]
struct SenderId {
    open_id: String,
}

#[derive(Debug, Deserialize)]
struct Message {
    message_type: String,
    content: String,
    chat_type: String,
    #[serde(default)]
    chat_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Chat {
    chat_id: String,
}

#[derive(Debug, Deserialize)]
struct TextContent {
    text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text_success() {
        let text = extract_text(r#"{"text":"hello"}"#).expect("should parse text content");
        assert_eq!(text, "hello");
    }

    #[test]
    fn test_resolve_receive_target_for_p2p() {
        let event = EventBody {
            sender: Sender {
                sender_id: SenderId {
                    open_id: "ou_test_user".to_string(),
                },
            },
            message: Message {
                message_type: "text".to_string(),
                content: "{\"text\":\"hello\"}".to_string(),
                chat_type: "p2p".to_string(),
                chat_id: None,
            },
            chat: None,
        };

        let (receive_id, receive_id_type) = resolve_receive_target(&event).expect("should resolve p2p receive target");
        assert_eq!(receive_id, "ou_test_user");
        assert_eq!(receive_id_type, "open_id");
    }

    #[test]
    fn test_resolve_receive_target_for_group() {
        let event = EventBody {
            sender: Sender {
                sender_id: SenderId {
                    open_id: "ou_test_user".to_string(),
                },
            },
            message: Message {
                message_type: "text".to_string(),
                content: "{\"text\":\"hello\"}".to_string(),
                chat_type: "group".to_string(),
                chat_id: None,
            },
            chat: Some(Chat {
                chat_id: "oc_group_001".to_string(),
            }),
        };

        let (receive_id, receive_id_type) = resolve_receive_target(&event).expect("should resolve group receive target");
        assert_eq!(receive_id, "oc_group_001");
        assert_eq!(receive_id_type, "chat_id");
    }

    #[test]
    fn test_extract_text_invalid_json() {
        let result = extract_text("not-json");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_receive_target_group_fallback_message_chat_id() {
        let event = EventBody {
            sender: Sender {
                sender_id: SenderId {
                    open_id: "ou_test_user".to_string(),
                },
            },
            message: Message {
                message_type: "text".to_string(),
                content: "{\"text\":\"hello\"}".to_string(),
                chat_type: "group".to_string(),
                chat_id: Some("oc_group_from_message".to_string()),
            },
            chat: None,
        };

        let (receive_id, receive_id_type) =
            resolve_receive_target(&event).expect("should resolve from message.chat_id");
        assert_eq!(receive_id, "oc_group_from_message");
        assert_eq!(receive_id_type, "chat_id");
    }

    #[test]
    fn test_resolve_receive_target_p2p_missing_open_id() {
        let event = EventBody {
            sender: Sender {
                sender_id: SenderId {
                    open_id: String::new(),
                },
            },
            message: Message {
                message_type: "text".to_string(),
                content: "{\"text\":\"hello\"}".to_string(),
                chat_type: "p2p".to_string(),
                chat_id: None,
            },
            chat: None,
        };

        let result = resolve_receive_target(&event);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_receive_target_group_missing_chat_id() {
        let event = EventBody {
            sender: Sender {
                sender_id: SenderId {
                    open_id: "ou_test_user".to_string(),
                },
            },
            message: Message {
                message_type: "text".to_string(),
                content: "{\"text\":\"hello\"}".to_string(),
                chat_type: "group".to_string(),
                chat_id: None,
            },
            chat: None,
        };

        let result = resolve_receive_target(&event);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_payload_invalid_json_returns_ok() {
        let runtime_config = RuntimeConfig {
            app_id: "cli_test".to_string(),
            app_secret: "secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            timeout_secs: 3,
        };

        let result = handle_payload(&runtime_config, b"invalid-json").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_payload_non_message_event_returns_ok() {
        let runtime_config = RuntimeConfig {
            app_id: "cli_test".to_string(),
            app_secret: "secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            timeout_secs: 3,
        };

        let payload = serde_json::to_vec(&json!({
            "header": {"event_type": "im.chat.member.added_v1"},
            "event": {
                "sender": {"sender_id": {"open_id": "ou_test"}},
                "message": {
                    "message_type": "text",
                    "content": "{\"text\":\"hi\"}",
                    "chat_type": "p2p"
                }
            }
        }))
        .expect("serialize payload");

        let result = handle_payload(&runtime_config, &payload).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_payload_non_text_event_returns_ok() {
        let runtime_config = RuntimeConfig {
            app_id: "cli_test".to_string(),
            app_secret: "secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            timeout_secs: 3,
        };

        let payload = serde_json::to_vec(&json!({
            "header": {"event_type": "im.message.receive_v1"},
            "event": {
                "sender": {"sender_id": {"open_id": "ou_test"}},
                "message": {
                    "message_type": "image",
                    "content": "{}",
                    "chat_type": "p2p"
                }
            }
        }))
        .expect("serialize payload");

        let result = handle_payload(&runtime_config, &payload).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_payload_empty_text_returns_ok() {
        let runtime_config = RuntimeConfig {
            app_id: "cli_test".to_string(),
            app_secret: "secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            timeout_secs: 3,
        };

        let payload = serde_json::to_vec(&json!({
            "header": {"event_type": "im.message.receive_v1"},
            "event": {
                "sender": {"sender_id": {"open_id": "ou_test"}},
                "message": {
                    "message_type": "text",
                    "content": "{\"text\":\"   \"}",
                    "chat_type": "p2p"
                }
            }
        }))
        .expect("serialize payload");

        let result = handle_payload(&runtime_config, &payload).await;
        assert!(result.is_ok());
    }
}

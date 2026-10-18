use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use openlark_core::SDKResult;
use openlark_core::auth::{TokenProvider, TokenRequest};
use openlark_core::config::Config;
use serde_json::{Value, json};
use tokio::sync::oneshot;

use super::{
    CardDocument, ChannelError, ChatKind, DmMode, GroupAllowlist, InboundContent, InboundPolicy,
    LarkChannel, MessageId, MessageRecipient, Outbound, RejectReason, SentMessage,
};
use crate::ws_client::{
    CardAction, CardActionTrigger, EventHeader, ImMention, ImMessageReceiveV1,
    ImMessageReceiveV1Event, ImMessageSender, ImReceivedMessage, ImSenderId, WsClientError,
};

type BoxFut<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

const BUTTON_FIXTURE: &str = r#"{
  "schema":"2.0",
  "header":{"event_id":"f7984f25108f8137722bb63c1d00bd823c2","event_type":"card.action.trigger","create_time":"1603977298000000","token":"tok","app_id":"cli_a511af62e2b5d07f","tenant_key":"736588c9260f175d"},
  "event":{"operator":{"tenant_key":"736588c9260f175d","user_id":"on_8f6f0d15799e5c45","open_id":"ou_4063d88c980c9f2d"},"token":"c-295eed59e6dbb014b72cba6f2ff6d48da9971e99","action":{"value":{"key":"value"},"tag":"button","timezone":"8","name":"btn","form_value":{},"input_value":"","option":"","options":[],"checked":false},"host":"im_message","context":{"open_message_id":"om_dc0d7ab6d7b734d5bff5c0556e8e7616","open_chat_id":"oc_4d83f1dc8596c773a09e86f50a931b77"}}
}"#;

fn config() -> Arc<Config> {
    Arc::new(
        Config::builder()
            .app_id("cli_demo")
            .app_secret("secret_demo")
            .build(),
    )
}

struct FakeState {
    patches: Mutex<Vec<Value>>,
    entered: AtomicUsize,
    release: Mutex<Option<oneshot::Receiver<()>>>,
    sends: AtomicUsize,
}

struct FakeOutbound {
    state: Arc<FakeState>,
}

impl FakeOutbound {
    fn recording() -> Arc<Self> {
        Arc::new(Self {
            state: Arc::new(FakeState {
                patches: Mutex::new(Vec::new()),
                entered: AtomicUsize::new(0),
                release: Mutex::new(None),
                sends: AtomicUsize::new(0),
            }),
        })
    }

    fn blocking_first_patch() -> (Arc<Self>, oneshot::Sender<()>) {
        let (tx, rx) = oneshot::channel();
        let fake = Arc::new(Self {
            state: Arc::new(FakeState {
                patches: Mutex::new(Vec::new()),
                entered: AtomicUsize::new(0),
                release: Mutex::new(Some(rx)),
                sends: AtomicUsize::new(0),
            }),
        });
        (fake, tx)
    }

    fn patches(&self) -> Vec<Value> {
        self.state.patches.lock().expect("patches").clone()
    }
}

impl Outbound for FakeOutbound {
    fn bot_open_id(&self) -> BoxFut<'_, Result<String, ChannelError>> {
        Box::pin(async { Ok("ou_bot".to_string()) })
    }

    fn send_text(
        &self,
        _to: MessageRecipient,
        _text: String,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            state.sends.fetch_add(1, Ordering::SeqCst);
            Ok(SentMessage {
                message_id: MessageId::new("om_sent").expect("id"),
            })
        })
    }

    fn send_card(
        &self,
        _to: MessageRecipient,
        _card: CardDocument,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>> {
        Box::pin(async {
            Ok(SentMessage {
                message_id: MessageId::new("om_card").expect("id"),
            })
        })
    }

    fn reply_text(
        &self,
        _message_id: String,
        _text: String,
        _in_thread: bool,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>> {
        Box::pin(async {
            Ok(SentMessage {
                message_id: MessageId::new("om_reply").expect("id"),
            })
        })
    }

    fn reply_card(
        &self,
        _message_id: String,
        _card: CardDocument,
        _in_thread: bool,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>> {
        Box::pin(async {
            Ok(SentMessage {
                message_id: MessageId::new("om_stream").expect("id"),
            })
        })
    }

    fn patch_card(&self, _message_id: String, body: Value) -> BoxFut<'_, Result<(), ChannelError>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let n = state.entered.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                let release = state.release.lock().expect("release").take();
                if let Some(release) = release {
                    let _ = release.await;
                }
            }
            state.patches.lock().expect("patches").push(body);
            Ok(())
        })
    }
}

fn text_message(
    message_id: &str,
    chat_type: &str,
    chat_id: &str,
    mentions: Vec<ImMention>,
) -> ImMessageReceiveV1 {
    text_with(message_id, chat_type, chat_id, r#"{"text":"hi"}"#, mentions)
}

fn text_with(
    message_id: &str,
    chat_type: &str,
    chat_id: &str,
    content: &str,
    mentions: Vec<ImMention>,
) -> ImMessageReceiveV1 {
    ImMessageReceiveV1 {
        schema: "2.0".to_string(),
        header: EventHeader {
            event_id: format!("evt-{message_id}"),
            event_type: "im.message.receive_v1".to_string(),
            create_time: "1700000000000".to_string(),
            token: String::new(),
            app_id: "cli_demo".to_string(),
            tenant_key: String::new(),
        },
        event: ImMessageReceiveV1Event {
            sender: Some(ImMessageSender {
                sender_id: Some(ImSenderId {
                    open_id: "ou_user".to_string(),
                    user_id: "user".to_string(),
                    union_id: "on_user".to_string(),
                }),
                sender_type: "user".to_string(),
                tenant_key: String::new(),
            }),
            message: ImReceivedMessage {
                message_id: message_id.to_string(),
                root_id: "om_root".to_string(),
                parent_id: "om_parent".to_string(),
                create_time: "1700000000001".to_string(),
                update_time: String::new(),
                chat_id: chat_id.to_string(),
                thread_id: "omt_thread".to_string(),
                chat_type: chat_type.to_string(),
                message_type: "text".to_string(),
                content: content.to_string(),
                mentions,
            },
        },
    }
}

fn mention(key: &str, open_id: &str) -> ImMention {
    ImMention {
        key: key.to_string(),
        id: Some(ImSenderId {
            open_id: open_id.to_string(),
            ..ImSenderId::default()
        }),
        name: String::new(),
        mentioned_type: String::new(),
        tenant_key: String::new(),
    }
}

fn channel_with(fake: Arc<FakeOutbound>) -> LarkChannel {
    let outbound: Arc<dyn Outbound> = fake;
    LarkChannel::builder(config())
        .with_outbound(outbound)
        .on_message(|_message, _channel| async { Ok(()) })
        .build()
        .expect("channel")
}

#[test]
fn channel_normalizes_text() {
    let event = text_message("om_1", "p2p", "user-dm", Vec::new());
    let message = super::gate::normalize_message(&event, "ou_bot").expect("normalized");
    assert!(matches!(message.content, InboundContent::Text(ref text) if text == "hi"));
    assert_eq!(
        message.root_id.as_ref().map(MessageId::as_str),
        Some("om_root")
    );
    assert_eq!(
        message.parent_id.as_ref().map(MessageId::as_str),
        Some("om_parent")
    );
    assert_eq!(
        message.thread_id.as_ref().map(MessageId::as_str),
        Some("omt_thread")
    );
    assert!(matches!(message.chat_kind, ChatKind::P2p));
    assert_eq!(message.chat_id, "user-dm");
}

#[test]
fn channel_mention_all_uses_parsed_tokens() {
    let policy = InboundPolicy::default();
    let cases = [
        (
            r#"{"text":"user@all.example"}"#,
            vec![mention("@_user_1", "ou_bot")],
            false,
            Ok(()),
        ),
        (
            r#"{"text":"@allergy"}"#,
            vec![mention("@_user_1", "ou_bot")],
            false,
            Ok(()),
        ),
        (
            r#"{"text":"hi @_all"}"#,
            vec![mention("@_user_1", "ou_bot")],
            false,
            Ok(()),
        ),
        (
            r#"{"text":"hi"}"#,
            vec![mention("@_all", ""), mention("@_user_1", "ou_bot")],
            true,
            Err(RejectReason::MentionAllBlocked),
        ),
        (
            r#"{"text":"hi"}"#,
            vec![mention("@all", "")],
            true,
            Err(RejectReason::MentionAllBlocked),
        ),
        (
            r#"{"text":"@allergy"}"#,
            vec![mention("@allergy", "ou_other")],
            false,
            Err(RejectReason::NoMention),
        ),
    ];

    for (index, (content, mentions, mention_all, decision)) in cases.into_iter().enumerate() {
        let event = text_with(
            &format!("om_mention_{index}"),
            "group",
            "oc_room",
            content,
            mentions,
        );
        let message = super::gate::normalize_message(&event, "ou_bot").expect("normalized");
        assert_eq!(message.mention_all, mention_all, "{content}");
        assert_eq!(
            super::gate::decide(&policy, &message),
            decision,
            "{content}"
        );
    }
}

#[test]
fn channel_connect_outside_runtime_keeps_messages() {
    let seen = Arc::new(AtomicUsize::new(0));
    let seen_handler = Arc::clone(&seen);
    let fake = FakeOutbound::recording();
    let outbound: Arc<dyn Outbound> = fake;
    let channel = LarkChannel::builder(config())
        .with_outbound(outbound)
        .on_message(move |_message, _channel| {
            seen_handler.fetch_add(1, Ordering::SeqCst);
            async { Ok(()) }
        })
        .build()
        .expect("channel");
    let _ = channel.inner.bot.set("ou_bot".to_string());

    {
        let mut connect = std::pin::pin!(channel.connect());
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        let polled = connect.as_mut().poll(&mut context);
        assert!(matches!(
            polled,
            std::task::Poll::Ready(Err(ChannelError::NoRuntime))
        ));
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    runtime.block_on(async {
        channel.ensure_workers().expect("retry inside runtime");
        channel
            .on_im(text_message("om_retry", "p2p", "user-dm", Vec::new()))
            .expect("dispatch");
        wait_until(|| seen.load(Ordering::SeqCst) == 1).await;
    });
    assert_eq!(seen.load(Ordering::SeqCst), 1);
}

#[tokio::test(flavor = "current_thread")]
async fn channel_policy_drops() {
    assert!(matches!(
        GroupAllowlist::only(Vec::<String>::new()),
        Err(ChannelError::EmptyAllowlist)
    ));
    assert!(matches!(
        DmMode::allowlist(Vec::<&str>::new()),
        Err(ChannelError::EmptyAllowlist)
    ));

    let messages = Arc::new(AtomicUsize::new(0));
    let reasons = Arc::new(Mutex::new(Vec::new()));
    let messages_handler = Arc::clone(&messages);
    let reasons_handler = Arc::clone(&reasons);
    let fake = FakeOutbound::recording();
    let outbound: Arc<dyn Outbound> = fake;
    let channel = LarkChannel::builder(config())
        .with_outbound(outbound)
        .on_message(move |_message, _channel| {
            messages_handler.fetch_add(1, Ordering::SeqCst);
            async { Ok(()) }
        })
        .on_reject(move |_message, reason| {
            reasons_handler.lock().expect("reasons").push(reason);
            async { Ok(()) }
        })
        .build()
        .expect("channel");
    let _ = channel.inner.bot.set("ou_bot".to_string());
    channel.ensure_workers().expect("workers");

    channel
        .on_im(text_message("om_group", "group", "oc_room", Vec::new()))
        .expect("dispatch");
    channel
        .on_im(text_message(
            "om_all",
            "group",
            "oc_room",
            vec![mention("@_all", ""), mention("@_user_1", "ou_bot")],
        ))
        .expect("dispatch");
    channel
        .on_im(text_message("om_dm", "p2p", "user-dm", Vec::new()))
        .expect("dispatch");
    wait_until(|| {
        messages.load(Ordering::SeqCst) == 1 && reasons.lock().expect("reasons").len() == 2
    })
    .await;

    assert_eq!(messages.load(Ordering::SeqCst), 1);
    assert_eq!(
        reasons.lock().expect("reasons").as_slice(),
        &[RejectReason::NoMention, RejectReason::MentionAllBlocked]
    );

    channel
        .on_im(text_message("om_dm", "p2p", "user-dm", Vec::new()))
        .expect("dispatch");
    tokio::task::yield_now().await;
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert_eq!(messages.load(Ordering::SeqCst), 1);
    assert_eq!(reasons.lock().expect("reasons").len(), 2);
}

#[tokio::test(flavor = "current_thread")]
async fn channel_patch_queue() {
    let (fake, release) = FakeOutbound::blocking_first_patch();
    let entered = Arc::clone(&fake.state);
    let channel = channel_with(Arc::clone(&fake));
    let stream = Arc::new(
        channel
            .reply_to(&MessageId::new("om_target").expect("id"))
            .card_stream(CardDocument::from_value(json!({"schema": "2.0", "n": 0})).expect("card"))
            .await
            .expect("stream"),
    );
    let card_a = CardDocument::from_value(json!({"schema": "2.0", "n": 1})).expect("card");
    let card_b = CardDocument::from_value(json!({"schema": "2.0", "n": 2})).expect("card");

    let stream_a = Arc::clone(&stream);
    let first = tokio::spawn(async move { stream_a.update(card_a).await });
    wait_until(|| entered.entered.load(Ordering::SeqCst) == 1).await;
    let stream_b = Arc::clone(&stream);
    let second = tokio::spawn(async move { stream_b.update(card_b).await });
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }
    assert_eq!(entered.entered.load(Ordering::SeqCst), 1);

    release.send(()).expect("release");
    first.await.expect("join").expect("update 1");
    second.await.expect("join").expect("update 2");

    let patches = fake.patches();
    assert_eq!(patches.len(), 2);
    assert_card_body(&patches[0], &json!({"schema": "2.0", "n": 1}));
    assert_card_body(&patches[1], &json!({"schema": "2.0", "n": 2}));

    stream.close().await.expect("close");
    stream.close().await.expect("close again");
    assert_eq!(fake.patches().len(), 2);
    let err = stream
        .update(CardDocument::from_value(json!({"schema": "2.0", "n": 3})).expect("card"))
        .await
        .expect_err("closed");
    assert!(matches!(err, ChannelError::StreamClosed));
    assert_eq!(fake.patches().len(), 2);
}

#[tokio::test(flavor = "current_thread")]
async fn channel_card_action_value() {
    let fake = FakeOutbound::recording();
    let sends = Arc::clone(&fake.state);
    let seen = Arc::new(Mutex::new(Vec::new()));
    let seen_handler = Arc::clone(&seen);
    let phase = Arc::new(AtomicUsize::new(0));
    let phase_handler = Arc::clone(&phase);
    let fake_for_send = Arc::clone(&fake);
    let outbound: Arc<dyn Outbound> = fake_for_send;
    let channel = LarkChannel::builder(config())
        .with_outbound(outbound)
        .on_card_action(move |ch, action| {
            let button = action
                .value
                .as_ref()
                .and_then(|value| value.get("key"))
                .and_then(Value::as_str)
                == Some("value");
            seen_handler.lock().expect("seen").push(action);
            let mut ack = super::CardAck::toast("已收到");
            if button {
                let phase = Arc::clone(&phase_handler);
                ack = ack.after(async move {
                    assert!(phase.load(Ordering::SeqCst) >= 2);
                    let _ = ch
                        .send(MessageRecipient::chat_id("oc_room"))
                        .text("after")
                        .await;
                    phase.store(3, Ordering::SeqCst);
                    Ok(())
                });
            }
            ack
        })
        .build()
        .expect("channel");
    channel.ensure_workers().expect("workers");

    let button: CardActionTrigger = serde_json::from_str(BUTTON_FIXTURE).expect("fixture");
    let response = channel
        .on_card(button.clone())
        .expect("button")
        .expect("toast");
    assert_eq!(phase.load(Ordering::SeqCst), 0);
    phase.store(2, Ordering::SeqCst);
    let toast = response.toast.expect("toast");
    assert_eq!(toast.r#type, "info");
    assert_eq!(toast.content.as_deref(), Some("已收到"));
    assert!(fake.patches().is_empty());

    tokio::task::yield_now().await;
    wait_until(|| phase.load(Ordering::SeqCst) == 3).await;
    assert_eq!(sends.sends.load(Ordering::SeqCst), 1);
    assert!(fake.patches().is_empty());

    let mut form = button.clone();
    form.header.event_id = "form-event".to_string();
    form.event.action = Some(CardAction {
        value: button
            .event
            .action
            .as_ref()
            .and_then(|action| action.value.clone()),
        tag: "button".to_string(),
        timezone: String::new(),
        name: "submit".to_string(),
        form_value: Some(json!({"reason": "迟到"})),
        input_value: String::new(),
        option: String::new(),
        options: Vec::new(),
        checked: false,
    });
    let form_response = channel.on_card(form).expect("form").expect("toast");
    assert!(form_response.toast.is_some());
    assert!(fake.patches().is_empty());

    let seen = seen.lock().expect("seen");
    assert_eq!(seen.len(), 2);
    assert_eq!(seen[0].value, Some(json!({"key": "value"})));
    assert_eq!(seen[0].form_value, Some(json!({})));
    assert_eq!(seen[1].value, Some(json!({"key": "value"})));
    assert_eq!(seen[1].form_value, Some(json!({"reason": "迟到"})));
}

#[tokio::test(flavor = "current_thread")]
async fn channel_reconnect_loop() {
    let outcomes = Mutex::new(VecDeque::from([
        Err(WsClientError::ConnectionClosed { reason: None }),
        Err(WsClientError::ConnectionClosed { reason: None }),
        Err(WsClientError::UnexpectedResponse),
    ]));
    let opens = AtomicUsize::new(0);
    let sleeps = Mutex::new(Vec::new());
    let stopped = AtomicBool::new(false);
    let result = super::reconnect_loop(
        || {
            opens.fetch_add(1, Ordering::SeqCst);
            let next = outcomes
                .lock()
                .expect("outcomes")
                .pop_front()
                .expect("open");
            std::future::ready(next)
        },
        |delay| {
            sleeps.lock().expect("sleeps").push(delay);
            std::future::ready(())
        },
        &stopped,
    )
    .await;
    assert!(matches!(
        result,
        Err(ChannelError::Socket(WsClientError::UnexpectedResponse))
    ));
    assert_eq!(opens.load(Ordering::SeqCst), 3);
    assert_eq!(
        sleeps.lock().expect("sleeps").as_slice(),
        &[Duration::from_secs(1), Duration::from_secs(2)]
    );

    let opens_after_stop = AtomicUsize::new(0);
    let mut slept = false;
    let result = super::reconnect_loop(
        || {
            opens_after_stop.fetch_add(1, Ordering::SeqCst);
            stopped.store(true, Ordering::Release);
            std::future::ready(Err(WsClientError::ConnectionClosed { reason: None }))
        },
        |_delay| {
            slept = true;
            std::future::ready(())
        },
        &stopped,
    )
    .await;
    assert!(result.is_ok());
    assert!(!slept);
    assert_eq!(opens_after_stop.load(Ordering::SeqCst), 1);
}

fn assert_card_body(body: &Value, card: &Value) {
    let content = body
        .get("content")
        .and_then(Value::as_str)
        .expect("content string");
    let parsed: Value = serde_json::from_str(content).expect("card json");
    assert_eq!(&parsed, card);
    assert!(body.get("content").is_some_and(Value::is_string));
}

#[tokio::test]
#[ignore = "需要 OPENLARK_CHANNEL_E2E=1 和真实群里的人工点击"]
async fn channel_card_callback_e2e() {
    let ready = std::env::var("OPENLARK_CHANNEL_E2E").ok().as_deref() == Some("1")
        && std::env::var("OPENLARK_APP_ID")
            .ok()
            .is_some_and(|value| !value.trim().is_empty())
        && std::env::var("OPENLARK_APP_SECRET")
            .ok()
            .is_some_and(|value| !value.trim().is_empty())
        && std::env::var("OPENLARK_CHANNEL_CHAT_ID")
            .ok()
            .is_some_and(|value| !value.trim().is_empty());
    if !ready {
        eprintln!(
            "跳过 channel 卡片回调端到端：未设置 OPENLARK_CHANNEL_E2E=1 或缺少 OPENLARK_APP_ID / OPENLARK_APP_SECRET / OPENLARK_CHANNEL_CHAT_ID"
        );
        return;
    }

    let app_id = std::env::var("OPENLARK_APP_ID").expect("app id");
    let app_secret = std::env::var("OPENLARK_APP_SECRET").expect("app secret");
    let chat_id = std::env::var("OPENLARK_CHANNEL_CHAT_ID").expect("chat id");
    let mut builder = Config::builder().app_id(app_id).app_secret(app_secret);
    if let Ok(base_url) = std::env::var("OPENLARK_BASE_URL")
        && !base_url.trim().is_empty()
    {
        builder = builder.base_url(base_url);
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos().to_string())
        .unwrap_or_else(|_| "nonce".to_string());

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let channel = LarkChannel::builder(Arc::new(builder.build()))
        .on_message(|_message, _channel| async { Ok(()) })
        .on_card_action(move |_channel, action| {
            let _ = tx.send(action);
            super::CardAck::toast("收到")
        })
        .build()
        .expect("channel");

    let connected = channel.clone();
    let mut connect_task = tokio::spawn(async move { connected.connect().await });
    let card = openlark_communication::MessageCard::new()
        .title("回调")
        .elements([
            openlark_communication::button("确认")
                .value(json!({"k": "button", "n": nonce}))
                .element(),
            openlark_communication::form(
                "ticket",
                [
                    openlark_communication::input("reason").element(),
                    openlark_communication::button("提交")
                        .name("submit")
                        .action_type(openlark_communication::FormAction::Submit)
                        .element(),
                ],
            ),
        ]);
    let wait_secs = e2e_wait_secs(
        std::env::var("OPENLARK_CHANNEL_E2E_TIMEOUT_SECS")
            .ok()
            .as_deref(),
    );
    channel
        .send(MessageRecipient::chat_id(&chat_id))
        .card(CardDocument::from_message_card(card).expect("card"))
        .await
        .expect("send card");
    eprintln!(
        "在该群点击「确认」；在 reason 里填写这个 nonce 并提交。nonce={nonce} chat_id={chat_id} 等待={wait_secs}秒（OPENLARK_CHANNEL_E2E_TIMEOUT_SECS）"
    );
    let deadline = tokio::time::Instant::now() + Duration::from_secs(wait_secs);
    let mut saw_button = false;
    let mut saw_form = false;
    let mut connect_done = false;
    while !(saw_button && saw_form) {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        if remaining.is_zero() {
            panic!("channel 卡片回调端到端超时");
        }
        tokio::select! {
            joined = &mut connect_task, if !connect_done => {
                connect_done = true;
                match joined {
                    Ok(Err(ChannelError::Socket(WsClientError::ConnectionClosed { .. }))) => {}
                    Ok(Err(err)) => panic!("连接失败: {err}"),
                    Err(err) => panic!("连接任务结束: {err}"),
                    Ok(Ok(())) => panic!("连接在卡片动作到齐前返回"),
                }
            }
            maybe = tokio::time::timeout(remaining, rx.recv()) => {
                let Some(action) = maybe.ok().flatten() else {
                    panic!("channel 卡片回调端到端超时");
                };
                let value_matches = action
                    .value
                    .as_ref()
                    .and_then(|value| value.get("k"))
                    .and_then(Value::as_str)
                    == Some("button")
                    && action
                        .value
                        .as_ref()
                        .and_then(|value| value.get("n"))
                        .and_then(Value::as_str)
                        == Some(nonce.as_str());
                if value_matches {
                    saw_button = true;
                }
                if action
                    .form_value
                    .as_ref()
                    .and_then(|value| value.get("reason"))
                    .and_then(Value::as_str)
                    == Some(nonce.as_str())
                {
                    saw_form = true;
                }
            }
        }
    }
    channel.shutdown();
    connect_task.abort();
}

fn e2e_wait_secs(raw: Option<&str>) -> u64 {
    raw.and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|secs| *secs > 0)
        .unwrap_or(180)
}

#[test]
fn channel_e2e_wait_secs_is_configurable() {
    assert_eq!(e2e_wait_secs(None), 180);
    assert_eq!(e2e_wait_secs(Some("")), 180);
    assert_eq!(e2e_wait_secs(Some("0")), 180);
    assert_eq!(e2e_wait_secs(Some("abc")), 180);
    assert_eq!(e2e_wait_secs(Some(" 45 ")), 45);
}

#[derive(Debug)]
struct MarkerProvider;

impl TokenProvider for MarkerProvider {
    fn get_token(
        &self,
        _request: TokenRequest,
    ) -> Pin<Box<dyn Future<Output = SDKResult<String>> + Send + '_>> {
        Box::pin(async { Ok("tenant-token-marker".to_string()) })
    }
}

#[tokio::test(flavor = "current_thread")]
async fn channel_installs_auth_token_provider() {
    let channel = LarkChannel::builder(config())
        .on_message(|_message, _channel| async { Ok(()) })
        .build()
        .expect("channel");
    let provider = channel.inner.config.token_provider();
    assert!(!provider.is_noop());
    let rendered = format!("{provider:?}");
    assert!(rendered.contains("AuthTokenProvider"), "{rendered}");

    let custom = Arc::new(config().with_token_provider(MarkerProvider));
    let channel = LarkChannel::builder(custom)
        .on_message(|_message, _channel| async { Ok(()) })
        .build()
        .expect("channel");
    let token = channel
        .inner
        .config
        .token_provider()
        .get_tenant_token(None)
        .await
        .expect("marker");
    assert_eq!(token, "tenant-token-marker");
}

async fn wait_until(mut pred: impl FnMut() -> bool) {
    for _ in 0..100 {
        if pred() {
            return;
        }
        tokio::task::yield_now().await;
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    panic!("条件未在时限内成立");
}

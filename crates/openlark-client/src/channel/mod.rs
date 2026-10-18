//! Channel 会话层。
//!
//! 在现有长连接和 IM helper 之上提供归一化、入站策略、回复和流式卡片更新。
//! 不改 `LarkWsClient::open` 的签名，也不在 `Session` 里重连。

mod gate;

#[cfg(test)]
mod tests;

use std::collections::{HashSet, VecDeque};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};

use openlark_bot::BotService;
use openlark_communication::im::v1::message::patch::PatchMessageCardRequest;
use openlark_communication::im::v1::message::reply::{ReplyMessageBody, ReplyMessageRequest};
use openlark_communication::{CommunicationClient, MessageCard, ReplyTarget};
use openlark_core::config::Config;
use openlark_core::error::CoreError;
use serde_json::Value;
use tokio::runtime::Handle;
use tokio::sync::{Notify, mpsc, oneshot};

use crate::ws_client::{
    CardActionCard, CardActionTrigger, CardActionTriggerResponse, CardToast,
    EventDispatcherHandler, ImMessageReceiveV1, LarkWsClient, WsClientError,
};

const MESSAGE_QUEUE: usize = 256;
const REJECT_QUEUE: usize = 64;
const PATCH_QUEUE: usize = 100;
const DEDUP_LIMIT: usize = 4096;
const BACKOFF_START: Duration = Duration::from_secs(1);
const BACKOFF_CAP: Duration = Duration::from_secs(30);
const SESSION_RESET: Duration = Duration::from_secs(60);

type BoxFut<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
type SharedFuture = Pin<Box<dyn Future<Output = Result<(), ChannelError>> + Send>>;
type MessageHandler = Arc<
    dyn Fn(
            InboundMessage,
            LarkChannel,
        ) -> Pin<Box<dyn Future<Output = Result<(), ChannelError>> + Send>>
        + Send
        + Sync,
>;
type RejectHandler = Arc<
    dyn Fn(
            InboundMessage,
            RejectReason,
        ) -> Pin<Box<dyn Future<Output = Result<(), ChannelError>> + Send>>
        + Send
        + Sync,
>;
type CardHandler = Arc<dyn Fn(LarkChannel, InboundCardAction) -> CardAck + Send + Sync>;

/// 会话错误。
#[derive(Debug, thiserror::Error)]
pub enum ChannelError {
    /// 已有一次 `connect` 还没返回。
    #[error("已有连接在进行")]
    AlreadyConnected,
    /// `on_message` 和 `on_card_action` 都没注册。
    #[error("未注册 on_message 或 on_card_action")]
    NoHandler,
    /// 当前线程没有 tokio 运行时，worker 无法启动。
    #[error("当前没有 tokio 运行时")]
    NoRuntime,
    /// 卡片流已经 `close`。
    #[error("卡片流已关闭")]
    StreamClosed,
    /// 创建或回复的响应里没有可用的 `message_id`。
    #[error("响应缺少 message_id")]
    MissingMessageId,
    /// 文本去掉空白后是空的。
    #[error("文本不能为空")]
    EmptyText,
    /// 卡片是 `null` 或空对象，或构建器拒绝了这张卡。
    #[error("卡片无效")]
    InvalidCard,
    /// 机器人 `open_id` 没取到，长连接不会打开。
    #[error("无法获取机器人身份")]
    BotIdentity,
    /// 白名单迭代器里没有非空 id。
    #[error("名单不能为空")]
    EmptyAllowlist,
    /// IM 或补丁请求失败。
    #[error("传输错误: {0}")]
    Transport(#[from] CoreError),
    /// 长连接返回了除正常断开以外的错误。
    #[error("长连接错误: {0}")]
    Socket(#[from] WsClientError),
}

/// 非空消息 id。全空白在边界被拒绝。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MessageId(String);

impl MessageId {
    /// 拒绝空串和全空白。
    pub fn new(id: impl Into<String>) -> Result<Self, ChannelError> {
        let id = id.into();
        if id.trim().is_empty() {
            Err(ChannelError::MissingMessageId)
        } else {
            Ok(Self(id))
        }
    }

    /// 原始 id 文本。
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MessageId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// 归一化后的入站消息。
#[derive(Debug, Clone, PartialEq)]
pub struct InboundMessage {
    /// 事件头里的 `event_id`。
    pub event_id: String,
    /// 消息 id。
    pub message_id: MessageId,
    /// 根消息 id。没有时为 `None`。
    pub root_id: Option<MessageId>,
    /// 父消息 id。没有时为 `None`。
    pub parent_id: Option<MessageId>,
    /// 话题 id。没有时为 `None`。不会自动变成话题回复。
    pub thread_id: Option<MessageId>,
    /// 会话 id，原样保留。
    pub chat_id: String,
    /// 由 `chat_type` 决定，不看 id 前缀。
    pub chat_kind: ChatKind,
    /// 发送者。
    pub sender: Sender,
    /// 正文。文本不再是 JSON 字符串。
    pub content: InboundContent,
    /// 消息里的 mention。
    pub mentions: Vec<Mention>,
    /// 某条 mention 的 key 是 `@_all` 或 `@all`。正文里的同形文字不算。
    pub mention_all: bool,
    /// 某个 mention 的 `open_id` 等于机器人。
    pub mentioned_bot: bool,
    /// `message.create_time`，解析失败时用事件头。不参与过期丢弃。
    pub create_time_ms: Option<i64>,
}

/// 入站正文。
#[derive(Debug, Clone, PartialEq)]
pub enum InboundContent {
    /// `message_type == "text"`，且 content 是 `{"text": ...}`。
    Text(String),
    /// 其它能解析成 JSON 的 content。
    Other {
        /// 原始 `message_type`。
        message_type: String,
        /// 解析后的 JSON。
        value: Value,
    },
}

/// 会话种类。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatKind {
    /// `chat_type` 为 `p2p`。
    P2p,
    /// `chat_type` 为 `group`。
    Group,
    /// 其它取值，原样保留。
    Other(String),
}

/// 发送者 id。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sender {
    /// `open_id`。
    pub open_id: String,
    /// `user_id`。
    pub user_id: String,
    /// `union_id`。
    pub union_id: String,
}

/// 一条 mention。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mention {
    /// mention key，例如 `@_user_1`。
    pub key: String,
    /// 被提及者的 `open_id`。
    pub open_id: String,
    /// 显示名。
    pub name: String,
}

/// 按钮和表单共用的卡片动作。
#[derive(Debug, Clone, PartialEq)]
pub struct InboundCardAction {
    /// 事件头里的 `event_id`。
    pub event_id: String,
    /// `context.open_message_id`。
    pub message_id: MessageId,
    /// `context.open_chat_id`。
    pub chat_id: String,
    /// 操作者 `open_id`。
    pub operator_open_id: String,
    /// 组件 tag。
    pub tag: String,
    /// 组件 name。
    pub name: String,
    /// 按钮回调值。
    pub value: Option<Value>,
    /// 表单值。空对象原样留下。
    pub form_value: Option<Value>,
    /// `event.token`。流式补丁不读它。
    pub token: String,
}

/// 入站策略。字段私有，用方法修改。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundPolicy {
    require_mention: bool,
    respond_to_mention_all: bool,
    groups: GroupAllowlist,
    dm: DmMode,
}

impl Default for InboundPolicy {
    fn default() -> Self {
        Self {
            require_mention: true,
            respond_to_mention_all: false,
            groups: GroupAllowlist::Unrestricted,
            dm: DmMode::Open,
        }
    }
}

impl InboundPolicy {
    /// 群消息是否必须 @ 机器人或 @所有人。单聊不看这个开关。
    pub fn require_mention(mut self, yes: bool) -> Self {
        self.require_mention = yes;
        self
    }

    /// 为真时，@所有人的群消息可以进 `on_message`。
    pub fn respond_to_mention_all(mut self, yes: bool) -> Self {
        self.respond_to_mention_all = yes;
        self
    }

    /// 替换群白名单。
    pub fn groups(mut self, groups: GroupAllowlist) -> Self {
        self.groups = groups;
        self
    }

    /// 替换单聊策略。
    pub fn dm(mut self, mode: DmMode) -> Self {
        self.dm = mode;
        self
    }
}

/// 群白名单。空名单不可构造。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupAllowlist {
    /// 不按群 id 限制。
    Unrestricted,
    /// 只接收这些群。
    Only(NonEmptyIds),
}

/// 单聊策略。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmMode {
    /// 单聊都接收。
    Open,
    /// 单聊都拒绝。
    Disabled,
    /// 只接收这些发送者的 `open_id`。
    Allowlist(NonEmptyIds),
}

/// 非空 id 列表。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyIds {
    ids: Vec<String>,
}

impl NonEmptyIds {
    /// 名单里是否有这个 id。比较的是完整字符串。
    pub fn contains(&self, id: &str) -> bool {
        self.ids.iter().any(|item| item == id)
    }
}

impl GroupAllowlist {
    /// 空迭代器，或滤掉空白后为空，返回 [`ChannelError::EmptyAllowlist`]。
    pub fn only(ids: impl IntoIterator<Item = impl Into<String>>) -> Result<Self, ChannelError> {
        Ok(Self::Only(non_empty_ids(ids)?))
    }

    /// 不限制群。
    pub fn unrestricted() -> Self {
        Self::Unrestricted
    }
}

impl DmMode {
    /// 空迭代器，或滤掉空白后为空，返回 [`ChannelError::EmptyAllowlist`]。
    pub fn allowlist(
        open_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, ChannelError> {
        Ok(Self::Allowlist(non_empty_ids(open_ids)?))
    }
}

/// 策略拒绝原因。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectReason {
    /// 群不在白名单里。
    GroupNotAllowed,
    /// 群消息没有 @ 机器人，也不是 @所有人。
    NoMention,
    /// @所有人，且策略不响应 @所有人。同时 @ 了机器人也拒绝。
    MentionAllBlocked,
    /// 单聊已关闭。
    DmDisabled,
    /// 单聊发送者不在白名单里。
    SenderNotAllowed,
    /// `chat_type` 不是 `p2p` 或 `group`。
    ChatKindUnsupported,
}

/// 发给 `send_card` 或补丁的卡片 JSON。
#[derive(Debug, Clone, PartialEq)]
pub struct CardDocument(Value);

impl CardDocument {
    /// 用 schema 2.0 构建器产出卡片。构建失败时返回 [`ChannelError::InvalidCard`]。
    pub fn from_message_card(card: MessageCard) -> Result<Self, ChannelError> {
        card.to_value()
            .map(Self)
            .map_err(|_| ChannelError::InvalidCard)
    }

    /// 拒绝 `null` 和 `{}`。不检查 schema。
    pub fn from_value(value: Value) -> Result<Self, ChannelError> {
        if value.is_null() || value.as_object().is_some_and(|object| object.is_empty()) {
            return Err(ChannelError::InvalidCard);
        }
        Ok(Self(value))
    }

    fn as_value(&self) -> &Value {
        &self.0
    }
}

/// 卡片回调的同步回写。HTTP 放在 [`CardAck::after`]。
pub struct CardAck {
    response: Option<CardActionTriggerResponse>,
    after: Option<SharedFuture>,
}

impl std::fmt::Debug for CardAck {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CardAck")
            .field("response", &self.response)
            .field("after", &self.after.is_some())
            .finish()
    }
}

impl CardAck {
    /// ACK 不带业务体。
    pub fn empty() -> Self {
        Self {
            response: None,
            after: None,
        }
    }

    /// toast，类型为 `info`。
    pub fn toast(content: impl Into<String>) -> Self {
        Self {
            response: Some(CardActionTriggerResponse {
                toast: Some(CardToast {
                    r#type: "info".to_string(),
                    title: None,
                    content: Some(content.into()),
                }),
                card: None,
            }),
            after: None,
        }
    }

    /// 立即替换卡片。这是 ACK 里的 raw 卡片，不是补丁请求。
    pub fn replace(card: CardDocument) -> Self {
        Self {
            response: Some(CardActionTriggerResponse {
                toast: None,
                card: Some(CardActionCard {
                    r#type: "raw".to_string(),
                    data: Some(card.0),
                }),
            }),
            after: None,
        }
    }

    /// 回调返回之后再跑的异步工作。改不了这次 ACK。
    pub fn after<F>(mut self, fut: F) -> Self
    where
        F: Future<Output = Result<(), ChannelError>> + Send + 'static,
    {
        self.after = Some(Box::pin(fut));
        self
    }

    fn split(self) -> (Option<CardActionTriggerResponse>, Option<SharedFuture>) {
        (self.response, self.after)
    }
}

/// 已发出的消息。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SentMessage {
    /// 服务端返回的消息 id。
    pub message_id: MessageId,
}

/// 按接收者发送。不要求已经 `connect`。
#[derive(Debug)]
pub struct OutboundSend {
    channel: LarkChannel,
    to: MessageRecipient,
}

impl OutboundSend {
    /// 发送文本。
    pub async fn text(self, text: impl Into<String>) -> Result<SentMessage, ChannelError> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(ChannelError::EmptyText);
        }
        self.channel.inner.outbound.send_text(self.to, text).await
    }

    /// 发送一张卡片。
    pub async fn card(self, card: CardDocument) -> Result<SentMessage, ChannelError> {
        self.channel.inner.outbound.send_card(self.to, card).await
    }

    /// 发出第一张卡片，并返回只服务这一条消息的补丁流。
    pub async fn card_stream(self, first: CardDocument) -> Result<CardStream, ChannelError> {
        let outbound = Arc::clone(&self.channel.inner.outbound);
        let sent = self.card(first).await?;
        CardStream::start(outbound, sent.message_id).await
    }
}

/// 按消息 id 回复。
#[derive(Debug)]
pub struct OutboundReply {
    channel: LarkChannel,
    message_id: MessageId,
    in_thread: bool,
}

impl OutboundReply {
    /// 在话题里回复。入站 `thread_id` 不会自动打开这个开关。
    pub fn in_thread(mut self) -> Self {
        self.in_thread = true;
        self
    }

    /// 回复文本。
    pub async fn text(self, text: impl Into<String>) -> Result<SentMessage, ChannelError> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(ChannelError::EmptyText);
        }
        self.channel
            .inner
            .outbound
            .reply_text(self.message_id.as_str().to_string(), text, self.in_thread)
            .await
    }

    /// 回复一张卡片。
    pub async fn card(self, card: CardDocument) -> Result<SentMessage, ChannelError> {
        self.channel
            .inner
            .outbound
            .reply_card(self.message_id.as_str().to_string(), card, self.in_thread)
            .await
    }

    /// 回复第一张卡片，并返回补丁流。
    pub async fn card_stream(self, first: CardDocument) -> Result<CardStream, ChannelError> {
        let outbound = Arc::clone(&self.channel.inner.outbound);
        let sent = self.card(first).await?;
        CardStream::start(outbound, sent.message_id).await
    }
}

/// 一条消息上的卡片补丁流。队列容量 100。
pub struct CardStream {
    message_id: MessageId,
    tx: Mutex<Option<mpsc::Sender<PatchJob>>>,
    done: Mutex<Option<oneshot::Receiver<()>>>,
    closed: AtomicBool,
}

impl std::fmt::Debug for CardStream {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CardStream")
            .field("message_id", &self.message_id)
            .field("closed", &self.closed.load(Ordering::Relaxed))
            .finish()
    }
}

impl CardStream {
    /// 等到这一次补丁的 HTTP 结果。队列满时堵在入队。
    pub async fn update(&self, card: CardDocument) -> Result<(), ChannelError> {
        if self.closed.load(Ordering::Acquire) {
            return Err(ChannelError::StreamClosed);
        }
        let (ack_tx, ack_rx) = oneshot::channel();
        let job = PatchJob {
            body: gate::patch_body(&card),
            ack: ack_tx,
        };
        let sender = lock(&self.tx).clone();
        let Some(sender) = sender else {
            return Err(ChannelError::StreamClosed);
        };
        sender
            .send(job)
            .await
            .map_err(|_| ChannelError::StreamClosed)?;
        ack_rx.await.map_err(|_| ChannelError::StreamClosed)?
    }

    /// 第一次会等已入队的补丁发完。之后再调用直接成功。
    pub async fn close(&self) -> Result<(), ChannelError> {
        if self.closed.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        lock(&self.tx).take();
        let done = lock(&self.done).take();
        if let Some(done) = done {
            let _ = done.await;
        }
        Ok(())
    }

    async fn start(
        outbound: Arc<dyn Outbound>,
        message_id: MessageId,
    ) -> Result<Self, ChannelError> {
        let handle = Handle::try_current().map_err(|_| ChannelError::NoRuntime)?;
        let (tx, mut rx) = mpsc::channel::<PatchJob>(PATCH_QUEUE);
        let (done_tx, done_rx) = oneshot::channel();
        let worker_id = message_id.as_str().to_string();
        handle.spawn(async move {
            while let Some(job) = rx.recv().await {
                let result = outbound.patch_card(worker_id.clone(), job.body).await;
                let _ = job.ack.send(result);
            }
            let _ = done_tx.send(());
        });
        Ok(Self {
            message_id,
            tx: Mutex::new(Some(tx)),
            done: Mutex::new(Some(done_rx)),
            closed: AtomicBool::new(false),
        })
    }
}

/// 会话入口。内部是 `Arc`，克隆只增加引用计数。
#[derive(Clone)]
pub struct LarkChannel {
    inner: Arc<Inner>,
}

impl std::fmt::Debug for LarkChannel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LarkChannel")
            .finish_non_exhaustive()
    }
}

/// 会话构建器。
pub struct LarkChannelBuilder {
    config: Arc<Config>,
    policy: InboundPolicy,
    on_message: Option<MessageHandler>,
    on_card: Option<CardHandler>,
    on_reject: Option<RejectHandler>,
    #[cfg(test)]
    outbound: Option<Arc<dyn Outbound>>,
}

impl LarkChannel {
    /// 用这份配置创建构建器。
    pub fn builder(config: Arc<Config>) -> LarkChannelBuilder {
        LarkChannelBuilder {
            config,
            policy: InboundPolicy::default(),
            on_message: None,
            on_card: None,
            on_reject: None,
            #[cfg(test)]
            outbound: None,
        }
    }

    /// 取机器人身份，注册私有分发器，并循环打开长连接。
    ///
    /// 只有正常断开会重连。其它错误返回给调用方。并发的第二次调用返回
    /// [`ChannelError::AlreadyConnected`]。这次返回之后可以再调用。
    pub async fn connect(&self) -> Result<(), ChannelError> {
        if self
            .inner
            .state
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(ChannelError::AlreadyConnected);
        }
        let _guard = RunningGuard(&self.inner.state);
        self.inner.stopped.store(false, Ordering::Release);
        self.ensure_workers()?;
        self.ensure_bot().await?;
        if self.inner.stopped.load(Ordering::Acquire) {
            return Ok(());
        }
        let channel = self.clone();
        let stopped = Arc::clone(&self.inner.stopped);
        let stop = Arc::clone(&self.inner.stop);
        reconnect_loop(
            move || {
                let channel = channel.clone();
                async move { channel.open_once().await }
            },
            move |delay| {
                let stopped = Arc::clone(&stopped);
                let stop = Arc::clone(&stop);
                async move { interruptible_sleep(delay, &stop, &stopped).await }
            },
            &self.inner.stopped,
        )
        .await
    }

    /// 置位停止。正在进行的 `open` 不会被取消，返回后不再重连。
    pub fn shutdown(&self) {
        self.inner.stopped.store(true, Ordering::Release);
        self.inner.stop.notify_waiters();
    }

    /// 向一个接收者发消息。
    pub fn send(&self, to: MessageRecipient) -> OutboundSend {
        OutboundSend {
            channel: self.clone(),
            to,
        }
    }

    /// 回复这条入站消息。
    pub fn reply(&self, msg: &InboundMessage) -> OutboundReply {
        self.reply_to(&msg.message_id)
    }

    /// 回复指定消息 id。
    pub fn reply_to(&self, message_id: &MessageId) -> OutboundReply {
        OutboundReply {
            channel: self.clone(),
            message_id: message_id.clone(),
            in_thread: false,
        }
    }

    fn ensure_workers(&self) -> Result<(), ChannelError> {
        if self.inner.workers_started.load(Ordering::Acquire) {
            return Ok(());
        }
        // 先确认当前线程有 runtime。失败时闩保持关闭，之后在 runtime 里重试还能建 worker。
        let handle = Handle::try_current().map_err(|_| ChannelError::NoRuntime)?;
        if self.inner.workers_started.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        *lock(&self.inner.runtime) = Some(handle.clone());
        if let Some(handler) = self.inner.on_message.clone() {
            let (tx, mut rx) = mpsc::channel(MESSAGE_QUEUE);
            *lock(&self.inner.message_tx) = Some(tx);
            let channel = self.clone();
            handle.spawn(async move {
                while let Some(message) = rx.recv().await {
                    if let Err(err) = handler(message, channel.clone()).await {
                        tracing::error!(error = %err, "on_message 失败");
                    }
                }
            });
        }
        if let Some(handler) = self.inner.on_reject.clone() {
            let (tx, mut rx) = mpsc::channel::<(InboundMessage, RejectReason)>(REJECT_QUEUE);
            *lock(&self.inner.reject_tx) = Some(tx);
            handle.spawn(async move {
                while let Some((message, reason)) = rx.recv().await {
                    if let Err(err) = handler(message, reason).await {
                        tracing::error!(error = %err, "on_reject 失败");
                    }
                }
            });
        }
        Ok(())
    }

    async fn ensure_bot(&self) -> Result<(), ChannelError> {
        if self.inner.bot.get().is_some() {
            return Ok(());
        }
        let id = self.inner.outbound.bot_open_id().await?;
        if id.trim().is_empty() {
            return Err(ChannelError::BotIdentity);
        }
        let _ = self.inner.bot.set(id);
        Ok(())
    }

    async fn open_once(&self) -> Result<(), WsClientError> {
        let handler = match self.make_dispatcher() {
            Ok(handler) => handler,
            Err(err) => {
                tracing::error!(error = %err, "注册分发器失败");
                return Err(WsClientError::UnexpectedResponse);
            }
        };
        LarkWsClient::open(Arc::clone(&self.inner.config), handler).await
    }

    fn make_dispatcher(&self) -> Result<EventDispatcherHandler, String> {
        let mut dispatcher = EventDispatcherHandler::builder();
        if self.inner.on_message.is_some() || self.inner.on_reject.is_some() {
            let channel = self.clone();
            dispatcher =
                dispatcher.register_im_message_receive_v1(move |event| channel.on_im(event))?;
        }
        if self.inner.on_card.is_some() {
            let channel = self.clone();
            dispatcher =
                dispatcher.register_card_action_trigger(move |event| channel.on_card(event))?;
        }
        Ok(dispatcher.build())
    }

    fn on_im(
        &self,
        event: ImMessageReceiveV1,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bot = self.inner.bot.get().map(String::as_str).unwrap_or("");
        let Some(message) = gate::normalize_message(&event, bot) else {
            return Ok(());
        };
        if !bot.is_empty() && message.sender.open_id == bot {
            tracing::debug!(
                message_id = message.message_id.as_str(),
                "丢弃机器人自己的消息"
            );
            return Ok(());
        }
        if !self.remember(&self.inner.seen_messages, message.message_id.as_str()) {
            return Ok(());
        }
        match gate::decide(&self.inner.policy, &message) {
            Ok(()) => self.enqueue_message(message),
            Err(reason) => self.enqueue_reject(message, reason),
        }
        Ok(())
    }

    fn on_card(
        &self,
        event: CardActionTrigger,
    ) -> Result<Option<CardActionTriggerResponse>, Box<dyn std::error::Error + Send + Sync>> {
        let Some(handler) = self.inner.on_card.clone() else {
            return Ok(None);
        };
        let Some(action) = gate::normalize_card(&event) else {
            return Ok(None);
        };
        let key = gate::card_dedup_key(&action);
        if !self.remember(&self.inner.seen_cards, &key) {
            return Ok(None);
        }
        let channel = self.clone();
        let ack = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            handler(channel, action)
        })) {
            Ok(ack) => ack,
            Err(_) => {
                tracing::error!("卡片回调 panic");
                CardAck::toast("内部错误")
            }
        };
        let (response, after) = ack.split();
        if let Some(future) = after {
            self.spawn_after(future);
        }
        Ok(response)
    }

    fn enqueue_message(&self, message: InboundMessage) {
        let sender = lock(&self.inner.message_tx).clone();
        match sender {
            Some(sender) => {
                if sender.try_send(message).is_err() {
                    self.note_drop("消息队列已满，丢弃入站消息");
                }
            }
            None => self.note_drop("消息 worker 不在，丢弃入站消息"),
        }
    }

    fn enqueue_reject(&self, message: InboundMessage, reason: RejectReason) {
        let sender = lock(&self.inner.reject_tx).clone();
        match sender {
            Some(sender) => {
                if sender.try_send((message, reason)).is_err() {
                    self.note_drop("拒绝队列已满，丢弃入站消息");
                }
            }
            None => {
                tracing::debug!(?reason, "入站消息被策略拒绝");
            }
        }
    }

    fn note_drop(&self, message: &'static str) {
        self.inner.dropped.fetch_add(1, Ordering::Relaxed);
        tracing::debug!("{message}");
    }

    fn remember(&self, table: &Mutex<Dedup>, key: &str) -> bool {
        lock(table).insert(key)
    }

    fn spawn_after(&self, future: SharedFuture) {
        let Some(handle) = lock(&self.inner.runtime).clone() else {
            tracing::error!("没有运行时，丢弃卡片 after");
            return;
        };
        handle.spawn(async move {
            if let Err(err) = future.await {
                tracing::error!(error = %err, "卡片 after 失败");
            }
        });
    }
}

impl LarkChannelBuilder {
    /// 替换入站策略。
    pub fn policy(mut self, policy: InboundPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// 允许进入业务的消息。回调在独立 worker 上 `await`，不挡住长连接 ACK。
    pub fn on_message<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(InboundMessage, LarkChannel) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), ChannelError>> + Send + 'static,
    {
        self.on_message = Some(Arc::new(move |message, channel| {
            Box::pin(handler(message, channel))
        }));
        self
    }

    /// 同步卡片回调。返回值写入这次 ACK。
    pub fn on_card_action<F>(mut self, handler: F) -> Self
    where
        F: Fn(LarkChannel, InboundCardAction) -> CardAck + Send + Sync + 'static,
    {
        self.on_card = Some(Arc::new(handler));
        self
    }

    /// 被策略拒绝的消息。不进入 `on_message`。
    pub fn on_reject<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(InboundMessage, RejectReason) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), ChannelError>> + Send + 'static,
    {
        self.on_reject = Some(Arc::new(move |message, reason| {
            Box::pin(handler(message, reason))
        }));
        self
    }

    /// `on_message` 和 `on_card_action` 至少要有一个。
    pub fn build(self) -> Result<LarkChannel, ChannelError> {
        if self.on_message.is_none() && self.on_card.is_none() {
            return Err(ChannelError::NoHandler);
        }
        #[cfg(test)]
        let outbound = self
            .outbound
            .unwrap_or_else(|| Arc::new(LiveOutbound::new(Arc::clone(&self.config))));
        #[cfg(not(test))]
        let outbound = Arc::new(LiveOutbound::new(Arc::clone(&self.config)));
        Ok(LarkChannel {
            inner: Arc::new(Inner {
                config: self.config,
                policy: self.policy,
                outbound,
                on_message: self.on_message,
                on_card: self.on_card,
                on_reject: self.on_reject,
                bot: OnceLock::new(),
                message_tx: Mutex::new(None),
                reject_tx: Mutex::new(None),
                seen_messages: Mutex::new(Dedup::default()),
                seen_cards: Mutex::new(Dedup::default()),
                dropped: AtomicU64::new(0),
                workers_started: AtomicBool::new(false),
                state: AtomicU8::new(0),
                stopped: Arc::new(AtomicBool::new(false)),
                stop: Arc::new(Notify::new()),
                runtime: Mutex::new(None),
            }),
        })
    }

    #[cfg(test)]
    fn with_outbound(mut self, outbound: Arc<dyn Outbound>) -> Self {
        self.outbound = Some(outbound);
        self
    }
}

/// 消息接收者。与 communication helper 是同一个类型。
pub use openlark_communication::MessageRecipient;

struct Inner {
    config: Arc<Config>,
    policy: InboundPolicy,
    outbound: Arc<dyn Outbound>,
    on_message: Option<MessageHandler>,
    on_card: Option<CardHandler>,
    on_reject: Option<RejectHandler>,
    bot: OnceLock<String>,
    message_tx: Mutex<Option<mpsc::Sender<InboundMessage>>>,
    reject_tx: Mutex<Option<mpsc::Sender<(InboundMessage, RejectReason)>>>,
    seen_messages: Mutex<Dedup>,
    seen_cards: Mutex<Dedup>,
    dropped: AtomicU64,
    workers_started: AtomicBool,
    state: AtomicU8,
    stopped: Arc<AtomicBool>,
    stop: Arc<Notify>,
    runtime: Mutex<Option<Handle>>,
}

#[derive(Default)]
struct Dedup {
    order: VecDeque<String>,
    keys: HashSet<String>,
}

impl Dedup {
    fn insert(&mut self, key: &str) -> bool {
        if !self.keys.insert(key.to_string()) {
            return false;
        }
        self.order.push_back(key.to_string());
        if self.order.len() > DEDUP_LIMIT
            && let Some(old) = self.order.pop_front()
        {
            self.keys.remove(&old);
        }
        true
    }
}

struct PatchJob {
    body: Value,
    ack: oneshot::Sender<Result<(), ChannelError>>,
}

struct RunningGuard<'a>(&'a AtomicU8);

impl Drop for RunningGuard<'_> {
    fn drop(&mut self) {
        self.0.store(0, Ordering::Release);
    }
}

trait Outbound: Send + Sync {
    fn bot_open_id(&self) -> BoxFut<'_, Result<String, ChannelError>>;
    fn send_text(
        &self,
        to: MessageRecipient,
        text: String,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>>;
    fn send_card(
        &self,
        to: MessageRecipient,
        card: CardDocument,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>>;
    fn reply_text(
        &self,
        message_id: String,
        text: String,
        in_thread: bool,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>>;
    fn reply_card(
        &self,
        message_id: String,
        card: CardDocument,
        in_thread: bool,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>>;
    fn patch_card(&self, message_id: String, body: Value) -> BoxFut<'_, Result<(), ChannelError>>;
}

struct LiveOutbound {
    config: Arc<Config>,
    communication: CommunicationClient,
}

impl LiveOutbound {
    fn new(config: Arc<Config>) -> Self {
        let communication = CommunicationClient::new((*config).clone());
        Self {
            config,
            communication,
        }
    }
}

impl Outbound for LiveOutbound {
    fn bot_open_id(&self) -> BoxFut<'_, Result<String, ChannelError>> {
        let config = (*self.config).clone();
        Box::pin(async move {
            let response = BotService::new(config)
                .bot_info()
                .execute()
                .await
                .map_err(|_| ChannelError::BotIdentity)?;
            response
                .open_id()
                .map(str::to_string)
                .ok_or(ChannelError::BotIdentity)
        })
    }

    fn send_text(
        &self,
        to: MessageRecipient,
        text: String,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>> {
        let im = self.communication.im.clone();
        Box::pin(async move {
            let value = im.send_text(to, text).await?;
            sent_from_value(value)
        })
    }

    fn send_card(
        &self,
        to: MessageRecipient,
        card: CardDocument,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>> {
        let im = self.communication.im.clone();
        Box::pin(async move {
            let value = im.send_card(to, card.0).await?;
            sent_from_value(value)
        })
    }

    fn reply_text(
        &self,
        message_id: String,
        text: String,
        in_thread: bool,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>> {
        let im = self.communication.im.clone();
        Box::pin(async move {
            let target = reply_target(message_id, in_thread);
            let value = im.reply_text(target, text).await?;
            sent_from_value(value)
        })
    }

    fn reply_card(
        &self,
        message_id: String,
        card: CardDocument,
        in_thread: bool,
    ) -> BoxFut<'_, Result<SentMessage, ChannelError>> {
        let config = (*self.config).clone();
        Box::pin(async move {
            let body = ReplyMessageBody {
                content: card.as_value().to_string(),
                msg_type: "interactive".to_string(),
                reply_in_thread: Some(in_thread),
                uuid: None,
            };
            let value = ReplyMessageRequest::new(config)
                .message_id(message_id)
                .execute(body)
                .await?;
            sent_from_value(value)
        })
    }

    fn patch_card(&self, message_id: String, body: Value) -> BoxFut<'_, Result<(), ChannelError>> {
        let config = (*self.config).clone();
        Box::pin(async move {
            PatchMessageCardRequest::new(config)
                .message_id(message_id)
                .execute(body)
                .await?;
            Ok(())
        })
    }
}

fn reply_target(message_id: String, in_thread: bool) -> ReplyTarget {
    if in_thread {
        ReplyTarget::in_thread(message_id)
    } else {
        ReplyTarget::direct(message_id)
    }
}

fn sent_from_value(value: Value) -> Result<SentMessage, ChannelError> {
    let id = value
        .get("message_id")
        .and_then(Value::as_str)
        .unwrap_or("");
    Ok(SentMessage {
        message_id: MessageId::new(id)?,
    })
}

fn non_empty_ids(
    ids: impl IntoIterator<Item = impl Into<String>>,
) -> Result<NonEmptyIds, ChannelError> {
    let ids = ids
        .into_iter()
        .map(Into::into)
        .filter(|id| !id.trim().is_empty())
        .collect::<Vec<_>>();
    if ids.is_empty() {
        Err(ChannelError::EmptyAllowlist)
    } else {
        Ok(NonEmptyIds { ids })
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|err| err.into_inner())
}

async fn interruptible_sleep(delay: Duration, stop: &Notify, stopped: &AtomicBool) {
    let notified = stop.notified();
    tokio::pin!(notified);
    if stopped.load(Ordering::Acquire) {
        return;
    }
    tokio::select! {
        _ = tokio::time::sleep(delay) => {}
        _ = notified => {}
    }
}

pub(super) async fn reconnect_loop<Open, OpenFut, Sleep, SleepFut>(
    mut open: Open,
    mut sleep: Sleep,
    stopped: &AtomicBool,
) -> Result<(), ChannelError>
where
    Open: FnMut() -> OpenFut,
    OpenFut: Future<Output = Result<(), WsClientError>>,
    Sleep: FnMut(Duration) -> SleepFut,
    SleepFut: Future<Output = ()>,
{
    let mut delay = BACKOFF_START;
    loop {
        if stopped.load(Ordering::Acquire) {
            return Ok(());
        }
        let started = Instant::now();
        match open().await {
            Ok(()) | Err(WsClientError::ConnectionClosed { .. }) => {
                if stopped.load(Ordering::Acquire) {
                    return Ok(());
                }
                if started.elapsed() >= SESSION_RESET {
                    delay = BACKOFF_START;
                }
                let wait = delay;
                sleep(wait).await;
                if stopped.load(Ordering::Acquire) {
                    return Ok(());
                }
                delay = (delay * 2).min(BACKOFF_CAP);
            }
            Err(err) => return Err(ChannelError::Socket(err)),
        }
    }
}

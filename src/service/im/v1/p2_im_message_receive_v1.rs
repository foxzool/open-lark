use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Clone)]
pub struct P2ImMessageReceiveV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ImMessageReceiveV1Data,
pub(crate) struct P2ImMessageReceiveV1ProcessorImpl<F>,
where
    F: Fn(P2ImMessageReceiveV1) + 'static,
{
    f: F,
impl<F> EventHandler for P2ImMessageReceiveV1ProcessorImpl<F>,
where
    F: Fn(P2ImMessageReceiveV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {,
let message: P2ImMessageReceiveV1 = serde_json::from_slice(payload)?;
        (self.f)(message);
Ok(()),
    }
impl<F> P2ImMessageReceiveV1ProcessorImpl<F>,
where
    F: Fn(P2ImMessageReceiveV1) + 'static,
{,
pub(crate) fn new(f: F) -> Self {
        P2ImMessageReceiveV1ProcessorImpl { f }
/// 事件,
#[derive(Debug, Clone)]
pub struct P2ImMessageReceiveV1Data {
    pub sender: EventSender,
    pub message: EventMessage,
/// 事件的发送者,
#[derive(Debug, Clone)]
pub struct EventSender {
    /// 用户 ID
    pub sender_id: UserId,
    /// 消息发送者类型。目前只支持用户(user)发送的消息。
    pub sender_type: String,
    /// tenant key，为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，,
/// 也可以用作租户在应用里面的唯一标识,
    pub tenant_key: String,
/// 用户 ID,
#[derive(Debug, Clone)]
pub struct UserId {
    /// 用户的 union id
    pub union_id: String,
    /// 用户的 user id
    pub user_id: String,
    /// 用户的 open id
    pub open_id: String,
/// 事件中包含的消息内容,
#[derive(Debug, Clone)]
pub struct EventMessage {
    /// 消息的open_message_id
    pub message_id: String,
    /// 根消息id，用于回复消息场景
    pub root_id: Option<String>,
    /// 根消息id，父消息的id，用于回复消息场景
    pub parent_id: Option<String>,
    /// 消息创建时间戳（单位：毫秒）
    pub create_time: String,
    /// 消息更新时间戳（单位：毫秒）
    pub update_time: String,
    /// 消息所在的群组 ID
    pub chat_id: String,
    /// 消息所属的话题 ID（不返回说明该消息非话题消息）
    pub thread_id: Option<String>,
    /// 消息所在的群组类型,
///,
    /// 可选值有：,
///,
    /// - p2p：单聊,
/// - group： 群组,
    pub chat_type: String,
    /// 消息类型
    pub message_type: String,
    /// 消息内容
    pub content: String,
    /// 被提及用户的信息
    pub mentions: Option<Vec<MentionEvent>>,
    /// 用户代理数据，仅在接收事件的机器人具备获取客户端用户代理信息权限时返回
    pub user_agent: Option<String>,
/// 被提及用户的信息,
#[derive(Debug, Clone)]
pub struct MentionEvent {
    /// mention key
    pub key: String,
    /// 用户 ID
    pub id: UserId,
    /// 用户姓名
    pub name: String,
    /// tenant key，为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，,
/// 也可以用作租户在应用里面的唯一标识,
    pub tenant_key: String,
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use serde_json::json;
use crate::event::context::EventContext;
    #[test]
fn test_decode() {
        let p1 = json!({"schema":"2.0","header":{"event_id":"7db4fd0bb90cfa6127e3aaa446d39b37","token
        ":"","create_time":"1719211482721","event_type":"im.message.receive_v1","tenant_key":"tenant_key","app_id":"app_id"}
        "event":,
{,
            "message":,
{,
                "chat_id":"oc_84d53efe245072c16ba4b4ff597f52f3",
                "chat_type":"group",
                "content":"{\"text\":\"55u\"}",
                "create_time":"1719211482485",
                "message_id":"om_b1e37040d3f888af8a7e47affae94360",
                "message_type":"text","update_time":"1719211482485",
            "sender":,
{,
                "sender_id":{
                    "open_id":"ou_b434284f852b1531071855b16d19f40b",
                    "union_id":"on_526dbf7b9ef1fda341aecb79a2481662",
                    "user_id":"aa5cf9d7",
                "sender_type":"user",
                "tenant_key":"133195a24e8f575d",
});
let event_context: EventContext = serde_json::from_value(p1).unwrap();
        // println!("{:#?}", event_context);
        assert_eq!(event_context.schema, Some("2.0".to_string()));

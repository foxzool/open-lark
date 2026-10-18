//! 入站归一化、策略和补丁 body。没有 I/O。

use serde_json::{Value, json};

use super::{
    CardDocument, ChatKind, DmMode, GroupAllowlist, InboundCardAction, InboundContent,
    InboundMessage, InboundPolicy, Mention, MessageId, RejectReason, Sender,
};
use crate::ws_client::{CardActionTrigger, ImMention, ImMessageReceiveV1};

pub(super) fn normalize_message(
    event: &ImMessageReceiveV1,
    bot_open_id: &str,
) -> Option<InboundMessage> {
    let message = &event.event.message;
    let message_id = MessageId::new(&message.message_id).ok()?;
    let content = normalize_content(&message.message_type, &message.content)?;
    let mentions = message
        .mentions
        .iter()
        .map(mention_from)
        .collect::<Vec<_>>();
    let mention_all = mentions.iter().any(|item| is_mention_all_key(&item.key))
        || content_has_mention_all(&message.content);
    let mentioned_bot =
        !bot_open_id.is_empty() && mentions.iter().any(|item| item.open_id == bot_open_id);
    let sender = event
        .event
        .sender
        .as_ref()
        .and_then(|sender| sender.sender_id.as_ref());

    Some(InboundMessage {
        event_id: event.header.event_id.clone(),
        message_id,
        root_id: MessageId::new(&message.root_id).ok(),
        parent_id: MessageId::new(&message.parent_id).ok(),
        thread_id: MessageId::new(&message.thread_id).ok(),
        chat_id: message.chat_id.clone(),
        chat_kind: chat_kind(&message.chat_type),
        sender: Sender {
            open_id: sender.map(|id| id.open_id.clone()).unwrap_or_default(),
            user_id: sender.map(|id| id.user_id.clone()).unwrap_or_default(),
            union_id: sender.map(|id| id.union_id.clone()).unwrap_or_default(),
        },
        content,
        mentions,
        mention_all,
        mentioned_bot,
        create_time_ms: parse_time(&message.create_time)
            .or_else(|| parse_time(&event.header.create_time)),
    })
}

pub(super) fn normalize_card(event: &CardActionTrigger) -> Option<InboundCardAction> {
    let context = event.event.context.as_ref()?;
    let message_id = MessageId::new(&context.open_message_id).ok()?;
    let action = event.event.action.as_ref()?;
    let operator = event.event.operator.as_ref();
    Some(InboundCardAction {
        event_id: event.header.event_id.clone(),
        message_id,
        chat_id: context.open_chat_id.clone(),
        operator_open_id: operator
            .map(|item| item.open_id.clone())
            .unwrap_or_default(),
        tag: action.tag.clone(),
        name: action.name.clone(),
        value: action.value.clone(),
        form_value: action.form_value.clone(),
        token: event.event.token.clone(),
    })
}

pub(super) fn card_dedup_key(action: &InboundCardAction) -> String {
    if !action.event_id.trim().is_empty() {
        return action.event_id.clone();
    }
    json!({
        "message_id": action.message_id.as_str(),
        "operator_open_id": action.operator_open_id,
        "tag": action.tag,
        "name": action.name,
        "value": action.value,
        "form_value": action.form_value,
    })
    .to_string()
}

/// 群的顺序与官方 policy gate 一致：白名单，没 @，然后 @所有人。
pub(super) fn decide(policy: &InboundPolicy, msg: &InboundMessage) -> Result<(), RejectReason> {
    match &msg.chat_kind {
        ChatKind::Other(_) => Err(RejectReason::ChatKindUnsupported),
        ChatKind::Group => decide_group(policy, msg),
        ChatKind::P2p => decide_dm(policy, msg),
    }
}

pub(super) fn patch_body(card: &CardDocument) -> Value {
    json!({ "content": card.as_value().to_string() })
}

fn decide_group(policy: &InboundPolicy, msg: &InboundMessage) -> Result<(), RejectReason> {
    if let GroupAllowlist::Only(ids) = &policy.groups
        && !ids.contains(&msg.chat_id)
    {
        return Err(RejectReason::GroupNotAllowed);
    }
    if policy.require_mention && !msg.mentioned_bot && !msg.mention_all {
        return Err(RejectReason::NoMention);
    }
    if msg.mention_all && !policy.respond_to_mention_all {
        return Err(RejectReason::MentionAllBlocked);
    }
    Ok(())
}

fn decide_dm(policy: &InboundPolicy, msg: &InboundMessage) -> Result<(), RejectReason> {
    match &policy.dm {
        DmMode::Open => Ok(()),
        DmMode::Disabled => Err(RejectReason::DmDisabled),
        DmMode::Allowlist(ids) => {
            if ids.contains(&msg.sender.open_id) {
                Ok(())
            } else {
                Err(RejectReason::SenderNotAllowed)
            }
        }
    }
}

fn normalize_content(message_type: &str, raw: &str) -> Option<InboundContent> {
    let value: Value = serde_json::from_str(raw).ok()?;
    if message_type == "text" {
        let text = value.get("text")?.as_str()?.to_string();
        return Some(InboundContent::Text(text));
    }
    Some(InboundContent::Other {
        message_type: message_type.to_string(),
        value,
    })
}

fn mention_from(mention: &ImMention) -> Mention {
    Mention {
        key: mention.key.clone(),
        open_id: mention
            .id
            .as_ref()
            .map(|id| id.open_id.clone())
            .unwrap_or_default(),
        name: mention.name.clone(),
    }
}

fn is_mention_all_key(key: &str) -> bool {
    key == "@_all" || key == "@all"
}

fn content_has_mention_all(raw: &str) -> bool {
    raw.contains("@_all") || raw.contains("@all")
}

fn chat_kind(chat_type: &str) -> ChatKind {
    match chat_type {
        "p2p" => ChatKind::P2p,
        "group" => ChatKind::Group,
        other => ChatKind::Other(other.to_string()),
    }
}

fn parse_time(raw: &str) -> Option<i64> {
    let raw = raw.trim();
    if raw.is_empty() {
        None
    } else {
        raw.parse().ok()
    }
}

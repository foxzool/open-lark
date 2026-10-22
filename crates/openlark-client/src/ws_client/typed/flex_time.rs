//! JSON 字符串与数字互认的时间戳反序列化。
//!
//! 飞书事件里同一字段可能是 `"1700000000000"`，也可能是 `1700000000000`。
//! 字符串字段把整数收成十进制文本；整数字段把十进制字符串解析成 `i64`。

use std::fmt;

use serde::de::{self, Deserializer, Visitor};

/// 反序列化 `String` 时间戳。缺字段仍由 `#[serde(default)]` 处理。
pub(super) fn de_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(StringVisitor)
}

/// 反序列化 `Option<String>` 时间戳。`null` 为 `None`，空字符串仍是 `Some("")`。
pub(super) fn de_opt_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(OptStringVisitor)
}

/// 反序列化 `Option<i64>` 时间戳。`null` 与 `""` 为 `None`，非整数报错。
pub(super) fn de_opt_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(OptI64Visitor)
}

fn integer_from_f64<E: de::Error>(value: f64) -> Result<i64, E> {
    if !value.is_finite() || value.fract() != 0.0 {
        return Err(E::custom(format!("时间戳不是整数: {value}")));
    }
    let as_i64 = value as i64;
    if as_i64 as f64 != value {
        return Err(E::custom(format!("时间戳超出 i64: {value}")));
    }
    Ok(as_i64)
}

fn i64_from_u64<E: de::Error>(value: u64) -> Result<i64, E> {
    i64::try_from(value).map_err(|_| E::custom(format!("时间戳超出 i64: {value}")))
}

fn i64_from_str<E: de::Error>(value: &str) -> Result<Option<i64>, E> {
    let value = value.trim();
    if value.is_empty() {
        return Ok(None);
    }
    value
        .parse::<i64>()
        .map(Some)
        .map_err(|_| E::custom(format!("时间戳不是十进制整数: {value}")))
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("时间戳字符串或整数")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<String, E> {
        Ok(value.to_owned())
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<String, E> {
        Ok(value)
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<String, E> {
        Ok(value.to_string())
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<String, E> {
        Ok(value.to_string())
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<String, E> {
        integer_from_f64(value).map(|value| value.to_string())
    }

    fn visit_unit<E: de::Error>(self) -> Result<String, E> {
        Err(E::custom("时间戳不能为 null"))
    }
}

struct OptStringVisitor;

impl<'de> Visitor<'de> for OptStringVisitor {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("时间戳字符串、整数或 null")
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringVisitor).map(Some)
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Some(value.to_owned()))
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(Some(value))
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Some(value.to_string()))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Some(value.to_string()))
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
        integer_from_f64(value).map(|value| Some(value.to_string()))
    }
}

struct OptI64Visitor;

impl<'de> Visitor<'de> for OptI64Visitor {
    type Value = Option<i64>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("时间戳整数、十进制字符串或 null")
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(OptI64Visitor)
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        i64_from_str(value)
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Self::Value, E> {
        i64_from_str(&value)
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Some(value))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        i64_from_u64(value).map(Some)
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
        integer_from_f64(value).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::super::corehr_v2;
    use super::super::{EventHeader, ImReceivedMessage, approval, contact, drive, hire, im, vc};

    macro_rules! opt_string_case {
        ($ty:ty, $field:ident) => {{
            let num: $ty =
                serde_json::from_str(concat!("{\"", stringify!($field), "\":1700000000000}"))
                    .expect(concat!(stringify!($ty), " number"));
            let text: $ty =
                serde_json::from_str(concat!("{\"", stringify!($field), "\":\"1700000000000\"}"))
                    .expect(concat!(stringify!($ty), " string"));
            assert_eq!(num.$field.as_deref(), Some("1700000000000"));
            assert_eq!(text.$field.as_deref(), Some("1700000000000"));
        }};
    }

    macro_rules! opt_i64_case {
        ($ty:ty, $field:ident) => {{
            let num: $ty =
                serde_json::from_str(concat!("{\"", stringify!($field), "\":1700000000000}"))
                    .expect(concat!(stringify!($ty), " number"));
            let text: $ty =
                serde_json::from_str(concat!("{\"", stringify!($field), "\":\"1700000000000\"}"))
                    .expect(concat!(stringify!($ty), " string"));
            assert_eq!(num.$field, Some(1_700_000_000_000));
            assert_eq!(text.$field, Some(1_700_000_000_000));
            let bad =
                serde_json::from_str::<$ty>(concat!("{\"", stringify!($field), "\":\"nope\"}"));
            assert!(
                bad.is_err(),
                concat!(stringify!($ty), " should reject nope")
            );
        }};
    }

    macro_rules! string_case {
        ($ty:ty, $field:ident) => {{
            let num: $ty =
                serde_json::from_str(concat!("{\"", stringify!($field), "\":1700000000000}"))
                    .expect(concat!(stringify!($ty), " number"));
            let text: $ty =
                serde_json::from_str(concat!("{\"", stringify!($field), "\":\"1700000000000\"}"))
                    .expect(concat!(stringify!($ty), " string"));
            assert_eq!(num.$field, "1700000000000");
            assert_eq!(text.$field, "1700000000000");
        }};
    }

    #[test]
    fn timestamp_fields_accept_json_number_and_decimal_string() {
        opt_i64_case!(approval::ApprovalEvent, timestamp);
        opt_string_case!(approval::ApprovalInstanceStatusChangedV4Event, operate_time);
        opt_string_case!(approval::ApprovalTaskStatusChangedV4Event, operate_time);
        opt_i64_case!(contact::UserEvent, join_time);
        opt_i64_case!(contact::User, join_time);
        opt_string_case!(contact::UserAssignInfo, start_time);
        opt_string_case!(contact::UserAssignInfo, end_time);
        opt_string_case!(corehr_v2::CorehrEmployeeDomainEventV2Event, opt_time);
        opt_string_case!(corehr_v2::CorehrJobChangeUpdatedV2Event, updated_time);
        opt_string_case!(corehr_v2::CorehrOffboardingUpdatedV2Event, updated_time);
        opt_i64_case!(drive::DriveFileBitableFieldChangedV1Event, update_time);
        opt_i64_case!(drive::DriveFileBitableRecordChangedV1Event, update_time);
        opt_i64_case!(hire::HireApplicationStageChangedV1Event, update_time);
        opt_string_case!(hire::HireReferralAccountAssetsUpdateV1Event, modify_time);
        opt_string_case!(
            im::ImChatAccessEventBotP2pChatEnteredV1Event,
            last_message_create_time
        );
        opt_string_case!(im::EventMessageReader, read_time);
        opt_string_case!(im::ImMessageRecalledV1Event, recall_time);
        opt_string_case!(im::ImMessageReactionCreatedV1Event, action_time);
        opt_string_case!(im::ImMessageReactionDeletedV1Event, action_time);
        opt_string_case!(vc::VcBotMeetingInvitedV1Event, invite_time);
        opt_string_case!(vc::MeetingAgentEventMeeting, start_time);
        opt_string_case!(vc::MeetingAgentEventMeeting, end_time);
        opt_string_case!(vc::MeetingEventMeeting, start_time);
        opt_string_case!(vc::MeetingEventMeeting, end_time);
        opt_string_case!(vc::TimeConfig, start_time);
        opt_string_case!(vc::TimeConfig, end_time);
        opt_string_case!(vc::ParticipantJoinedItem, join_time);
        opt_string_case!(vc::ParticipantLeftItem, leave_time);
        opt_string_case!(vc::ChatMessageItem, send_time);
        opt_string_case!(vc::MagicShareStartedItem, time);
        opt_string_case!(vc::MagicShareEndedItem, time);
        opt_string_case!(vc::DocumentContextChangedItem, time);
        opt_string_case!(vc::CountdownItem, end_time);
        opt_string_case!(vc::CountdownItem, event_time);
        opt_string_case!(vc::CountdownItem, countdown_set_time);
        opt_string_case!(vc::RoomStatusEvent, disable_start_time);
        opt_string_case!(vc::RoomStatusEvent, disable_end_time);
        string_case!(EventHeader, create_time);
        string_case!(ImReceivedMessage, create_time);
        string_case!(ImReceivedMessage, update_time);
    }

    #[derive(Debug, Deserialize)]
    struct OptI64Probe {
        #[serde(default, deserialize_with = "super::de_opt_i64")]
        value: Option<i64>,
    }

    #[test]
    fn opt_i64_rejects_fraction_and_accepts_null_or_blank() {
        assert!(serde_json::from_str::<OptI64Probe>(r#"{"value":1.5}"#).is_err());
        let blank: OptI64Probe = serde_json::from_str(r#"{"value":""}"#).unwrap();
        assert_eq!(blank.value, None);
        let null: OptI64Probe = serde_json::from_str(r#"{"value":null}"#).unwrap();
        assert_eq!(null.value, None);
        let missing: OptI64Probe = serde_json::from_str("{}").unwrap();
        assert_eq!(missing.value, None);
    }
}

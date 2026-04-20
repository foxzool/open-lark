//! OpenLark Meeting 契约测试
//!
//! 测试会议与日程服务模块的主要数据模型序列化/反序列化契约。

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, json, to_value, Value};

fn assert_json_contract<T>(value: &T, expected: Value)
where
    T: Serialize,
{
    assert_eq!(to_value(value).unwrap(), expected);
}

fn parse_contract<T>(payload: Value) -> T
where
    T: DeserializeOwned,
{
    from_value(payload).unwrap()
}

#[test]
fn test_json_serialization_contracts() {
    let test_data = json!({
        "meeting_id": "test_meeting_123",
        "topic": "Test Meeting"
    });

    let serialized = to_value(&test_data).unwrap();
    let deserialized: Value = parse_contract(serialized.clone());
    assert_eq!(serialized, deserialized);
}

#[test]
fn test_vc_room_contract() {
    let room = json!({
        "room_id": "room_123",
        "room_name": "会议室A",
        "capacity": 20,
        "status": 1
    });

    let parsed: Value = parse_contract(room.clone());
    assert_eq!(parsed["room_id"], "room_123");
    assert_json_contract(&parsed, room);
}

#[test]
fn test_vc_meeting_contract() {
    let meeting = json!({
        "meeting_id": "meeting_123",
        "topic": "项目周会",
        "start_time": "2024-01-15T14:00:00Z",
        "end_time": "2024-01-15T16:00:00Z",
        "status": "ongoing"
    });

    let parsed: Value = parse_contract(meeting.clone());
    assert_eq!(parsed["meeting_id"], "meeting_123");
    assert_json_contract(&parsed, meeting);
}

#[test]
fn test_calendar_contract() {
    let calendar = json!({
        "calendar_id": "cal_123",
        "summary": "团队日历",
        "color": "#1890ff"
    });

    let parsed: Value = parse_contract(calendar.clone());
    assert_eq!(parsed["calendar_id"], "cal_123");
    assert_json_contract(&parsed, calendar);
}

#[test]
fn test_calendar_event_contract() {
    let event = json!({
        "event_id": "event_123",
        "summary": "项目评审",
        "start_time": "2024-01-15T10:00:00Z",
        "end_time": "2024-01-15T11:00:00Z",
        "attendees": [
            {"user_id": "user_1"},
            {"user_id": "user_2"}
        ]
    });

    let parsed: Value = parse_contract(event.clone());
    assert_eq!(parsed["event_id"], "event_123");
    assert_json_contract(&parsed, event);
}

#[test]
fn test_meeting_room_contract() {
    let meeting_room = json!({
        "room_id": "room_123",
        "name": "大会议室",
        "capacity": 50
    });

    let parsed: Value = parse_contract(meeting_room.clone());
    assert_eq!(parsed["room_id"], "room_123");
    assert_json_contract(&parsed, meeting_room);
}

#[test]
fn test_reservation_contract() {
    let reservation = json!({
        "reserve_id": "res_123",
        "room_id": "room_456",
        "topic": "部门会议",
        "status": "confirmed"
    });

    let parsed: Value = parse_contract(reservation.clone());
    assert_eq!(parsed["reserve_id"], "res_123");
    assert_json_contract(&parsed, reservation);
}

#[test]
fn test_freebusy_contract() {
    let freebusy = json!({
        "user_id": "user_123",
        "busy_times": [
            {
                "start_time": "2024-01-15T09:00:00Z",
                "end_time": "2024-01-15T10:00:00Z"
            }
        ]
    });

    let parsed: Value = parse_contract(freebusy.clone());
    assert_eq!(parsed["user_id"], "user_123");
    assert_json_contract(&parsed, freebusy);
}

#[test]
fn test_error_response_contract() {
    let error_response = json!({
        "code": 400,
        "msg": "Room not available"
    });

    let parsed: Value = parse_contract(error_response.clone());
    assert_eq!(parsed["code"], 400);
    assert_json_contract(&parsed, error_response);
}

#[test]
fn test_list_response_contract() {
    let list_response = json!({
        "items": [
            {"room_id": "room_1", "name": "会议室1"}
        ],
        "has_more": false
    });

    let parsed: Value = parse_contract(list_response.clone());
    assert_eq!(parsed["items"].as_array().unwrap().len(), 1);
    assert_json_contract(&parsed, list_response);
}

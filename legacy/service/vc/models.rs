use serde::{Deserialize, Serialize};

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserIdType {
    /// 用户ID
    #[serde(rename = "user_id")]
    UserId,
    /// union_id
    #[serde(rename = "union_id")]
    UnionId,
    /// open_id
    #[serde(rename = "open_id")]
    OpenId,
}

impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
            UserIdType::OpenId => "open_id",
        }
    }
}

/// 会议室ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoomIdType {
    /// 会议室ID
    #[serde(rename = "room_id")]
    RoomId,
    /// 会议室名称
    #[serde(rename = "omm_room_id")]
    OmmRoomId,
}

impl RoomIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RoomIdType::RoomId => "room_id",
            RoomIdType::OmmRoomId => "omm_room_id",
        }
    }
}

/// 会议状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MeetingStatus {
    /// 未开始
    #[serde(rename = "not_started")]
    NotStarted,
    /// 进行中
    #[serde(rename = "in_progress")]
    InProgress,
    /// 已结束
    #[serde(rename = "ended")]
    Ended,
    /// 已取消
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// 会议类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MeetingType {
    /// 即时会议
    #[serde(rename = "instant")]
    Instant,
    /// 预约会议
    #[serde(rename = "scheduled")]
    Scheduled,
    /// 周期性会议
    #[serde(rename = "recurring")]
    Recurring,
}

/// 预约会议信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reserve {
    /// 预约ID
    pub id: String,
    /// 会议主题
    pub topic: String,
    /// 会议号
    pub meeting_no: String,
    /// 会议密码
    pub password: Option<String>,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
    /// 主持人
    pub host_user: Option<UserInfo>,
    /// 状态
    pub status: MeetingStatus,
    /// 会议类型
    pub meeting_type: MeetingType,
    /// 创建时间
    pub create_time: Option<String>,
}

/// 会议信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting {
    /// 会议ID
    pub id: String,
    /// 会议主题
    pub topic: String,
    /// 会议号
    pub meeting_no: String,
    /// 会议密码
    pub password: Option<String>,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: Option<String>,
    /// 主持人
    pub host_user: Option<UserInfo>,
    /// 状态
    pub status: MeetingStatus,
    /// 参会人数
    pub participant_count: Option<i32>,
    /// 创建时间
    pub create_time: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub id: String,
    /// 用户名称
    pub name: Option<String>,
    /// 用户头像
    pub avatar_url: Option<String>,
}

/// 会议室信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    /// 会议室ID
    pub room_id: String,
    /// 会议室名称
    pub name: String,
    /// 会议室描述
    pub description: Option<String>,
    /// 会议室容量
    pub capacity: Option<i32>,
    /// 会议室位置
    pub location: Option<String>,
    /// 会议室状态
    pub status: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
}

/// 录制信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recording {
    /// 录制ID
    pub recording_id: String,
    /// 会议ID
    pub meeting_id: String,
    /// 录制标题
    pub title: Option<String>,
    /// 录制时长
    pub duration: Option<i32>,
    /// 录制大小
    pub size: Option<i64>,
    /// 录制状态
    pub status: Option<String>,
    /// 录制开始时间
    pub start_time: Option<String>,
    /// 录制结束时间
    pub end_time: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_vc_user_id_type_serialization() {
        let types = vec![UserIdType::UserId, UserIdType::UnionId, UserIdType::OpenId];

        for user_type in types {
            let serialized = serde_json::to_string(&user_type).unwrap();
            let deserialized: UserIdType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(user_type, deserialized);
        }
    }

    #[test]
    fn test_user_id_type_as_str() {
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
    }

    #[test]
    fn test_user_id_type_serde_rename() {
        let user_id = UserIdType::UserId;
        let serialized = serde_json::to_string(&user_id).unwrap();
        assert_eq!(serialized, "\"user_id\"");

        let union_id = UserIdType::UnionId;
        let serialized = serde_json::to_string(&union_id).unwrap();
        assert_eq!(serialized, "\"union_id\"");

        let open_id = UserIdType::OpenId;
        let serialized = serde_json::to_string(&open_id).unwrap();
        assert_eq!(serialized, "\"open_id\"");
    }

    #[test]
    fn test_room_id_type_serialization() {
        let types = vec![RoomIdType::RoomId, RoomIdType::OmmRoomId];

        for room_type in types {
            let serialized = serde_json::to_string(&room_type).unwrap();
            let deserialized: RoomIdType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(room_type, deserialized);
        }
    }

    #[test]
    fn test_room_id_type_as_str() {
        assert_eq!(RoomIdType::RoomId.as_str(), "room_id");
        assert_eq!(RoomIdType::OmmRoomId.as_str(), "omm_room_id");
    }

    #[test]
    fn test_room_id_type_serde_rename() {
        let room_id = RoomIdType::RoomId;
        let serialized = serde_json::to_string(&room_id).unwrap();
        assert_eq!(serialized, "\"room_id\"");

        let omm_room_id = RoomIdType::OmmRoomId;
        let serialized = serde_json::to_string(&omm_room_id).unwrap();
        assert_eq!(serialized, "\"omm_room_id\"");
    }

    #[test]
    fn test_meeting_status_serialization() {
        let statuses = vec![
            MeetingStatus::NotStarted,
            MeetingStatus::InProgress,
            MeetingStatus::Ended,
            MeetingStatus::Cancelled,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: MeetingStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_meeting_status_serde_rename() {
        let not_started = MeetingStatus::NotStarted;
        let serialized = serde_json::to_string(&not_started).unwrap();
        assert_eq!(serialized, "\"not_started\"");

        let in_progress = MeetingStatus::InProgress;
        let serialized = serde_json::to_string(&in_progress).unwrap();
        assert_eq!(serialized, "\"in_progress\"");

        let ended = MeetingStatus::Ended;
        let serialized = serde_json::to_string(&ended).unwrap();
        assert_eq!(serialized, "\"ended\"");

        let cancelled = MeetingStatus::Cancelled;
        let serialized = serde_json::to_string(&cancelled).unwrap();
        assert_eq!(serialized, "\"cancelled\"");
    }

    #[test]
    fn test_meeting_type_serialization() {
        let types = vec![
            MeetingType::Instant,
            MeetingType::Scheduled,
            MeetingType::Recurring,
        ];

        for meeting_type in types {
            let serialized = serde_json::to_string(&meeting_type).unwrap();
            let deserialized: MeetingType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(meeting_type, deserialized);
        }
    }

    #[test]
    fn test_meeting_type_serde_rename() {
        let instant = MeetingType::Instant;
        let serialized = serde_json::to_string(&instant).unwrap();
        assert_eq!(serialized, "\"instant\"");

        let scheduled = MeetingType::Scheduled;
        let serialized = serde_json::to_string(&scheduled).unwrap();
        assert_eq!(serialized, "\"scheduled\"");

        let recurring = MeetingType::Recurring;
        let serialized = serde_json::to_string(&recurring).unwrap();
        assert_eq!(serialized, "\"recurring\"");
    }

    #[test]
    fn test_user_info_serialization() {
        let user = UserInfo {
            id: "user_123".to_string(),
            name: Some("张三".to_string()),
            avatar_url: Some("https://avatar.example.com/123".to_string()),
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: UserInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(user.id, deserialized.id);
        assert_eq!(user.name, deserialized.name);
        assert_eq!(user.avatar_url, deserialized.avatar_url);
    }

    #[test]
    fn test_user_info_minimal() {
        let user = UserInfo {
            id: "minimal_user".to_string(),
            name: None,
            avatar_url: None,
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: UserInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(user.id, deserialized.id);
        assert!(deserialized.name.is_none());
        assert!(deserialized.avatar_url.is_none());
    }

    #[test]
    fn test_reserve_serialization() {
        let reserve = Reserve {
            id: "reserve_123".to_string(),
            topic: "团队周会".to_string(),
            meeting_no: "123456789".to_string(),
            password: Some("password123".to_string()),
            start_time: "2024-01-01T10:00:00Z".to_string(),
            end_time: "2024-01-01T11:00:00Z".to_string(),
            host_user: Some(UserInfo {
                id: "host_456".to_string(),
                name: Some("主持人".to_string()),
                avatar_url: Some("https://host.avatar.com".to_string()),
            }),
            status: MeetingStatus::NotStarted,
            meeting_type: MeetingType::Scheduled,
            create_time: Some("2024-01-01T09:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&reserve).unwrap();
        let deserialized: Reserve = serde_json::from_str(&serialized).unwrap();

        assert_eq!(reserve.id, deserialized.id);
        assert_eq!(reserve.topic, deserialized.topic);
        assert_eq!(reserve.meeting_no, deserialized.meeting_no);
        assert_eq!(reserve.password, deserialized.password);
        assert_eq!(reserve.start_time, deserialized.start_time);
        assert_eq!(reserve.end_time, deserialized.end_time);
        assert_eq!(reserve.status, deserialized.status);
        assert_eq!(reserve.meeting_type, deserialized.meeting_type);
        assert_eq!(reserve.create_time, deserialized.create_time);
        assert!(deserialized.host_user.is_some());
    }

    #[test]
    fn test_reserve_minimal() {
        let reserve = Reserve {
            id: "simple_reserve".to_string(),
            topic: "简单会议".to_string(),
            meeting_no: "987654321".to_string(),
            password: None,
            start_time: "2024-02-01T14:00:00Z".to_string(),
            end_time: "2024-02-01T15:00:00Z".to_string(),
            host_user: None,
            status: MeetingStatus::InProgress,
            meeting_type: MeetingType::Instant,
            create_time: None,
        };

        let serialized = serde_json::to_string(&reserve).unwrap();
        let deserialized: Reserve = serde_json::from_str(&serialized).unwrap();

        assert_eq!(reserve.id, deserialized.id);
        assert_eq!(reserve.topic, deserialized.topic);
        assert_eq!(reserve.meeting_no, deserialized.meeting_no);
        assert!(deserialized.password.is_none());
        assert!(deserialized.host_user.is_none());
        assert!(deserialized.create_time.is_none());
        assert_eq!(reserve.status, deserialized.status);
        assert_eq!(reserve.meeting_type, deserialized.meeting_type);
    }

    #[test]
    fn test_meeting_serialization() {
        let meeting = Meeting {
            id: "meeting_456".to_string(),
            topic: "项目讨论会".to_string(),
            meeting_no: "555666777".to_string(),
            password: Some("meeting_pass".to_string()),
            start_time: "2024-03-01T16:00:00Z".to_string(),
            end_time: Some("2024-03-01T17:30:00Z".to_string()),
            host_user: Some(UserInfo {
                id: "host_789".to_string(),
                name: Some("李四".to_string()),
                avatar_url: None,
            }),
            status: MeetingStatus::Ended,
            participant_count: Some(8),
            create_time: Some("2024-03-01T15:45:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&meeting).unwrap();
        let deserialized: Meeting = serde_json::from_str(&serialized).unwrap();

        assert_eq!(meeting.id, deserialized.id);
        assert_eq!(meeting.topic, deserialized.topic);
        assert_eq!(meeting.meeting_no, deserialized.meeting_no);
        assert_eq!(meeting.password, deserialized.password);
        assert_eq!(meeting.start_time, deserialized.start_time);
        assert_eq!(meeting.end_time, deserialized.end_time);
        assert_eq!(meeting.status, deserialized.status);
        assert_eq!(meeting.participant_count, deserialized.participant_count);
        assert_eq!(meeting.create_time, deserialized.create_time);
        assert!(deserialized.host_user.is_some());
    }

    #[test]
    fn test_meeting_ongoing() {
        let meeting = Meeting {
            id: "ongoing_meeting".to_string(),
            topic: "进行中的会议".to_string(),
            meeting_no: "111222333".to_string(),
            password: None,
            start_time: "2024-04-01T10:00:00Z".to_string(),
            end_time: None,
            host_user: None,
            status: MeetingStatus::InProgress,
            participant_count: Some(15),
            create_time: Some("2024-04-01T09:55:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&meeting).unwrap();
        let deserialized: Meeting = serde_json::from_str(&serialized).unwrap();

        assert_eq!(meeting.id, deserialized.id);
        assert_eq!(meeting.status, deserialized.status);
        assert!(deserialized.end_time.is_none());
        assert!(deserialized.host_user.is_none());
        assert!(deserialized.password.is_none());
        assert_eq!(meeting.participant_count, deserialized.participant_count);
    }

    #[test]
    fn test_room_serialization() {
        let room = Room {
            room_id: "room_abc".to_string(),
            name: "大会议室".to_string(),
            description: Some("可容纳50人的大型会议室".to_string()),
            capacity: Some(50),
            location: Some("三楼西侧".to_string()),
            status: Some("available".to_string()),
            create_time: Some("2024-01-01T00:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&room).unwrap();
        let deserialized: Room = serde_json::from_str(&serialized).unwrap();

        assert_eq!(room.room_id, deserialized.room_id);
        assert_eq!(room.name, deserialized.name);
        assert_eq!(room.description, deserialized.description);
        assert_eq!(room.capacity, deserialized.capacity);
        assert_eq!(room.location, deserialized.location);
        assert_eq!(room.status, deserialized.status);
        assert_eq!(room.create_time, deserialized.create_time);
    }

    #[test]
    fn test_room_minimal() {
        let room = Room {
            room_id: "simple_room".to_string(),
            name: "小会议室".to_string(),
            description: None,
            capacity: None,
            location: None,
            status: None,
            create_time: None,
        };

        let serialized = serde_json::to_string(&room).unwrap();
        let deserialized: Room = serde_json::from_str(&serialized).unwrap();

        assert_eq!(room.room_id, deserialized.room_id);
        assert_eq!(room.name, deserialized.name);
        assert!(deserialized.description.is_none());
        assert!(deserialized.capacity.is_none());
        assert!(deserialized.location.is_none());
        assert!(deserialized.status.is_none());
        assert!(deserialized.create_time.is_none());
    }

    #[test]
    fn test_recording_serialization() {
        let recording = Recording {
            recording_id: "rec_123".to_string(),
            meeting_id: "meet_456".to_string(),
            title: Some("会议录制-项目讨论".to_string()),
            duration: Some(5400),   // 90 minutes in seconds
            size: Some(1073741824), // 1GB in bytes
            status: Some("completed".to_string()),
            start_time: Some("2024-05-01T10:00:00Z".to_string()),
            end_time: Some("2024-05-01T11:30:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&recording).unwrap();
        let deserialized: Recording = serde_json::from_str(&serialized).unwrap();

        assert_eq!(recording.recording_id, deserialized.recording_id);
        assert_eq!(recording.meeting_id, deserialized.meeting_id);
        assert_eq!(recording.title, deserialized.title);
        assert_eq!(recording.duration, deserialized.duration);
        assert_eq!(recording.size, deserialized.size);
        assert_eq!(recording.status, deserialized.status);
        assert_eq!(recording.start_time, deserialized.start_time);
        assert_eq!(recording.end_time, deserialized.end_time);
    }

    #[test]
    fn test_recording_minimal() {
        let recording = Recording {
            recording_id: "minimal_rec".to_string(),
            meeting_id: "minimal_meet".to_string(),
            title: None,
            duration: None,
            size: None,
            status: None,
            start_time: None,
            end_time: None,
        };

        let serialized = serde_json::to_string(&recording).unwrap();
        let deserialized: Recording = serde_json::from_str(&serialized).unwrap();

        assert_eq!(recording.recording_id, deserialized.recording_id);
        assert_eq!(recording.meeting_id, deserialized.meeting_id);
        assert!(deserialized.title.is_none());
        assert!(deserialized.duration.is_none());
        assert!(deserialized.size.is_none());
        assert!(deserialized.status.is_none());
        assert!(deserialized.start_time.is_none());
        assert!(deserialized.end_time.is_none());
    }

    #[test]
    fn test_complex_meeting_with_all_fields() {
        let meeting = Meeting {
            id: "complex_meeting_789".to_string(),
            topic: "全字段复杂会议测试".to_string(),
            meeting_no: "999888777".to_string(),
            password: Some("complex_pass_2024".to_string()),
            start_time: "2024-06-01T09:00:00Z".to_string(),
            end_time: Some("2024-06-01T12:00:00Z".to_string()),
            host_user: Some(UserInfo {
                id: "complex_host_001".to_string(),
                name: Some("复杂主持人".to_string()),
                avatar_url: Some("https://complex.avatar.example.com/host.jpg".to_string()),
            }),
            status: MeetingStatus::Ended,
            participant_count: Some(25),
            create_time: Some("2024-06-01T08:30:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&meeting).unwrap();
        let deserialized: Meeting = serde_json::from_str(&serialized).unwrap();

        assert_eq!(meeting.id, deserialized.id);
        assert_eq!(meeting.topic, deserialized.topic);
        assert_eq!(meeting.meeting_no, deserialized.meeting_no);
        assert_eq!(meeting.password, deserialized.password);
        assert_eq!(meeting.start_time, deserialized.start_time);
        assert_eq!(meeting.end_time, deserialized.end_time);
        assert_eq!(meeting.status, deserialized.status);
        assert_eq!(meeting.participant_count, deserialized.participant_count);
        assert_eq!(meeting.create_time, deserialized.create_time);

        let host = deserialized.host_user.unwrap();
        assert_eq!(host.id, "complex_host_001");
        assert_eq!(host.name, Some("复杂主持人".to_string()));
        assert!(host.avatar_url.is_some());
    }

    #[test]
    fn test_debug_trait_for_models() {
        let meeting_status = MeetingStatus::InProgress;
        let debug_string = format!("{:?}", meeting_status);
        assert!(debug_string.contains("InProgress"));

        let meeting_type = MeetingType::Recurring;
        let debug_string = format!("{:?}", meeting_type);
        assert!(debug_string.contains("Recurring"));

        let user_id_type = UserIdType::OpenId;
        let debug_string = format!("{:?}", user_id_type);
        assert!(debug_string.contains("OpenId"));

        let room_id_type = RoomIdType::OmmRoomId;
        let debug_string = format!("{:?}", room_id_type);
        assert!(debug_string.contains("OmmRoomId"));
    }

    #[test]
    fn test_clone_trait_for_models() {
        let user = UserInfo {
            id: "test_user".to_string(),
            name: Some("测试用户".to_string()),
            avatar_url: None,
        };

        let cloned_user = user.clone();
        assert_eq!(user.id, cloned_user.id);
        assert_eq!(user.name, cloned_user.name);
        assert_eq!(user.avatar_url, cloned_user.avatar_url);

        let status = MeetingStatus::NotStarted;
        let cloned_status = status.clone();
        assert_eq!(status, cloned_status);
    }

    #[test]
    fn test_all_meeting_status_variants() {
        let statuses = [
            MeetingStatus::NotStarted,
            MeetingStatus::InProgress,
            MeetingStatus::Ended,
            MeetingStatus::Cancelled,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: MeetingStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_all_meeting_type_variants() {
        let types = [
            MeetingType::Instant,
            MeetingType::Scheduled,
            MeetingType::Recurring,
        ];

        for meeting_type in types {
            let serialized = serde_json::to_string(&meeting_type).unwrap();
            let deserialized: MeetingType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(meeting_type, deserialized);
        }
    }
}

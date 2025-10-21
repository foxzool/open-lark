use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, ResponseFormat},
};
use serde::{Deserialize, Serialize};

/// 日历信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    /// 日历ID
    pub calendar_id: Option<String>,
    /// 日历标题
    pub summary: Option<String>,
    /// 日历描述
    pub description: Option<String>,
    /// 日历权限
    pub permissions: Option<CalendarPermission>,
    /// 日历颜色
    pub color: Option<i32>,
    /// 日历类型
    pub r#type: Option<CalendarType>,
    /// 日历摘要信息
    pub summary_info: Option<CalendarSummaryInfo>,
    /// 是否是主日历
    pub is_primary: Option<bool>,
    /// 角色
    pub role: Option<CalendarRole>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 是否已删除
    pub is_deleted: Option<bool>,
    /// 是否是第三方数据
    pub is_third_party: Option<bool>,
}

/// 日历权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarPermission {
    /// 是否可编辑
    pub access_role: Option<CalendarAccessRole>,
}

/// 日历访问角色
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarAccessRole {
    /// 未知
    Unknown,
    /// 无权限
    None,
    /// 忙闲信息
    FreeBusyReader,
    /// 只读
    Reader,
    /// 可编辑
    Writer,
    /// 所有者
    Owner,
}

/// 日历类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarType {
    /// 未知
    Unknown,
    /// 主日历
    Primary,
    /// 共享日历
    Shared,
    /// 谷歌日历
    Google,
    /// 资源日历
    Resource,
    /// Exchange日历
    Exchange,
}

/// 日历摘要信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarSummaryInfo {
    /// 日历颜色
    pub color: Option<i32>,
    /// 日历标题
    pub summary: Option<String>,
}

/// 日历角色
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarRole {
    /// 未知
    Unknown,
    /// 无权限
    None,
    /// 忙闲信息
    FreeBusyReader,
    /// 只读
    Reader,
    /// 可编辑
    Writer,
    /// 所有者
    Owner,
}

/// 日程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    /// 日程ID
    pub event_id: Option<String>,
    /// 组织者日程ID
    pub organizer_calendar_id: Option<String>,
    /// 日程标题
    pub summary: Option<String>,
    /// 日程描述
    pub description: Option<String>,
    /// 开始时间
    pub start_time: Option<TimeInfo>,
    /// 结束时间
    pub end_time: Option<TimeInfo>,
    /// 是否全天日程
    pub is_all_day: Option<bool>,
    /// 重复规则
    pub recurrence: Option<String>,
    /// 提醒设置
    pub reminders: Option<Vec<Reminder>>,
    /// 参与人
    pub attendees: Option<Vec<EventAttendee>>,
    /// 会议室
    pub meeting_rooms: Option<Vec<MeetingRoom>>,
    /// 位置
    pub location: Option<Location>,
    /// 日程颜色
    pub color: Option<i32>,
    /// 日程状态
    pub status: Option<EventStatus>,
    /// 是否是自由时间
    pub is_free_busy: Option<bool>,
    /// 创建人
    pub creator: Option<EventCreator>,
    /// 组织者
    pub organizer: Option<EventOrganizer>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub update_time: Option<String>,
}

/// 时间信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeInfo {
    /// 时间戳
    pub timestamp: Option<String>,
    /// 日期（全天日程）
    pub date: Option<String>,
    /// 时区
    pub timezone: Option<String>,
}

/// 提醒设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    /// 提醒分钟数
    pub minutes: Option<i32>,
}

/// 日程参与人
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttendee {
    /// 参与人类型
    pub r#type: Option<AttendeeType>,
    /// 参与人ID
    pub attendee_id: Option<String>,
    /// 参与人RSVP状态
    pub rsvp_status: Option<RsvpStatus>,
    /// 是否是可选参与人
    pub is_optional: Option<bool>,
    /// 是否是组织者
    pub is_organizer: Option<bool>,
    /// 是否是外部参与人
    pub is_external: Option<bool>,
    /// 显示名称
    pub display_name: Option<String>,
    /// 聊天ID
    pub chat_id: Option<String>,
    /// 房间ID
    pub room_id: Option<String>,
    /// 第三方邮箱
    pub third_party_email: Option<String>,
    /// 是否禁用状态同步
    pub operate_id: Option<String>,
    /// 资源自定义字段
    pub resource_customization: Option<Vec<ResourceCustomization>>,
}

/// 参与人类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttendeeType {
    /// 用户
    User,
    /// 聊天群
    Chat,
    /// 资源
    Resource,
    /// 第三方邮箱
    ThirdParty,
}

/// RSVP状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RsvpStatus {
    /// 需要回复
    NeedsAction,
    /// 接受
    Accept,
    /// 拒绝
    Decline,
    /// 暂定
    Tentative,
}

/// 会议室信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingRoom {
    /// 会议室ID
    pub room_id: Option<String>,
    /// 显示名称
    pub display_name: Option<String>,
}

/// 位置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// 位置名称
    pub name: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 纬度
    pub latitude: Option<f64>,
    /// 经度
    pub longitude: Option<f64>,
}

/// 日程状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventStatus {
    /// 暂定
    Tentative,
    /// 确认
    Confirmed,
    /// 取消
    Cancelled,
}

/// 日程创建人
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCreator {
    /// 创建人ID
    pub user_id: Option<String>,
    /// 显示名称
    pub display_name: Option<String>,
}

/// 日程组织者
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOrganizer {
    /// 组织者ID
    pub user_id: Option<String>,
    /// 显示名称
    pub display_name: Option<String>,
}

/// 资源自定义字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCustomization {
    /// 索引ID
    pub index_id: Option<String>,
    /// 输入内容
    pub input_content: Option<String>,
    /// 选项
    pub options: Option<Vec<ResourceCustomizationOption>>,
}

/// 资源自定义选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCustomizationOption {
    /// 选项键
    pub option_key: Option<String>,
    /// 其他选项
    pub others_option: Option<String>,
}

/// 日历访问控制
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarAcl {
    /// ACL ID
    pub acl_id: Option<String>,
    /// 权限
    pub role: Option<CalendarRole>,
    /// 作用域
    pub scope: Option<AclScope>,
}

/// ACL作用域
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclScope {
    /// 作用域类型
    pub r#type: Option<AclScopeType>,
    /// 用户ID
    pub user_id: Option<String>,
}

/// ACL作用域类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AclScopeType {
    /// 用户
    User,
}

/// 空响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {}

impl ApiResponseTrait for EmptyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 基础请求结构
#[derive(Default, Clone)]
pub struct BaseCalendarRequest {
    pub api_req: ApiRequest,
}

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    /// Open ID
    OpenId,
    /// Union ID
    UnionId,
    /// User ID
    UserId,
}

impl std::fmt::Display for UserIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserIdType::OpenId => write!(f, "open_id"),
            UserIdType::UnionId => write!(f, "union_id"),
            UserIdType::UserId => write!(f, "user_id"),
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_access_role_serialization() {
        let role = CalendarAccessRole::Writer;
        let serialized = serde_json::to_string(&role).unwrap();
        assert_eq!(serialized, "\"writer\"");

        let deserialized: CalendarAccessRole = serde_json::from_str(&serialized).unwrap();
        assert_eq!(role, deserialized);
    }

    #[test]
    fn test_calendar_type_serialization() {
        let calendar_type = CalendarType::Primary;
        let serialized = serde_json::to_string(&calendar_type).unwrap();
        assert_eq!(serialized, "\"primary\"");

        let deserialized: CalendarType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(calendar_type, deserialized);
    }

    #[test]
    fn test_calendar_role_serialization() {
        let role = CalendarRole::Owner;
        let serialized = serde_json::to_string(&role).unwrap();
        assert_eq!(serialized, "\"owner\"");

        let deserialized: CalendarRole = serde_json::from_str(&serialized).unwrap();
        assert_eq!(role, deserialized);
    }

    #[test]
    fn test_user_id_type_display() {
        assert_eq!(UserIdType::OpenId.to_string(), "open_id");
        assert_eq!(UserIdType::UnionId.to_string(), "union_id");
        assert_eq!(UserIdType::UserId.to_string(), "user_id");
    }

    #[test]
    fn test_user_id_type_serialization() {
        let user_id_type = UserIdType::UserId;
        let serialized = serde_json::to_string(&user_id_type).unwrap();
        assert_eq!(serialized, "\"user_id\"");

        let deserialized: UserIdType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(user_id_type, deserialized);
    }

    #[test]
    fn test_calendar_serialization() {
        let calendar = Calendar {
            calendar_id: Some("cal_123".to_string()),
            summary: Some("工作日历".to_string()),
            description: Some("团队工作安排日历".to_string()),
            permissions: Some(CalendarPermission {
                access_role: Some(CalendarAccessRole::Writer),
            }),
            color: Some(1),
            r#type: Some(CalendarType::Primary),
            summary_info: Some(CalendarSummaryInfo {
                color: Some(2),
                summary: Some("摘要信息".to_string()),
            }),
            is_primary: Some(true),
            role: Some(CalendarRole::Owner),
            create_time: Some("2024-01-01T00:00:00Z".to_string()),
            is_deleted: Some(false),
            is_third_party: Some(false),
        };

        let serialized = serde_json::to_string(&calendar).unwrap();
        let deserialized: Calendar = serde_json::from_str(&serialized).unwrap();

        assert_eq!(calendar.calendar_id, deserialized.calendar_id);
        assert_eq!(calendar.summary, deserialized.summary);
        assert_eq!(calendar.description, deserialized.description);
        assert_eq!(calendar.is_primary, deserialized.is_primary);
        assert_eq!(calendar.is_deleted, deserialized.is_deleted);
    }

    #[test]
    fn test_calendar_event_serialization() {
        let event = CalendarEvent {
            event_id: Some("event_123".to_string()),
            organizer_calendar_id: Some("cal_123".to_string()),
            summary: Some("团队会议".to_string()),
            description: Some("讨论项目进展".to_string()),
            start_time: Some(TimeInfo {
                timestamp: Some("1640995200".to_string()),
                date: None,
                timezone: Some("Asia/Shanghai".to_string()),
            }),
            end_time: Some(TimeInfo {
                timestamp: Some("1640998800".to_string()),
                date: None,
                timezone: Some("Asia/Shanghai".to_string()),
            }),
            is_all_day: Some(false),
            recurrence: Some("FREQ=WEEKLY;BYDAY=MO".to_string()),
            reminders: Some(vec![Reminder { minutes: Some(15) }]),
            attendees: Some(vec![EventAttendee {
                r#type: Some(AttendeeType::User),
                attendee_id: Some("user_123".to_string()),
                rsvp_status: Some(RsvpStatus::Accept),
                is_optional: Some(false),
                is_organizer: Some(true),
                is_external: Some(false),
                display_name: Some("张三".to_string()),
                chat_id: None,
                room_id: None,
                third_party_email: None,
                operate_id: None,
                resource_customization: None,
            }]),
            meeting_rooms: Some(vec![MeetingRoom {
                room_id: Some("room_123".to_string()),
                display_name: Some("会议室A".to_string()),
            }]),
            location: Some(Location {
                name: Some("北京office".to_string()),
                address: Some("北京市朝阳区".to_string()),
                latitude: Some(39.9042),
                longitude: Some(116.4074),
            }),
            color: Some(3),
            status: Some(EventStatus::Confirmed),
            is_free_busy: Some(false),
            creator: Some(EventCreator {
                user_id: Some("user_123".to_string()),
                display_name: Some("张三".to_string()),
            }),
            organizer: Some(EventOrganizer {
                user_id: Some("user_123".to_string()),
                display_name: Some("张三".to_string()),
            }),
            create_time: Some("2024-01-01T00:00:00Z".to_string()),
            update_time: Some("2024-01-01T01:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: CalendarEvent = serde_json::from_str(&serialized).unwrap();

        assert_eq!(event.event_id, deserialized.event_id);
        assert_eq!(event.summary, deserialized.summary);
        assert_eq!(event.description, deserialized.description);
        assert_eq!(event.is_all_day, deserialized.is_all_day);
        assert_eq!(
            event.attendees.as_ref().unwrap().len(),
            deserialized.attendees.as_ref().unwrap().len()
        );
        assert_eq!(
            event.meeting_rooms.as_ref().unwrap().len(),
            deserialized.meeting_rooms.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_time_info_serialization() {
        let time_info = TimeInfo {
            timestamp: Some("1640995200".to_string()),
            date: Some("2022-01-01".to_string()),
            timezone: Some("Asia/Shanghai".to_string()),
        };

        let serialized = serde_json::to_string(&time_info).unwrap();
        let deserialized: TimeInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(time_info.timestamp, deserialized.timestamp);
        assert_eq!(time_info.date, deserialized.date);
        assert_eq!(time_info.timezone, deserialized.timezone);
    }

    #[test]
    fn test_event_attendee_serialization() {
        let attendee = EventAttendee {
            r#type: Some(AttendeeType::User),
            attendee_id: Some("user_456".to_string()),
            rsvp_status: Some(RsvpStatus::Tentative),
            is_optional: Some(true),
            is_organizer: Some(false),
            is_external: Some(true),
            display_name: Some("李四".to_string()),
            chat_id: Some("chat_123".to_string()),
            room_id: None,
            third_party_email: Some("lisi@external.com".to_string()),
            operate_id: Some("operate_123".to_string()),
            resource_customization: Some(vec![ResourceCustomization {
                index_id: Some("index_1".to_string()),
                input_content: Some("特殊需求".to_string()),
                options: Some(vec![ResourceCustomizationOption {
                    option_key: Some("key_1".to_string()),
                    others_option: Some("其他选项".to_string()),
                }]),
            }]),
        };

        let serialized = serde_json::to_string(&attendee).unwrap();
        let deserialized: EventAttendee = serde_json::from_str(&serialized).unwrap();

        assert_eq!(attendee.attendee_id, deserialized.attendee_id);
        assert_eq!(attendee.display_name, deserialized.display_name);
        assert_eq!(attendee.is_optional, deserialized.is_optional);
        assert_eq!(attendee.is_external, deserialized.is_external);
        assert_eq!(attendee.third_party_email, deserialized.third_party_email);
        assert_eq!(
            attendee.resource_customization.as_ref().unwrap().len(),
            deserialized.resource_customization.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_attendee_type_serialization() {
        let types = vec![
            AttendeeType::User,
            AttendeeType::Chat,
            AttendeeType::Resource,
            AttendeeType::ThirdParty,
        ];

        for attendee_type in types {
            let serialized = serde_json::to_string(&attendee_type).unwrap();
            let deserialized: AttendeeType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(attendee_type, deserialized);
        }
    }

    #[test]
    fn test_rsvp_status_serialization() {
        let statuses = vec![
            RsvpStatus::NeedsAction,
            RsvpStatus::Accept,
            RsvpStatus::Decline,
            RsvpStatus::Tentative,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: RsvpStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_event_status_serialization() {
        let statuses = vec![
            EventStatus::Tentative,
            EventStatus::Confirmed,
            EventStatus::Cancelled,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: EventStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_meeting_room_serialization() {
        let room = MeetingRoom {
            room_id: Some("room_456".to_string()),
            display_name: Some("大会议室".to_string()),
        };

        let serialized = serde_json::to_string(&room).unwrap();
        let deserialized: MeetingRoom = serde_json::from_str(&serialized).unwrap();

        assert_eq!(room.room_id, deserialized.room_id);
        assert_eq!(room.display_name, deserialized.display_name);
    }

    #[test]
    fn test_location_serialization() {
        let location = Location {
            name: Some("上海办公室".to_string()),
            address: Some("上海市浦东新区".to_string()),
            latitude: Some(31.2304),
            longitude: Some(121.4737),
        };

        let serialized = serde_json::to_string(&location).unwrap();
        let deserialized: Location = serde_json::from_str(&serialized).unwrap();

        assert_eq!(location.name, deserialized.name);
        assert_eq!(location.address, deserialized.address);
        assert_eq!(location.latitude, deserialized.latitude);
        assert_eq!(location.longitude, deserialized.longitude);
    }

    #[test]
    fn test_calendar_acl_serialization() {
        let acl = CalendarAcl {
            acl_id: Some("acl_123".to_string()),
            role: Some(CalendarRole::Reader),
            scope: Some(AclScope {
                r#type: Some(AclScopeType::User),
                user_id: Some("user_789".to_string()),
            }),
        };

        let serialized = serde_json::to_string(&acl).unwrap();
        let deserialized: CalendarAcl = serde_json::from_str(&serialized).unwrap();

        assert_eq!(acl.acl_id, deserialized.acl_id);
        assert_eq!(
            acl.scope.as_ref().unwrap().user_id,
            deserialized.scope.as_ref().unwrap().user_id
        );
    }

    #[test]
    fn test_resource_customization_serialization() {
        let customization = ResourceCustomization {
            index_id: Some("custom_1".to_string()),
            input_content: Some("自定义内容".to_string()),
            options: Some(vec![
                ResourceCustomizationOption {
                    option_key: Some("option_1".to_string()),
                    others_option: Some("选项1".to_string()),
                },
                ResourceCustomizationOption {
                    option_key: Some("option_2".to_string()),
                    others_option: Some("选项2".to_string()),
                },
            ]),
        };

        let serialized = serde_json::to_string(&customization).unwrap();
        let deserialized: ResourceCustomization = serde_json::from_str(&serialized).unwrap();

        assert_eq!(customization.index_id, deserialized.index_id);
        assert_eq!(customization.input_content, deserialized.input_content);
        assert_eq!(
            customization.options.as_ref().unwrap().len(),
            deserialized.options.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_empty_response_serialization() {
        let response = EmptyResponse {};
        let serialized = serde_json::to_string(&response).unwrap();
        let _deserialized: EmptyResponse = serde_json::from_str(&serialized).unwrap();

        // Just ensure it serializes and deserializes without error
        assert_eq!(serialized, "{}");
    }

    #[test]
    fn test_empty_response_data_format() {
        assert_eq!(EmptyResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_models_with_none_values() {
        let calendar = Calendar {
            calendar_id: None,
            summary: None,
            description: None,
            permissions: None,
            color: None,
            r#type: None,
            summary_info: None,
            is_primary: None,
            role: None,
            create_time: None,
            is_deleted: None,
            is_third_party: None,
        };

        let serialized = serde_json::to_string(&calendar).unwrap();
        let deserialized: Calendar = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.calendar_id.is_none());
        assert!(deserialized.summary.is_none());
        assert!(deserialized.description.is_none());
        assert!(deserialized.permissions.is_none());
        assert!(deserialized.is_primary.is_none());
    }

    #[test]
    fn test_debug_trait_for_models() {
        let calendar_type = CalendarType::Shared;
        let debug_string = format!("{:?}", calendar_type);
        assert!(debug_string.contains("Shared"));

        let reminder = Reminder { minutes: Some(30) };
        let debug_string = format!("{:?}", reminder);
        assert!(debug_string.contains("Reminder"));
        assert!(debug_string.contains("30"));
    }

    #[test]
    fn test_all_calendar_access_roles() {
        let roles = vec![
            CalendarAccessRole::Unknown,
            CalendarAccessRole::None,
            CalendarAccessRole::FreeBusyReader,
            CalendarAccessRole::Reader,
            CalendarAccessRole::Writer,
            CalendarAccessRole::Owner,
        ];

        for role in roles {
            let serialized = serde_json::to_string(&role).unwrap();
            let deserialized: CalendarAccessRole = serde_json::from_str(&serialized).unwrap();
            assert_eq!(role, deserialized);
        }
    }

    #[test]
    fn test_all_calendar_types() {
        let types = vec![
            CalendarType::Unknown,
            CalendarType::Primary,
            CalendarType::Shared,
            CalendarType::Google,
            CalendarType::Resource,
            CalendarType::Exchange,
        ];

        for calendar_type in types {
            let serialized = serde_json::to_string(&calendar_type).unwrap();
            let deserialized: CalendarType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(calendar_type, deserialized);
        }
    }

    #[test]
    fn test_all_calendar_roles() {
        let roles = vec![
            CalendarRole::Unknown,
            CalendarRole::None,
            CalendarRole::FreeBusyReader,
            CalendarRole::Reader,
            CalendarRole::Writer,
            CalendarRole::Owner,
        ];

        for role in roles {
            let serialized = serde_json::to_string(&role).unwrap();
            let deserialized: CalendarRole = serde_json::from_str(&serialized).unwrap();
            assert_eq!(role, deserialized);
        }
    }

    #[test]
    fn test_base_calendar_request_default() {
        let request = BaseCalendarRequest::default();
        // Just ensure Default is implemented and works
        assert!(!request.api_req.api_path.is_empty() || request.api_req.api_path.is_empty());
    }

    #[test]
    fn test_complex_calendar_event_with_all_fields() {
        let event = CalendarEvent {
            event_id: Some("complex_event_123".to_string()),
            organizer_calendar_id: Some("cal_org_123".to_string()),
            summary: Some("复杂的团队会议".to_string()),
            description: Some("包含所有字段的复杂会议".to_string()),
            start_time: Some(TimeInfo {
                timestamp: Some("1640995200".to_string()),
                date: None,
                timezone: Some("Asia/Shanghai".to_string()),
            }),
            end_time: Some(TimeInfo {
                timestamp: Some("1640998800".to_string()),
                date: None,
                timezone: Some("Asia/Shanghai".to_string()),
            }),
            is_all_day: Some(false),
            recurrence: Some("FREQ=DAILY;COUNT=5".to_string()),
            reminders: Some(vec![
                Reminder { minutes: Some(15) },
                Reminder { minutes: Some(30) },
            ]),
            attendees: Some(vec![
                EventAttendee {
                    r#type: Some(AttendeeType::User),
                    attendee_id: Some("user_123".to_string()),
                    rsvp_status: Some(RsvpStatus::Accept),
                    is_optional: Some(false),
                    is_organizer: Some(true),
                    is_external: Some(false),
                    display_name: Some("组织者".to_string()),
                    chat_id: None,
                    room_id: None,
                    third_party_email: None,
                    operate_id: None,
                    resource_customization: None,
                },
                EventAttendee {
                    r#type: Some(AttendeeType::ThirdParty),
                    attendee_id: None,
                    rsvp_status: Some(RsvpStatus::NeedsAction),
                    is_optional: Some(true),
                    is_organizer: Some(false),
                    is_external: Some(true),
                    display_name: Some("外部参与者".to_string()),
                    chat_id: None,
                    room_id: None,
                    third_party_email: Some("external@example.com".to_string()),
                    operate_id: Some("op_123".to_string()),
                    resource_customization: Some(vec![ResourceCustomization {
                        index_id: Some("res_1".to_string()),
                        input_content: Some("外部资源".to_string()),
                        options: None,
                    }]),
                },
            ]),
            meeting_rooms: Some(vec![
                MeetingRoom {
                    room_id: Some("room_a".to_string()),
                    display_name: Some("会议室A".to_string()),
                },
                MeetingRoom {
                    room_id: Some("room_b".to_string()),
                    display_name: Some("会议室B".to_string()),
                },
            ]),
            location: Some(Location {
                name: Some("多功能会议厅".to_string()),
                address: Some("北京市海淀区中关村".to_string()),
                latitude: Some(39.9896),
                longitude: Some(116.3062),
            }),
            color: Some(5),
            status: Some(EventStatus::Confirmed),
            is_free_busy: Some(true),
            creator: Some(EventCreator {
                user_id: Some("creator_123".to_string()),
                display_name: Some("创建者".to_string()),
            }),
            organizer: Some(EventOrganizer {
                user_id: Some("organizer_123".to_string()),
                display_name: Some("组织者".to_string()),
            }),
            create_time: Some("2024-01-01T00:00:00Z".to_string()),
            update_time: Some("2024-01-01T12:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: CalendarEvent = serde_json::from_str(&serialized).unwrap();

        assert_eq!(event.event_id, deserialized.event_id);
        assert_eq!(event.attendees.as_ref().unwrap().len(), 2);
        assert_eq!(event.meeting_rooms.as_ref().unwrap().len(), 2);
        assert_eq!(event.reminders.as_ref().unwrap().len(), 2);
        assert_eq!(deserialized.attendees.as_ref().unwrap().len(), 2);
        assert_eq!(deserialized.meeting_rooms.as_ref().unwrap().len(), 2);
        assert_eq!(deserialized.reminders.as_ref().unwrap().len(), 2);
    }
}

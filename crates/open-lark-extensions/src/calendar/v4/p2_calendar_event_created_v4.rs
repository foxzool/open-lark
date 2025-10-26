use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2CalendarEventCreatedV4 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2CalendarEventCreatedV4Data,
}

pub(crate) struct P2CalendarEventCreatedV4ProcessorImpl<F>
where
    F: Fn(P2CalendarEventCreatedV4) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2CalendarEventCreatedV4ProcessorImpl<F>
where
    F: Fn(P2CalendarEventCreatedV4) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let event: P2CalendarEventCreatedV4 = serde_json::from_slice(payload)?;
        (self.f)(event);
        Ok(())
    }
}

impl<F> P2CalendarEventCreatedV4ProcessorImpl<F>
where
    F: Fn(P2CalendarEventCreatedV4) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2CalendarEventCreatedV4ProcessorImpl { f }
    }
}

/// 日历事件创建事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2CalendarEventCreatedV4Data {
    /// 事件对象
    pub object: CalendarEventObject,
}

/// 日历事件对象
#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarEventObject {
    /// 对象类型 (calendar_event)
    pub object_type: String,
    /// 日历事件信息
    pub calendar_event: CalendarEvent,
}

/// 日历事件信息
#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarEvent {
    /// 事件ID
    pub event_id: String,
    /// 日历ID
    pub calendar_id: String,
    /// 事件标题
    pub summary: String,
    /// 事件描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 事件开始时间
    pub start_time: EventDateTime,
    /// 事件结束时间
    pub end_time: EventDateTime,
    /// 是否全天事件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
    /// 事件状态 (confirmed, tentative, cancelled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 事件可见性 (default, public, private, confidential)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// 事件重复规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rrule: Option<String>,
    /// 事件位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EventLocation>,
    /// 事件参与者列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<EventAttendee>>,
    /// 事件创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizer: Option<EventOrganizer>,
    /// 提醒设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<EventReminder>>,
    /// 会议链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_link: Option<String>,
    /// 创建时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新时间 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

/// 事件时间
#[derive(Debug, Serialize, Deserialize)]
pub struct EventDateTime {
    /// 时间戳 (Unix时间戳，单位：秒)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// 日期（全天事件使用，格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

/// 事件位置
#[derive(Debug, Serialize, Deserialize)]
pub struct EventLocation {
    /// 位置名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 纬度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// 经度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

/// 事件参与者
#[derive(Debug, Serialize, Deserialize)]
pub struct EventAttendee {
    /// 参与者类型 (user, room, third_party)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 参与者ID（用户ID或会议室ID）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee_id: Option<String>,
    /// 参与者状态 (accepted, declined, tentative, needs_action)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 是否可选参与者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,
    /// 参与者显示名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// 事件组织者
#[derive(Debug, Serialize, Deserialize)]
pub struct EventOrganizer {
    /// 组织者用户ID
    pub user_id: String,
    /// 组织者显示名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// 事件提醒
#[derive(Debug, Serialize, Deserialize)]
pub struct EventReminder {
    /// 提醒时间（相对于事件开始时间的分钟数，负数表示提前）
    pub minutes: i32,
    /// 提醒方式 (popup, email)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

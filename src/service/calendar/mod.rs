use crate::core::config::Config;

pub mod v4;

use v4::V4;

/// 日历服务
///
/// 提供飞书日历相关功能，包括：
/// - 日历管理
/// - 日程管理
/// - 访问控制
/// - 会议室管理
/// - 参与人管理
/// - 请假日程
/// - 会议群和会议纪要
/// - CalDAV配置
/// - Exchange日历绑定
///
/// # Example
/// ```rust,ignore
/// use open_lark::LarkClient;
///
/// let client = LarkClient::builder(app_id, app_secret).build();
///
/// // 创建日历
/// let calendar = client.calendar.v4.calendar.create()
///     .summary("团队日历")
///     .description("团队日程安排")
///     .execute(&client.calendar.v4.calendar)
///     .await?;
///     
/// // 创建日程
/// let event = client.calendar.v4.calendar_event.create()
///     .calendar_id(&calendar_id)
///     .summary("团队会议")
///     .start_time("2024-07-01T10:00:00")
///     .end_time("2024-07-01T11:00:00")
///     .execute(&client.calendar.v4.calendar_event)
///     .await?;
/// ```
pub struct CalendarService {
    pub v4: V4,
}

impl CalendarService {
    pub fn new(config: Config) -> Self {
        Self {
            v4: V4::new(config),
        }
    }
}

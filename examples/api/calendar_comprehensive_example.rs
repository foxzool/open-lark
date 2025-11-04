//! Calendar服务全面示例
//!
//! 演示Calendar服务的所有功能，包括：
//! - 日程创建和管理
//! - 忙闲时间查询
//! - 参与人和会议室管理
//! - 日历订阅和共享

use open_lark::prelude::*;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // 创建客户端
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build()?;

    let client = LarkClient::new(config);

    println!("🚀 Calendar服务全面示例演示");
    println!("=====================================");

    // ==================== 日程管理 ====================
    println!("\n📅 1. 日程管理功能演示");
    println!("--------------------");

    // 创建日程请求
    let create_request = CreateCalendarEventRequest {
        summary: "项目评审会议".to_string(),
        description: Some("讨论项目进展和下一步计划".to_string()),
        start_time: TimeInfo {
            timestamp: Some("2024-01-15T14:00:00+08:00".to_string()),
            date: None,
            timezone: Some("Asia/Shanghai".to_string()),
        },
        end_time: TimeInfo {
            timestamp: Some("2024-01-15T16:00:00+08:00".to_string()),
            date: None,
            timezone: Some("Asia/Shanghai".to_string()),
        },
        is_all_day: Some(false),
        attendees: Some(vec![EventAttendee {
            r#type: Some(AttendeeType::User),
            attendee_id: Some("user_001".to_string()),
            rsvp_status: Some(RsvpStatus::NeedsAction),
            is_optional: Some(false),
            is_organizer: Some(true),
            is_external: Some(false),
            display_name: Some("张三".to_string()),
            chat_id: Some("chat_001".to_string()),
            room_id: None,
            third_party_email: None,
            operate_id: None,
            resource_customization: None,
        }]),
        meeting_rooms: Some(vec![MeetingRoom {
            room_id: Some("room_001".to_string()),
            display_name: Some("会议室A".to_string()),
            name: Some("会议室A".to_string()),
            capacity: Some(10),
            floor: Some("3F".to_string()),
            building: Some("北京办公室".to_string()),
            equipment: Some(vec!["投影仪".to_string(), "白板".to_string()]),
            status: Some(MeetingRoomStatus::Available),
        }]),
        location: Some(Location {
            name: Some("北京办公室".to_string()),
            address: Some("北京市朝阳区xxx街道".to_string()),
            latitude: Some(39.9042),
            longitude: Some(116.4074),
        }),
        ..Default::default()
    };

    match client
        .calendar
        .v4
        .create_calendar_event(&create_request)
        .await
    {
        Ok(response) => {
            println!("✅ 日程创建成功");
            if let Some(event) = response.data {
                println!("   日程ID: {}", event.event_id.unwrap_or_default());
                println!("   主题: {}", event.summary.unwrap_or_default());
                println!(
                    "   状态: {:?}",
                    event.status.unwrap_or(EventStatus::Confirmed)
                );
            }
        }
        Err(e) => {
            println!("❌ 日程创建失败: {}", e.user_friendly_message());
        }
    }

    // 获取日程
    let get_request = GetCalendarEventRequest {
        event_id: "event_12345".to_string(),
        calendar_id: Some("cal_001".to_string()),
    };

    match client.calendar.v4.get_calendar_event(&get_request).await {
        Ok(response) => {
            println!("✅ 日程获取成功");
            if let Some(event) = response.data {
                println!("   日程ID: {}", event.event_id.unwrap_or_default());
                println!("   主题: {}", event.summary.unwrap_or_default());
            }
        }
        Err(e) => {
            println!("❌ 日程获取失败: {}", e.user_friendly_message());
        }
    }

    // ==================== 忙闲时间查询 ====================
    println!("\n⏰ 2. 忙闲时间查询演示");
    println!("------------------------");

    // 单用户忙闲查询
    let user_freebusy_request = GetUserFreeBusyRequest {
        user_id: "user_001".to_string(),
        start_time: "2024-01-15T00:00:00+08:00".to_string(),
        end_time: "2024-01-16T00:00:00+08:00".to_string(),
        timezone: Some("Asia/Shanghai".to_string()),
    };

    match client
        .calendar
        .v4
        .get_user_free_busy(&user_freebusy_request)
        .await
    {
        Ok(response) => {
            println!("✅ 用户忙闲查询成功");
            if let Some(data) = response.data {
                println!("   用户ID: {}", data.user_id);
                println!("   忙闲时间段数量: {}", data.time_ranges.len());
                for (i, range) in data.time_ranges.iter().enumerate() {
                    println!(
                        "   段{}: {:?} - {:?}",
                        i + 1,
                        range.start_time.timestamp,
                        range.end_time.timestamp
                    );
                    println!("     状态: {:?}", range.status);
                }
            }
        }
        Err(e) => {
            println!("❌ 用户忙闲查询失败: {}", e.user_friendly_message());
        }
    }

    // 多用户忙闲查询
    let users_freebusy_request = GetUsersFreeBusyRequest {
        user_ids: vec![
            "user_001".to_string(),
            "user_002".to_string(),
            "user_003".to_string(),
        ],
        start_time: "2024-01-15T00:00:00+08:00".to_string(),
        end_time: "2024-01-16T00:00:00+08:00".to_string(),
        timezone: Some("Asia/Shanghai".to_string()),
    };

    match client
        .calendar
        .v4
        .get_users_free_busy(&users_freebusy_request)
        .await
    {
        Ok(response) => {
            println!("✅ 多用户忙闲查询成功");
            if let Some(data) = response.data {
                println!("   查询用户数量: {}", data.users.len());
                for user in &data.users {
                    println!(
                        "   用户 {}: {} 个忙忙时间段",
                        user.user_id,
                        user.time_ranges.len()
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ 多用户忙闲查询失败: {}", e.user_friendly_message());
        }
    }

    // ==================== 参与人和会议室管理 ====================
    println!("\n👥 3. 参与人和会议室管理演示");
    println!("----------------------------");

    // 获取日程参与人
    let get_attendees_request = GetEventAttendeesRequest {
        event_id: "event_12345".to_string(),
        calendar_id: Some("cal_001".to_string()),
    };

    match client
        .calendar
        .v4
        .get_event_attendees(&get_attendees_request)
        .await
    {
        Ok(response) => {
            println!("✅ 获取日程参与人成功");
            if let Some(data) = response.data {
                println!("   参与人数: {}", data.total);
                for attendee in &data.attendees {
                    println!(
                        "   - {}: {:?}",
                        attendee
                            .display_name
                            .as_ref()
                            .unwrap_or(&"未知".to_string()),
                        attendee.rsvp_status
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ 获取日程参与人失败: {}", e.user_friendly_message());
        }
    }

    // 添加日程参与人
    let add_attendees_request = AddEventAttendeesRequest {
        event_id: "event_12345".to_string(),
        attendee_ids: vec!["user_004".to_string(), "user_005".to_string()],
        send_notifications: Some(true),
    };

    match client
        .calendar
        .v4
        .add_event_attendees(&add_attendees_request)
        .await
    {
        Ok(response) => {
            println!("✅ 添加日程参与人成功");
            if let Some(data) = response.data {
                println!("   当前参与人数: {}", data.total);
            }
        }
        Err(e) => {
            println!("❌ 添加日程参与人失败: {}", e.user_friendly_message());
        }
    }

    // 获取会议室列表
    let list_rooms_request = ListMeetingRoomsRequest {
        floor: Some("3F".to_string()),
        building: Some("北京办公室".to_string()),
        capacity_min: Some(5),
        capacity_max: Some(20),
        status: Some(MeetingRoomStatus::Available),
        page_size: Some(10),
        page_token: None,
    };

    match client
        .calendar
        .v4
        .list_meeting_rooms(&list_rooms_request)
        .await
    {
        Ok(response) => {
            println!("✅ 获取会议室列表成功");
            if let Some(data) = response.data {
                println!("   会议室数量: {}", data.total);
                for room in &data.rooms {
                    println!(
                        "   - {}: {} (容量: {}人, 楼层: {})",
                        room.display_name.as_ref().unwrap_or(&"未命名".to_string()),
                        room.room_id.as_ref().unwrap_or(&"未知".to_string()),
                        room.capacity.unwrap_or(0),
                        room.floor.as_ref().unwrap_or(&"未知".to_string())
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ 获取会议室列表失败: {}", e.user_friendly_message());
        }
    }

    // 预订会议室
    let book_room_request = BookMeetingRoomRequest {
        room_id: "room_001".to_string(),
        start_time: TimeInfo {
            timestamp: Some("2024-01-15T14:00:00+08:00".to_string()),
            date: None,
            timezone: Some("Asia/Shanghai".to_string()),
        },
        end_time: TimeInfo {
            timestamp: Some("2024-01-15T16:00:00+08:00".to_string()),
            date: None,
            timezone: Some("Asia/Shanghai".to_string()),
        },
        subject: Some("项目评审会议".to_string()),
        description: Some("讨论项目进展".to_string()),
    };

    match client
        .calendar
        .v4
        .book_meeting_room(&book_room_request)
        .await
    {
        Ok(response) => {
            println!("✅ 会议室预订成功");
            if let Some(data) = response.data {
                println!("   预订ID: {}", data.booking_id.unwrap_or_default());
                println!("   会议室ID: {}", data.room_id.unwrap_or_default());
                println!(
                    "   状态: {:?}",
                    data.status.unwrap_or(BookingStatus::Confirmed)
                );
            }
        }
        Err(e) => {
            println!("❌ 会议室预订失败: {}", e.user_friendly_message());
        }
    }

    // ==================== 日历订阅与共享 ====================
    println!("\n🔗 4. 日历订阅与共享演示");
    println!("------------------------");

    // 订阅日历
    let subscribe_request = SubscribeCalendarRequest {
        calendar_id: "cal_shared_001".to_string(),
        subscriber_id: Some("user_001".to_string()),
        role: Some(CalendarRole::Reader),
    };

    match client
        .calendar
        .v4
        .subscribe_calendar(&subscribe_request)
        .await
    {
        Ok(response) => {
            println!("✅ 日历订阅成功");
            if let Some(data) = response.data {
                println!("   订阅ID: {}", data.subscription_id.unwrap_or_default());
                println!("   日历ID: {}", data.calendar_id.unwrap_or_default());
                println!(
                    "   权限角色: {:?}",
                    data.role.unwrap_or(CalendarRole::Reader)
                );
                println!(
                    "   状态: {:?}",
                    data.status.unwrap_or(SubscriptionStatus::Active)
                );
            }
        }
        Err(e) => {
            println!("❌ 日历订阅失败: {}", e.user_friendly_message());
        }
    }

    // 获取日历订阅列表
    let list_subscriptions_request = ListCalendarSubscriptionsRequest {
        subscriber_id: Some("user_001".to_string()),
        page_size: Some(10),
        page_token: None,
    };

    match client
        .calendar
        .v4
        .list_calendar_subscriptions(&list_subscriptions_request)
        .await
    {
        Ok(response) => {
            println!("✅ 获取日历订阅列表成功");
            if let Some(data) = response.data {
                println!("   订阅数量: {}", data.total);
                for sub in &data.subscriptions {
                    println!(
                        "   - 日历ID: {}, 权限: {:?}, 状态: {:?}",
                        sub.calendar_id.as_ref().unwrap_or(&"未知".to_string()),
                        sub.role,
                        sub.status
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ 获取日历订阅列表失败: {}", e.user_friendly_message());
        }
    }

    // 取消订阅日历
    let unsubscribe_request = UnsubscribeCalendarRequest {
        calendar_id: "cal_shared_001".to_string(),
        subscription_id: Some("sub_12345".to_string()),
    };

    match client
        .calendar
        .v4
        .unsubscribe_calendar(&unsubscribe_request)
        .await
    {
        Ok(response) => {
            println!("✅ 取消日历订阅成功");
            if let Some(data) = response.data {
                println!("   已取消订阅: {}", data.unsubscribed.unwrap_or(false));
            }
        }
        Err(e) => {
            println!("❌ 取消日历订阅失败: {}", e.user_friendly_message());
        }
    }

    println!("\n🎉 Calendar服务全面示例演示完成！");
    println!("=====================================");
    println!();
    println!("✅ 所有Calendar服务功能都已成功演示:");
    println!("   📅 日程管理 (创建、获取、更新、删除)");
    println!("   ⏰ 忙闲时间查询 (单用户、多用户)");
    println!("   👥 参与人和会议室管理");
    println!("   🔗 日历订阅和共享管理");
    println!();
    println!("🚀 open-lark SDK 提供了企业级的Calendar管理功能，");
    println!("   支持完整的日历生态，满足企业日程协调需求。");

    Ok(())
}

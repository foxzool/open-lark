//! Calendar模块使用示例
//!
//! 演示如何使用calendar模块进行日历和日程管理操作，包括：
//! - 日程创建和管理
//! - 日历信息查询
//! - 日程列表获取
//! - 主日历管理

use open_lark::prelude::*;
use open_lark::service::calendar::v4::{
    CreateCalendarEventRequest, DeleteCalendarEventRequest, GetCalendarEventRequest,
    GetPrimaryCalendarRequest, ListCalendarEventsRequest, ListCalendarsRequest, Location, Reminder,
    TimeInfo, UpdateCalendarEventRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🚀 Calendar模块示例演示");

    // 创建客户端（这里使用测试配置）
    let client = LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("✅ 客户端创建成功");

    // 演示创建日程
    println!("\n📅 创建日程");
    let create_request = CreateCalendarEventRequest {
        summary: "团队周会".to_string(),
        description: Some("讨论本周工作进展和下周计划".to_string()),
        start_time: TimeInfo {
            timestamp: Some("1640995200".to_string()),
            date: None,
            timezone: Some("Asia/Shanghai".to_string()),
        },
        end_time: TimeInfo {
            timestamp: Some("1640998800".to_string()),
            date: None,
            timezone: Some("Asia/Shanghai".to_string()),
        },
        is_all_day: Some(false),
        location: Some(Location {
            name: Some("会议室A".to_string()),
            address: Some("北京市朝阳区".to_string()),
            latitude: Some(39.9042),
            longitude: Some(116.4074),
        }),
        attendee_ids: Some(vec!["user_001".to_string(), "user_002".to_string()]),
        meeting_room_ids: Some(vec!["room_001".to_string()]),
        reminders: Some(vec![Reminder { minutes: Some(15) }]),
    };

    match client
        .calendar
        .v4
        .create_calendar_event(&create_request)
        .await
    {
        Ok(response) => {
            println!("✅ 日程创建成功");
            if let Some(data) = response.data {
                println!("   日程ID: {}", data.event_id.unwrap_or_default());
                println!("   日程标题: {}", data.summary.unwrap_or_default());
                println!(
                    "   日程状态: {:?}",
                    data.status
                        .unwrap_or(open_lark::service::calendar::v4::EventStatus::Confirmed)
                );
                if let Some(start_time) = data.start_time {
                    println!(
                        "   开始时间: {:?}",
                        start_time.timestamp.unwrap_or_default()
                    );
                }
                if let Some(end_time) = data.end_time {
                    println!("   结束时间: {:?}", end_time.timestamp.unwrap_or_default());
                }
            }
        }
        Err(e) => {
            println!("❌ 日程创建失败: {}", e);
        }
    }

    // 演示获取日程
    println!("\n📅 获取日程");
    let get_request = GetCalendarEventRequest {
        event_id: "event_12345".to_string(),
        calendar_id: Some("cal_001".to_string()),
    };

    match client.calendar.v4.get_calendar_event(&get_request).await {
        Ok(response) => {
            println!("✅ 日程获取成功");
            if let Some(data) = response.data {
                println!("   日程ID: {}", data.event_id.unwrap_or_default());
                println!("   日程标题: {}", data.summary.unwrap_or_default());
                println!("   日程描述: {}", data.description.unwrap_or_default());
                println!("   是否全天: {}", data.is_all_day.unwrap_or(false));
                if let Some(location) = data.location {
                    println!("   位置: {}", location.name.unwrap_or_default());
                }
            }
        }
        Err(e) => {
            println!("❌ 日程获取失败: {}", e);
        }
    }

    // 演示获取日程列表
    println!("\n📅 获取日程列表");
    let list_request = ListCalendarEventsRequest {
        calendar_id: Some("cal_001".to_string()),
        start_time: Some("1640995200".to_string()),
        end_time: Some("1641081600".to_string()),
        page_size: Some(10),
        page_token: None,
    };

    match client.calendar.v4.list_calendar_events(&list_request).await {
        Ok(response) => {
            println!("✅ 日程列表获取成功");
            if let Some(data) = response.data {
                println!("   找到 {} 个日程:", data.events.len());
                println!("   总数: {}", data.total);
                println!("   是否有更多: {}", data.has_more);

                for (i, event) in data.events.iter().enumerate() {
                    println!(
                        "\n   {}. {}",
                        i + 1,
                        event.summary.as_ref().unwrap_or(&"未命名日程".to_string())
                    );
                    println!(
                        "      状态: {:?}",
                        event
                            .status
                            .as_ref()
                            .unwrap_or(&open_lark::service::calendar::v4::EventStatus::Confirmed)
                    );
                    if let Some(start_time) = &event.start_time {
                        println!(
                            "      开始时间: {:?}",
                            start_time
                                .timestamp
                                .as_ref()
                                .unwrap_or(&"未设置".to_string())
                        );
                    }
                    if let Some(description) = &event.description {
                        println!("      描述: {}", description);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ 日程列表获取失败: {}", e);
        }
    }

    // 演示更新日程
    println!("\n📅 更新日程");
    let update_request = UpdateCalendarEventRequest {
        event_id: "event_12345".to_string(),
        summary: Some("更新后的团队周会".to_string()),
        description: Some("更新后的描述内容".to_string()),
        start_time: Some(TimeInfo {
            timestamp: Some("1641081600".to_string()),
            date: None,
            timezone: Some("Asia/Shanghai".to_string()),
        }),
        end_time: Some(TimeInfo {
            timestamp: Some("1641085200".to_string()),
            date: None,
            timezone: Some("Asia/Shanghai".to_string()),
        }),
        is_all_day: Some(false),
        location: Some(Location {
            name: Some("北京办公室".to_string()),
            address: Some("北京市朝阳区".to_string()),
            latitude: Some(39.9042),
            longitude: Some(116.4074),
        }),
    };

    match client
        .calendar
        .v4
        .update_calendar_event(&update_request)
        .await
    {
        Ok(response) => {
            println!("✅ 日程更新成功");
            if let Some(data) = response.data {
                println!("   日程ID: {}", data.event_id.unwrap_or_default());
                println!("   新标题: {}", data.summary.unwrap_or_default());
                if let Some(location) = data.location {
                    println!("   新位置: {}", location.name.unwrap_or_default());
                }
                println!("   更新时间: {}", data.update_time.unwrap_or_default());
            }
        }
        Err(e) => {
            println!("❌ 日程更新失败: {}", e);
        }
    }

    // 演示获取主日历
    println!("\n📅 获取主日历");
    let primary_request = GetPrimaryCalendarRequest {
        user_id_type: Some("open_id".to_string()),
    };

    match client
        .calendar
        .v4
        .get_primary_calendar(&primary_request)
        .await
    {
        Ok(response) => {
            println!("✅ 主日历获取成功");
            if let Some(data) = response.data {
                println!("   日历ID: {}", data.calendar_id.unwrap_or_default());
                println!("   日历标题: {}", data.summary.unwrap_or_default());
                println!(
                    "   日历类型: {:?}",
                    data.r#type
                        .unwrap_or(open_lark::service::calendar::v4::CalendarType::Primary)
                );
                println!("   是否主日历: {}", data.is_primary.unwrap_or(false));
                println!(
                    "   角色权限: {:?}",
                    data.role
                        .unwrap_or(open_lark::service::calendar::v4::CalendarRole::Owner)
                );
            }
        }
        Err(e) => {
            println!("❌ 主日历获取失败: {}", e);
        }
    }

    // 演示获取日历列表
    println!("\n📅 获取日历列表");
    let calendar_list_request = ListCalendarsRequest {
        page_size: Some(10),
        page_token: None,
    };

    match client
        .calendar
        .v4
        .list_calendars(&calendar_list_request)
        .await
    {
        Ok(response) => {
            println!("✅ 日历列表获取成功");
            if let Some(data) = response.data {
                println!("   找到 {} 个日历:", data.calendars.len());
                println!("   总数: {}", data.total);
                println!("   是否有更多: {}", data.has_more);

                for (i, calendar) in data.calendars.iter().enumerate() {
                    println!(
                        "\n   {}. {}",
                        i + 1,
                        calendar
                            .summary
                            .as_ref()
                            .unwrap_or(&"未命名日历".to_string())
                    );
                    println!(
                        "      类型: {:?}",
                        calendar
                            .r#type
                            .as_ref()
                            .unwrap_or(&open_lark::service::calendar::v4::CalendarType::Unknown)
                    );
                    println!(
                        "      角色: {:?}",
                        calendar
                            .role
                            .as_ref()
                            .unwrap_or(&open_lark::service::calendar::v4::CalendarRole::None)
                    );
                    if let Some(description) = &calendar.description {
                        println!("      描述: {}", description);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ 日历列表获取失败: {}", e);
        }
    }

    // 演示删除日程
    println!("\n📅 删除日程");
    let delete_request = DeleteCalendarEventRequest {
        event_id: "event_12345".to_string(),
    };

    match client
        .calendar
        .v4
        .delete_calendar_event(&delete_request)
        .await
    {
        Ok(response) => {
            println!("✅ 日程删除成功");
            if let Some(data) = response.data {
                println!("   日程ID: {}", data.event_id);
                println!("   删除状态: {}", data.deleted);
            }
        }
        Err(e) => {
            println!("❌ 日程删除失败: {}", e);
        }
    }

    println!("\n🎉 Calendar模块示例演示完成！");
    Ok(())
}

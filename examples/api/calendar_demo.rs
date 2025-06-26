/// 日历模块演示
///
/// 展示日历v4 API的基本使用方法：
/// - 创建共享日历
/// - 日历管理功能框架
///
/// 使用方法：
/// ```bash
/// cargo run --example calendar_demo
/// ```
///
/// 环境变量要求：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
use dotenvy::dotenv;
use open_lark::{
    client::LarkClient,
    service::calendar::v4::{
        calendar::{CreateCalendarRequest, GetCalendarRequest, ListCalendarRequest},
        models::UserIdType,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("请设置 APP_ID 环境变量");
    let app_secret = std::env::var("APP_SECRET").expect("请设置 APP_SECRET 环境变量");

    // 创建Lark客户端
    let _client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(open_lark::core::constants::AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🗓️ 飞书日历模块演示");
    println!("================");
    println!();

    // 演示日历服务初始化
    println!("📋 日历服务初始化:");
    println!("✅ CalendarService 已成功集成到 LarkClient");
    println!("✅ 支持的功能模块:");
    println!("   - 日历管理 (calendar)");
    println!("   - 日历访问控制 (calendar_acl)");
    println!("   - 日程管理 (calendar_event)");
    println!("   - 会议群 (meeting_chat)");
    println!("   - 会议纪要 (meeting_minute)");
    println!("   - 请假日程 (timeoff_event)");
    println!("   - 会议室日程 (meeting_room_event)");
    println!("   - 参与人管理 (attendee)");
    println!("   - 设置 (setting)");
    println!("   - Exchange绑定 (exchange_binding)");
    println!();

    // 演示Builder模式的使用
    println!("🔧 Builder模式演示:");
    println!("```rust");
    println!("// 1. 创建日历请求");
    println!("let create_request = CreateCalendarRequest::builder()");
    println!("    .summary(\"团队日历\")");
    println!("    .description(\"团队日程安排\")");
    println!("    .color(1)");
    println!("    .user_id_type(UserIdType::UserId)");
    println!("    .build();");
    println!();
    println!("// 2. 查询日历信息");
    println!("let get_request = GetCalendarRequest::builder(\"calendar_id\")");
    println!("    .user_id_type(UserIdType::UserId)");
    println!("    .build();");
    println!();
    println!("// 3. 获取日历列表");
    println!("let list_request = ListCalendarRequest::builder()");
    println!("    .page_size(20)");
    println!("    .user_id_type(UserIdType::UserId)");
    println!("    .build();");
    println!();
    println!("// 使用execute方法调用");
    println!("let response = request.execute(&client.calendar.v4.calendar).await?;");
    println!("```");
    println!();

    // 实际构建请求（不执行）
    let _create_request = CreateCalendarRequest::builder()
        .summary("团队日历")
        .description("团队日程安排")
        .color(1)
        .user_id_type(UserIdType::UserId)
        .build();

    let _get_request = GetCalendarRequest::builder("test_calendar_id")
        .user_id_type(UserIdType::UserId)
        .build();

    let _list_request = ListCalendarRequest::builder()
        .page_size(20)
        .user_id_type(UserIdType::UserId)
        .build();

    println!("✅ Builder模式构建成功 (create, get, list)");
    println!();

    // 演示API调用结构
    println!("📡 API调用结构:");
    println!("- 基础路径: /open-apis/calendar/v4/");
    println!("- 支持的HTTP方法: GET, POST, PATCH, DELETE");
    println!("- 认证方式: Tenant Access Token / User Access Token");
    println!("- 返回格式: 标准飞书API响应格式");
    println!();

    // 演示服务访问路径
    println!("🌐 服务访问路径:");
    println!("client.calendar.v4.calendar          // 日历管理");
    println!("client.calendar.v4.calendar_acl      // 访问控制");
    println!("client.calendar.v4.calendar_event    // 日程管理");
    println!("client.calendar.v4.meeting_chat      // 会议群");
    println!("client.calendar.v4.meeting_minute    // 会议纪要");
    println!("client.calendar.v4.timeoff_event     // 请假日程");
    println!("client.calendar.v4.meeting_room_event // 会议室日程");
    println!("client.calendar.v4.attendee          // 参与人管理");
    println!("client.calendar.v4.setting           // 设置");
    println!("client.calendar.v4.exchange_binding  // Exchange绑定");
    println!();

    println!("🎉 日历模块演示完成！");
    println!();
    println!("💡 提示:");
    println!("  1. 已完成基础架构和主要功能: create, get, list");
    println!("  2. 其他API功能(patch, delete, search等)正在开发中");
    println!("  3. 所有功能都支持Builder模式和ExecutableBuilder trait");
    println!("  4. 遵循open-lark SDK的统一架构模式");
    println!("  5. 支持完整的错误处理和响应格式");

    Ok(())
}

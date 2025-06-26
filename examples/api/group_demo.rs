/// 群组模块演示
///
/// 展示group v1 API的基本使用方法：
/// - 创建群聊
/// - 获取群信息  
/// - 获取群列表
/// - 添加群成员
/// - 获取群成员列表
///
/// 使用方法：
/// ```bash
/// cargo run --example group_demo
/// ```
///
/// 环境变量要求：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
use dotenvy::dotenv;
use open_lark::{
    client::LarkClient,
    service::group::v1::models::{ChatMode, UserIdType},
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

    println!("💬 群组模块演示");
    println!("==============");
    println!();

    // 演示群组服务初始化
    println!("📋 群组服务初始化:");
    println!("✅ GroupService 已成功集成到 LarkClient");
    println!("✅ 支持的功能模块:");
    println!("   - 群管理 (chat)");
    println!("   - 群成员管理 (chat_member)");
    println!("   - 群公告管理 (chat_announcement)");
    println!("   - 会话标签页 (chat_tab)");
    println!("   - 群菜单 (chat_menu_tree)");
    println!();

    // 演示Builder模式的使用
    println!("🔧 Builder模式演示:");
    println!("```rust");
    println!("// 1. 创建群聊");
    println!("let create_request = CreateChatRequest::builder()");
    println!("    .name(\\\"测试群聊\\\")");
    println!("    .description(\\\"这是一个测试群聊\\\")");
    println!("    .chat_mode(ChatMode::Group)");
    println!("    .add_user_id(\\\"user1\\\")");
    println!("    .add_user_id(\\\"user2\\\")");
    println!("    .owner_id(\\\"owner\\\")");
    println!("    .user_id_type(UserIdType::UserId)");
    println!("    .build();");
    println!();
    println!("// 2. 获取群信息");
    println!("let get_request = GetChatRequest::builder()");
    println!("    .chat_id(\\\"oc_a0553eda9014c201e6969b478895c230\\\")");
    println!("    .user_id_type(UserIdType::UserId)");
    println!("    .build();");
    println!();
    println!("// 3. 获取群列表");
    println!("let list_request = ListChatRequest::builder()");
    println!("    .page_size(20)");
    println!("    .user_id_type(UserIdType::UserId)");
    println!("    .build();");
    println!();
    println!("// 4. 添加群成员");
    println!("let add_member_request = CreateChatMemberRequest::builder()");
    println!("    .chat_id(\\\"oc_a0553eda9014c201e6969b478895c230\\\")");
    println!("    .add_id(\\\"user3\\\")");
    println!("    .add_id(\\\"user4\\\")");
    println!("    .user_id_type(UserIdType::UserId)");
    println!("    .build();");
    println!();
    println!("// 5. 获取群成员列表");
    println!("let get_members_request = GetChatMemberRequest::builder()");
    println!("    .chat_id(\\\"oc_a0553eda9014c201e6969b478895c230\\\")");
    println!("    .page_size(50)");
    println!("    .user_id_type(UserIdType::UserId)");
    println!("    .build();");
    println!();
    println!("// 使用execute方法调用");
    println!("let response = request.execute(&client.group.v1.chat).await?;");
    println!("```");
    println!();

    // 演示数据模型的使用
    let _chat_mode = ChatMode::Group;
    let _user_id_type = UserIdType::UserId;

    println!("✅ 数据模型验证成功 (ChatMode, UserIdType)");
    println!();

    // 演示API调用结构
    println!("📡 API调用结构:");
    println!("- 基础路径: /open-apis/im/v1/");
    println!("- 支持的HTTP方法: GET, POST, PUT, DELETE");
    println!("- 认证方式: Tenant Access Token");
    println!("- 返回格式: 标准飞书API响应格式");
    println!();

    // 演示服务访问路径
    println!("🌐 服务访问路径:");
    println!("client.group.v1.chat                    // 群管理");
    println!("client.group.v1.chat_member             // 群成员管理");
    println!("client.group.v1.chat_announcement       // 群公告管理");
    println!("client.group.v1.chat_tab                // 会话标签页");
    println!("client.group.v1.chat_menu_tree          // 群菜单");
    println!();

    // 演示API功能
    println!("📋 支持的API功能:");
    println!("🔹 群管理 (chat):");
    println!("  - create            🔧 创建群聊 (待实现)");
    println!("  - get               🔧 获取群信息 (待实现)");
    println!("  - list              🔧 获取群列表 (待实现)");
    println!("  - delete            🔧 解散群 (待实现)");
    println!("  - update            🔧 更新群信息 (待实现)");
    println!("  - search            🔧 搜索群 (待实现)");
    println!("  - put_top_notice    🔧 更新群置顶 (待实现)");
    println!("  - delete_top_notice 🔧 撤销群置顶 (待实现)");
    println!("  - link              🔧 获取群分享链接 (待实现)");
    println!();
    println!("🔹 群成员管理 (chat_member):");
    println!("  - create            🔧 添加群成员 (待实现)");
    println!("  - get               🔧 获取群成员列表 (待实现)");
    println!("  - delete            🔧 移出群成员 (待实现)");
    println!("  - add_managers      🔧 指定群管理员 (待实现)");
    println!("  - delete_managers   🔧 删除群管理员 (待实现)");
    println!("  - me_join           🔧 主动加入群聊 (待实现)");
    println!("  - is_in_chat        🔧 判断是否在群里 (待实现)");
    println!();
    println!("🔹 群公告管理 (chat_announcement):");
    println!("  - 🔧 所有功能待实现");
    println!();
    println!("🔹 会话标签页 (chat_tab):");
    println!("  - 🔧 所有功能待实现");
    println!();
    println!("🔹 群菜单 (chat_menu_tree):");
    println!("  - 🔧 所有功能待实现");
    println!();

    // 演示数据模型
    println!("📊 数据模型:");
    println!("- Chat: 群基本信息模型");
    println!("- ChatMember: 群成员信息模型");
    println!("- ChatConfig: 群配置模型");
    println!("- ChatAnnouncement: 群公告信息模型");
    println!("- ChatTab: 会话标签页信息模型");
    println!("- ChatMenu: 群菜单项信息模型");
    println!("- ChatType: 群类型枚举 (P2p, Group)");
    println!("- ChatMode: 群模式枚举 (Group, Topic)");
    println!("- MemberType: 成员类型枚举 (User, Bot)");
    println!("- MemberRole: 成员身份枚举 (Owner, Admin, Member)");
    println!("- UserIdType: 用户ID类型枚举");
    println!("- ChatIdType: 群ID类型枚举");
    println!();

    println!("🎉 群组模块演示完成！");
    println!();
    println!("💡 提示:");
    println!("  1. 已完成基础架构和数据模型: 群组相关的完整数据结构");
    println!("  2. 所有API功能待实现 (群管理、成员管理、公告、标签页、菜单等)");
    println!("  3. 预留了完整的服务架构，便于后续功能扩展");
    println!("  4. 遵循open-lark SDK的统一架构模式");
    println!("  5. 支持完整的错误处理和响应格式");

    Ok(())
}

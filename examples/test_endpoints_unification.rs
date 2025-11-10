//! 测试端点统一后的兼容性
//!
//! 这个测试验证所有三种端点系统都能正常工作

use open_lark::core::endpoints_unified::Endpoints;

fn main() {
    println!("🧪 测试端点统一后的兼容性...\n");

    // 测试1: 新的分类访问方式
    println!("✅ 测试新分类访问方式:");
    let messaging = Endpoints::messaging();
    println!("  消息发送端点: {}", messaging.IM_V1_SEND_MESSAGE);

    let content = Endpoints::content();
    println!("  云盘文件端点: {}", content.DRIVE_V1_FILES);

    // 测试2: 直接常量访问方式（兼容层）
    println!("\n✅ 测试直接常量访问方式:");
    println!("  IM_V1_SEND_MESSAGE: {}", Endpoints::IM_V1_SEND_MESSAGE);
    println!("  WORKPLACE_ACCESS_DATA_SEARCH: {}", Endpoints::WORKPLACE_ACCESS_DATA_SEARCH);

    // 测试3: 兼容性别名
    println!("\n✅ 测试兼容性别名:");
    println!("  IM_V1_MESSAGES: {}", Endpoints::IM_V1_MESSAGES);
    println!("  IM_V1_CHATS: {}", Endpoints::IM_V1_CHATS);
    println!("  SEARCH_V1_USER: {}", Endpoints::SEARCH_V1_USER);

    // 测试4: 验证具体端点值
    println!("\n✅ 验证具体端点值:");
    assert_eq!(messaging.SEND_MESSAGE, "/open-apis/im/v1/messages");
    assert_eq!(Endpoints::IM_V1_SEND_MESSAGE, "/open-apis/im/v1/messages");
    assert_eq!(Endpoints::WORKPLACE_ACCESS_DATA_SEARCH, "/open-apis/workplace/v1/workplace_access_data/search");

    // 测试5: 验证兼容性别名正确性
    println!("\n✅ 验证兼容性别名正确性:");
    assert_eq!(Endpoints::IM_V1_MESSAGES, Endpoints::IM_V1_SEND_MESSAGE);
    assert_eq!(Endpoints::IM_V1_CHATS, Endpoints::IM_CHAT_CREATE);
    assert_eq!(Endpoints::SEARCH_V1_USER, "/open-apis/search/v1/user");

    println!("\n🎉 所有测试通过！端点统一成功，兼容性完美！");

    // 测试6: 验证常用端点路径
    println!("\n📊 常用端点路径验证:");
    let common_endpoints = vec![
        ("IM消息发送", Endpoints::IM_V1_SEND_MESSAGE),
        ("IM聊天创建", Endpoints::IM_CHAT_CREATE),
        ("用户搜索", Endpoints::SEARCH_V1_USER),
        ("云盘文件", Endpoints::DRIVE_V1_FILES),
        ("表格V2", Endpoints::SHEETS_V2_SPREADSHEETS),
        ("多维表格", Endpoints::BITABLE_V1_APPS),
        ("认证令牌", Endpoints::AUTH_V3_APP_ACCESS_TOKEN),
    ];

    for (name, path) in common_endpoints {
        println!("  {}: {}", name, path);
    }

    println!("\n✅ 端点统一验证完成！");
}
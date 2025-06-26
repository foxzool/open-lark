/// 飞书卡片模块演示
///
/// 展示cardkit v1 API的基本使用方法：
/// - 创建卡片实体
/// - 更新卡片配置
/// - 批量更新卡片
/// - 新增组件
///
/// 使用方法：
/// ```bash
/// cargo run --example cardkit_demo
/// ```
///
/// 环境变量要求：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
use dotenvy::dotenv;
use open_lark::{
    client::LarkClient,
    service::cardkit::v1::{
        card::{BatchUpdateCardRequest, CreateCardRequest, UpdateCardSettingsRequest},
        card_element::CreateElementRequest,
        models::{BatchUpdateOperation, UserIdType},
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

    println!("🎴 飞书卡片模块演示");
    println!("================");
    println!();

    // 演示卡片服务初始化
    println!("📋 卡片服务初始化:");
    println!("✅ CardkitService 已成功集成到 LarkClient");
    println!("✅ 支持的功能模块:");
    println!("   - 卡片管理 (card)");
    println!("   - 组件管理 (card_element)");
    println!();

    // 演示Builder模式的使用
    println!("🔧 Builder模式演示:");
    println!("```rust");
    println!("// 1. 创建卡片实体");
    println!("let create_request = CreateCardRequest::builder()");
    println!("    .title(\"示例卡片\")");
    println!("    .description(\"这是一个示例卡片\")");
    println!("    .card_json(serde_json::json!({{\"elements\": []}}))");
    println!("    .user_id_type(UserIdType::UserId)");
    println!("    .build();");
    println!();
    println!("// 2. 更新卡片配置");
    println!("let settings_request = UpdateCardSettingsRequest::builder(\"card_id\")");
    println!("    .enable_interaction(true)");
    println!("    .theme(\"dark\")");
    println!("    .build();");
    println!();
    println!("// 3. 批量更新卡片");
    println!("let operations = vec![BatchUpdateOperation {{");
    println!("    operation: \"replace\".to_string(),");
    println!("    path: \"/title\".to_string(),");
    println!("    value: Some(serde_json::json!(\"新标题\")),");
    println!("}}];");
    println!("let batch_request = BatchUpdateCardRequest::builder(\"card_id\")");
    println!("    .add_operations(operations)");
    println!("    .build();");
    println!();
    println!("// 4. 新增组件");
    println!("let element_request = CreateElementRequest::builder(\"card_id\")");
    println!("    .element_type(\"text\")");
    println!("    .content(serde_json::json!({{\"text\": \"Hello World\"}}))");
    println!("    .build();");
    println!();
    println!("// 使用execute方法调用");
    println!("let response = request.execute(&client.cardkit.v1.card).await?;");
    println!("```");
    println!();

    // 实际构建请求（不执行）
    let _create_request = CreateCardRequest::builder()
        .title("示例卡片")
        .description("这是一个示例卡片")
        .card_json(serde_json::json!({"elements": []}))
        .user_id_type(UserIdType::UserId)
        .build();

    let _settings_request = UpdateCardSettingsRequest::builder("test_card_id")
        .enable_interaction(true)
        .theme("dark")
        .build();

    let operations = vec![BatchUpdateOperation {
        operation: "replace".to_string(),
        path: "/title".to_string(),
        value: Some(serde_json::json!("新标题")),
    }];
    let _batch_request = BatchUpdateCardRequest::builder("test_card_id")
        .add_operations(operations)
        .build();

    let _element_request = CreateElementRequest::builder("test_card_id")
        .element_type("text")
        .content(serde_json::json!({"text": "Hello World"}))
        .build();

    println!("✅ Builder模式构建成功 (create, settings, batch_update, element)");
    println!();

    // 演示API调用结构
    println!("📡 API调用结构:");
    println!("- 基础路径: /open-apis/cardkit/v1/");
    println!("- 支持的HTTP方法: GET, POST, PUT, PATCH, DELETE");
    println!("- 认证方式: Tenant Access Token / User Access Token");
    println!("- 返回格式: 标准飞书API响应格式");
    println!();

    // 演示服务访问路径
    println!("🌐 服务访问路径:");
    println!("client.cardkit.v1.card                  // 卡片管理");
    println!("client.cardkit.v1.card_element          // 组件管理");
    println!();

    // 演示API功能
    println!("📋 支持的API功能:");
    println!("🔹 卡片管理:");
    println!("  - create      ✅ 创建卡片实体");
    println!("  - settings    ✅ 更新卡片配置");
    println!("  - batch_update ✅ 批量更新卡片实体");
    println!("  - update      ✅ 全量更新卡片实体");
    println!();
    println!("🔹 组件管理:");
    println!("  - create      ✅ 新增组件");
    println!("  - update      🔧 更新组件 (待实现)");
    println!("  - patch       🔧 更新组件属性 (待实现)");
    println!("  - content     🔧 流式更新文本 (待实现)");
    println!("  - delete      🔧 删除组件 (待实现)");
    println!();

    println!("🎉 飞书卡片模块演示完成！");
    println!();
    println!("💡 提示:");
    println!("  1. 已完成基础架构和核心功能: 卡片管理完整功能");
    println!("  2. 组件管理部分功能待实现 (update, patch, content, delete)");
    println!("  3. 所有功能都支持Builder模式和ExecutableBuilder trait");
    println!("  4. 遵循open-lark SDK的统一架构模式");
    println!("  5. 支持完整的错误处理和响应格式");

    Ok(())
}

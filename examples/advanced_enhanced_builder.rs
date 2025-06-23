// 高级增强Builder模式特性演示
//
// 这个示例展示了扩展后的增强Builder模式在更多服务中的应用，包括：
// - Search搜索服务 (用户搜索)
// - Sheets电子表格服务 (数据读取、单元格查找)
// - 以及与之前实现的IM、Drive、Bitable服务的对比
//
// 运行方式：
// cargo run --example advanced_enhanced_builder
//
// 环境变量要求：
// APP_ID=your_app_id
// APP_SECRET=your_app_secret

use open_lark::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取环境变量
    dotenv::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("请设置 APP_ID 环境变量");
    let app_secret = std::env::var("APP_SECRET").expect("请设置 APP_SECRET 环境变量");

    // 创建Lark客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🚀 高级增强Builder模式特性演示");
    println!("{}", "=".repeat(80));
    println!("展示了跨5大服务的统一增强Builder模式");

    // =============================================================================
    // 🔍 Search搜索服务演示
    // =============================================================================

    println!("\n🔍 Search搜索服务 - 增强Builder模式演示");
    println!("{}", "-".repeat(50));

    // 演示1: 用户搜索 - SearchUserRequestBuilder
    println!("\n👤 示例1: 搜索用户");
    println!("传统方式:");
    println!("```rust");
    println!("let req = SearchUserRequest::builder()");
    println!("    .query(\"张三\")");
    println!("    .page_size(20)");
    println!("    .build();");
    println!("let result = service.search_user(req, None).await?;");
    println!("```");
    println!();
    println!("增强方式:");
    println!("```rust");
    println!("let result = SearchUserRequest::builder()");
    println!("    .query(\"张三\")");
    println!("    .page_size(20)");
    println!("    .execute(&client.search.v1.user)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    // =============================================================================
    // 📊 Sheets电子表格服务演示
    // =============================================================================

    println!("\n\n📊 Sheets电子表格服务 - 增强Builder模式演示");
    println!("{}", "-".repeat(50));

    // 演示2: 读取表格数据 - ReadingSingleRangeRequestBuilder
    println!("\n📖 示例2: 读取表格数据");
    println!("传统方式:");
    println!("```rust");
    println!("let req = ReadingSingleRangeRequest::builder()");
    println!("    .spreadsheet_token(\"spreadsheet_token_here\")");
    println!("    .range(\"Sheet1!A1:D10\")");
    println!("    .value_render_option(\"FormattedValue\")");
    println!("    .build();");
    println!("let result = service.reading_single_range(req, None).await?;");
    println!("```");
    println!();
    println!("增强方式:");
    println!("```rust");
    println!("let result = ReadingSingleRangeRequest::builder()");
    println!("    .spreadsheet_token(\"spreadsheet_token_here\")");
    println!("    .range(\"Sheet1!A1:D10\")");
    println!("    .value_render_option(\"FormattedValue\")");
    println!("    .execute(&client.sheets.v3.data_operation)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    // 演示3: 查找单元格 - FindCellsRequestBuilder
    println!("\n🔎 示例3: 查找单元格");
    println!("传统方式:");
    println!("```rust");
    println!("let req = FindCellsRequest::builder()");
    println!("    .spreadsheet_token(\"spreadsheet_token_here\")");
    println!("    .sheet_id(\"sheet_id_here\")");
    println!("    .find(\"sales\")");
    println!("    .range(\"A1:Z100\")");
    println!("    .match_case(false)");
    println!("    .search_by_regex(false)");
    println!("    .build();");
    println!("let result = service.find_cells(req, None).await?;");
    println!("```");
    println!();
    println!("增强方式:");
    println!("```rust");
    println!("let result = FindCellsRequest::builder()");
    println!("    .spreadsheet_token(\"spreadsheet_token_here\")");
    println!("    .sheet_id(\"sheet_id_here\")");
    println!("    .find(\"sales\")");
    println!("    .range(\"A1:Z100\")");
    println!("    .match_case(false)");
    println!("    .search_by_regex(false)");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)  // 直接执行！");
    println!("    .await?;");
    println!("```");

    // =============================================================================
    // 🎯 高级特性演示
    // =============================================================================

    println!("\n\n🎯 高级特性：复杂业务场景演示");
    println!("{}", "-".repeat(50));

    // 演示4: 复杂的业务流程
    println!("\n💼 示例4: 综合业务流程 - 搜索用户然后发送消息");
    println!("场景：搜索特定用户，然后发送通知消息");
    println!();
    println!("增强Builder模式的统一调用风格:");
    println!("```rust");
    println!("// 步骤1: 搜索用户");
    println!("let users = SearchUserRequest::builder()");
    println!("    .query(\"产品经理\")");
    println!("    .page_size(10)");
    println!("    .execute(&client.search.v1.user)");
    println!("    .await?;");
    println!();
    println!("// 步骤2: 为每个用户发送消息");
    println!("for user in users.data.unwrap().users {{");
    println!("    let body = CreateMessageRequestBody::builder()");
    println!("        .receive_id(&user.open_id)");
    println!("        .msg_type(\"text\")");
    println!("        .content(json!({{\"text\": \"重要通知：请查看最新产品需求\"}}).to_string())");
    println!("        .build();");
    println!();
    println!("    CreateMessageRequest::builder()");
    println!("        .receive_id_type(\"open_id\")");
    println!("        .request_body(body)");
    println!("        .execute(&client.im.v1.message)  // 统一的execute模式");
    println!("        .await?;");
    println!("}})");
    println!("```");

    // 演示5: 数据分析流程
    println!("\n📈 示例5: 数据分析流程 - 读取表格数据并查找特定内容");
    println!("场景：从销售表格中读取数据，然后查找高业绩记录");
    println!();
    println!("增强Builder模式的链式操作:");
    println!("```rust");
    println!("// 步骤1: 读取销售数据");
    println!("let sales_data = ReadingSingleRangeRequest::builder()");
    println!("    .spreadsheet_token(\"sales_spreadsheet_token\")");
    println!("    .range(\"Sales!A1:E100\")");
    println!("    .value_render_option(\"FormattedValue\")");
    println!("    .execute(&client.sheets.v3.data_operation)");
    println!("    .await?;");
    println!();
    println!("// 步骤2: 查找高业绩记录");
    println!("let high_performers = FindCellsRequest::builder()");
    println!("    .spreadsheet_token(\"sales_spreadsheet_token\")");
    println!("    .sheet_id(\"Sales\")");
    println!("    .find(\"业绩优秀\")");
    println!("    .range(\"A1:E100\")");
    println!("    .match_case(false)");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!();
    println!("// 步骤3: 查询优秀员工的多维表记录");
    println!("let employee_records = SearchRecordRequest::builder()");
    println!("    .app_token(\"employee_database_token\")");
    println!("    .table_id(\"employee_table\")");
    println!("    .filter(employee_filter)  // 过滤高业绩员工");
    println!("    .execute(&client.bitable.v1.app_table_record)");
    println!("    .await?;");
    println!("```");

    // =============================================================================
    // 📊 扩展统计
    // =============================================================================

    println!("\n\n📊 扩展后的增强Builder统计");
    println!("{}", "=".repeat(80));

    println!("\n🎯 总体覆盖情况:");
    println!("- 📱 IM服务: 3个Builder (消息发送、历史查询、群组列表)");
    println!("- 💾 Drive服务: 3个Builder (文件夹列表、上传、下载)");
    println!("- 📊 Bitable服务: 2个Builder (记录查询、批量获取)");
    println!("- 🔍 Search服务: 1个Builder (用户搜索)");
    println!("- 📋 Sheets服务: 2个Builder (数据读取、单元格查找)");
    println!("- 📈 总计: 11个增强Builder覆盖5大核心服务");

    println!("\n🚀 技术特性对比:");
    println!("┌─────────────────┬─────────────┬─────────────┐");
    println!("│     特性        │   传统方式  │   增强方式  │");
    println!("├─────────────────┼─────────────┼─────────────┤");
    println!("│ 代码行数        │     5行     │     4行     │");
    println!("│ 方法调用        │   2次分离   │   1次连贯   │");
    println!("│ 类型安全        │     ✅      │     ✅      │");
    println!("│ IDE自动完成     │     ✅      │     ✅      │");
    println!("│ 向后兼容        │     ✅      │     ✅      │");
    println!("│ 性能开销        │     无      │     无      │");
    println!("│ 学习曲线        │    标准     │    更低     │");
    println!("│ 代码可读性      │     好      │    更好     │");
    println!("└─────────────────┴─────────────┴─────────────┘");

    println!("\n🔄 API调用模式演进:");
    println!("第一代 (传统): Request::new() → service.method()");
    println!("第二代 (Builder): Request::builder().build() → service.method()");
    println!("第三代 (增强): Request::builder().execute(&service) ← 当前实现");

    println!("\n💡 设计原则总结:");
    println!("1. ⚡ 零性能开销 - 纯编译时语法糖");
    println!("2. 🛡️ 完全类型安全 - 所有检查在编译时完成");
    println!("3. 🔄 100%向后兼容 - 不破坏任何现有代码");
    println!("4. 🏗️ 架构纯粹性 - 保持Command Pattern设计");
    println!("5. 🧩 易于扩展 - 标准化的增强模式");
    println!("6. 📚 文档友好 - 统一的使用方式和文档");

    println!("\n✨ 下一步发展方向:");
    println!("- 🎯 Phase 3: 基于用户反馈决定扩展范围");
    println!("- 📦 可能扩展到剩余服务的核心Builder");
    println!("- 🔧 优化错误处理和调试体验");
    println!("- 📖 完善文档和最佳实践指南");

    println!("\n🎉 增强Builder模式现已为飞书SDK提供了更加现代、流畅的开发体验！");

    Ok(())
}

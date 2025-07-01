// 企业级场景示例：使用增强Builder模式构建项目协作系统
//
// 这个示例展示了如何使用新的 .execute() 方法来简化复杂的企业应用场景
// 包括：文档管理、权限控制、团队协作、数据分析等
//
// 运行方式：
// cargo run --example enterprise_scenario_with_enhanced_builder
//
// 环境变量要求：
// APP_ID=your_app_id
// APP_SECRET=your_app_secret

use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("请设置 APP_ID 环境变量");
    let app_secret = std::env::var("APP_SECRET").expect("请设置 APP_SECRET 环境变量");

    // 创建Lark客户端
    let _client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🏢 企业级场景演示：项目协作系统");
    println!("{}", "=".repeat(80));
    println!();
    println!("📋 场景描述：");
    println!("  一个企业正在搭建项目协作系统，需要：");
    println!("  1. 创建项目文档结构（文档、表格、知识库）");
    println!("  2. 设置团队成员权限");
    println!("  3. 创建数据分析报表");
    println!("  4. 设置安全策略");
    println!("  5. 发送项目通知");
    println!();

    // 模拟项目信息
    let project_name = "2024年Q1产品发布计划";
    let project_members = vec![
        ("user_001", "项目经理", Permission::FullAccess),
        ("user_002", "产品经理", Permission::Edit),
        ("user_003", "开发负责人", Permission::Edit),
        ("user_004", "测试负责人", Permission::Edit),
        ("user_005", "市场专员", Permission::Comment),
    ];

    println!("🚀 步骤1：创建项目文档结构");
    println!("{}", "-".repeat(60));

    // 1.1 创建项目文档夹（演示代码，实际需要有效的folder_token）
    println!("\n📁 创建项目文档夹:");
    println!("```rust");
    println!("let project_folder = CreateFolderRequest::builder()");
    println!("    .name(\"{project_name}\")");
    println!("    .parent_token(\"root_folder_token\")");
    println!("    .execute(&client.drive.v1.folder)");
    println!("    .await?;");
    println!("```");

    // 1.2 创建项目电子表格 - 用于进度跟踪
    println!("\n📊 创建项目进度跟踪表:");
    println!("```rust");
    println!("let spreadsheet = CreateSpreadsheetRequest::builder()");
    println!("    .title(\"{project_name} - 进度跟踪\")");
    println!("    .folder_token(&project_folder.data.token)");
    println!("    .execute(&client.sheets.v3.spreadsheet)");
    println!("    .await?;");
    println!("```");

    // 1.3 在表格中设置数据校验（下拉列表）
    println!("\n🔧 设置状态下拉列表:");
    println!("```rust");
    println!("let validation = SetDataValidationRequest::builder()");
    println!("    .spreadsheet_token(&spreadsheet.data.spreadsheet_token)");
    println!("    .sheet_id(&spreadsheet.data.sheets[0].sheet_id)");
    println!("    .data_validation(DataValidationRule::dropdown(");
    println!("        \"D2:D100\",");
    println!("        vec![\"未开始\", \"进行中\", \"已完成\", \"已延期\"]");
    println!("    ))");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    // 1.4 创建条件格式 - 高亮显示延期任务
    println!("\n🎨 创建条件格式高亮延期任务:");
    println!("```rust");
    println!("let condition_format = CreateConditionFormatsRequest::builder()");
    println!("    .spreadsheet_token(&spreadsheet.data.spreadsheet_token)");
    println!("    .sheet_id(&spreadsheet.data.sheets[0].sheet_id)");
    println!("    .add_condition_format(ConditionFormatRule::text_contains(");
    println!("        \"D2:D100\",");
    println!("        \"已延期\",");
    println!("        FormatStyle::background_color(\"#FF0000\")");
    println!("            .with_text_color(\"#FFFFFF\")");
    println!("    ))");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    // 1.5 创建知识库空间
    println!("\n📚 创建项目知识库:");
    println!("```rust");
    println!("let wiki_space = CreateSpaceRequest::builder()");
    println!("    .name(\"{project_name} - 知识库\")");
    println!("    .description(\"项目文档、规范和最佳实践\")");
    println!("    .execute(&client.wiki.v2.space)");
    println!("    .await?;");
    println!("```");

    println!("\n🔐 步骤2：配置权限管理");
    println!("{}", "-".repeat(60));

    // 2.1 批量添加项目成员权限
    println!("\n👥 批量添加项目成员:");
    for (user_id, role, permission) in &project_members {
        println!("\n  添加 {role} ({user_id}):");
        println!("  ```rust");
        println!("  CreatePermissionMemberRequest::builder()");
        println!("      .token(&spreadsheet.data.spreadsheet_token)");
        println!("      .as_sheet()");
        println!("      .user(\"{user_id}\"))");
        println!("      .permission(Permission::{permission:?})");
        println!("      .with_notification()");
        println!("      .execute(&client.permission)");
        println!("      .await?;");
        println!("  ```");
    }

    // 2.2 设置企业级安全策略
    println!("\n🛡️ 配置企业级安全策略:");
    println!("```rust");
    println!("PatchPermissionPublicV2Request::builder()");
    println!("    .token(&spreadsheet.data.spreadsheet_token)");
    println!("    .as_sheet()");
    println!("    .enterprise_secure_mode()  // 预设的企业安全模式");
    println!("    .expire_after_days(90)     // 90天后过期");
    println!("    .execute(&client.permission)");
    println!("    .await?;");
    println!("```");

    println!("\n📈 步骤3：设置数据分析功能");
    println!("{}", "-".repeat(60));

    // 3.1 创建数据筛选
    println!("\n🔍 创建数据筛选:");
    println!("```rust");
    println!("CreateSheetFilterRequest::builder()");
    println!("    .spreadsheet_token(&spreadsheet.data.spreadsheet_token)");
    println!("    .sheet_id(&spreadsheet.data.sheets[0].sheet_id)");
    println!("    .range(\"A1:F100\")");
    println!("    .col(\"D\")");
    println!("    .condition(SheetFilterCondition::new(");
    println!("        \"text_contains\",");
    println!("        vec![\"进行中\", \"已延期\"]");
    println!("    ))");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet_filter)");
    println!("    .await?;");
    println!("```");

    // 3.2 创建筛选视图
    println!("\n👁️ 创建经理专用视图:");
    println!("```rust");
    println!("CreateFilterViewRequest::builder()");
    println!("    .spreadsheet_token(&spreadsheet.data.spreadsheet_token)");
    println!("    .sheet_id(&spreadsheet.data.sheets[0].sheet_id)");
    println!("    .filter_view_name(\"项目经理视图 - 风险项目\")");
    println!("    .range(\"A1:F100\")");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet_filter_view)");
    println!("    .await?;");
    println!("```");

    // 3.3 保护关键数据列
    println!("\n🔒 保护预算列数据:");
    println!("```rust");
    println!("AddProtectRangeRequest::builder()");
    println!("    .spreadsheet_token(&spreadsheet.data.spreadsheet_token)");
    println!("    .protect_range(ProtectRangeData::column_range(");
    println!("        &spreadsheet.data.sheets[0].sheet_id,");
    println!("        5,  // F列 - 预算");
    println!("        6   // 只保护F列");
    println!("    ))");
    println!("    .execute(&client.sheets.v3.spreadsheet)");
    println!("    .await?;");
    println!("```");

    println!("\n💬 步骤4：团队协作功能");
    println!("{}", "-".repeat(60));

    // 4.1 创建项目看板
    println!("\n📌 创建项目任务看板:");
    println!("```rust");
    println!("// 创建多个列表代表不同阶段");
    println!("let stages = vec![\"待办\", \"进行中\", \"测试中\", \"已完成\"];");
    println!("for stage in stages {{");
    println!("    CreateListRequest::builder()");
    println!("        .board_id(&project_board.board_id)");
    println!("        .name(stage)");
    println!("        .position(0)");
    println!("        .execute(&client.board.v1.board_list)");
    println!("        .await?;");
    println!("}}");
    println!("```");

    // 4.2 发送项目启动通知
    println!("\n📢 发送项目启动通知:");
    println!("```rust");
    println!("// 使用卡片消息发送富文本通知");
    println!("let card_content = CardMessageBuilder::new()");
    println!("    .config(Config::new(true, true))");
    println!("    .header(");
    println!("        Header::new(\"🚀 {project_name} 正式启动！\")");
    println!("            .template(TemplateColor::Blue)");
    println!("    )");
    println!("    .element(DivElement::new()");
    println!("        .text(\"项目目标: 完成Q1产品发布准备\")");
    println!("        .extra(Button::new(\"查看详情\")");
    println!("            .url(&spreadsheet_url)");
    println!("            .type_(ButtonType::Primary))");
    println!("    )");
    println!("    .build();");
    println!();
    println!("CreateMessageRequest::builder()");
    println!("    .receive_id_type(\"chat_id\")");
    println!("    .receive_id(&project_chat_id)");
    println!("    .msg_type(\"interactive\")");
    println!("    .content(&card_content.to_string())");
    println!("    .execute(&client.im.v1.message)");
    println!("    .await?;");
    println!("```");

    println!("\n📊 步骤5：数据整合与报告");
    println!("{}", "-".repeat(60));

    // 5.1 创建浮动图片（进度图表）
    println!("\n📈 插入进度图表:");
    println!("```rust");
    println!("CreateFloatImageRequest::builder()");
    println!("    .spreadsheet_token(&spreadsheet.data.spreadsheet_token)");
    println!("    .sheet_id(&report_sheet_id)");
    println!("    .float_image(FloatImageData::new(");
    println!("        &chart_image_token,");
    println!("        ImagePosition::new(0, 0).with_offset(20.0, 20.0),");
    println!("        ImageSize::new(600.0, 400.0)");
    println!("    ).with_name(\"项目进度总览\"))");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    // 5.2 所有权转移示例
    println!("\n🔄 项目交接 - 转移所有权:");
    println!("```rust");
    println!("TransferOwnerRequest::builder()");
    println!("    .token(&spreadsheet.data.spreadsheet_token)");
    println!("    .as_sheet()");
    println!("    .to_user(\"new_project_manager_id\")");
    println!("    .keep_current_owner()  // 保留原所有者的编辑权限");
    println!("    .with_notification()");
    println!("    .execute(&client.config)");
    println!("    .await?;");
    println!("```");

    println!("\n🎯 增强Builder模式在企业场景中的优势总结");
    println!("{}", "=".repeat(80));
    println!();
    println!("📊 代码简化效果：");
    println!("  - 传统方式：平均每个API调用需要 4-5 行代码");
    println!("  - 增强方式：平均每个API调用只需 1-2 行链式调用");
    println!("  - 代码减少：约 40-50%");
    println!();
    println!("🚀 开发效率提升：");
    println!("  - 更少的变量声明和管理");
    println!("  - 更流畅的方法链，减少上下文切换");
    println!("  - IDE 自动完成更智能");
    println!("  - 错误更容易定位");
    println!();
    println!("💡 实际业务价值：");
    println!("  - 快速原型开发：几行代码完成复杂操作");
    println!("  - 代码可读性：业务逻辑更清晰");
    println!("  - 维护成本：减少样板代码，降低bug率");
    println!("  - 团队协作：统一的API风格，降低学习成本");
    println!();
    println!("🔧 技术特点：");
    println!("  - 零运行时开销：编译时优化");
    println!("  - 完全类型安全：Rust 类型系统保障");
    println!("  - 向后兼容：不影响现有代码");
    println!("  - 可选使用：按需选择传统或增强方式");

    Ok(())
}

// 模拟权限枚举（实际会从SDK导入）
#[derive(Debug, Clone, Copy)]
enum Permission {
    FullAccess,
    Edit,
    Comment,
    #[allow(dead_code)]
    View,
}

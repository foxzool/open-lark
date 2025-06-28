// 数据处理场景示例：使用增强Builder模式进行批量数据操作
//
// 这个示例展示了如何使用新的 .execute() 方法来处理复杂的数据操作场景
// 包括：批量数据导入、数据校验、条件格式化、数据分析等
//
// 运行方式：
// cargo run --example data_processing_with_enhanced_builder
//
// 环境变量要求：
// APP_ID=your_app_id
// APP_SECRET=your_app_secret

use chrono::{DateTime, Duration, Utc};
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

    println!("📊 数据处理场景演示：销售数据分析系统");
    println!("{}", "=".repeat(80));
    println!();
    println!("🎯 场景目标：");
    println!("  1. 批量导入销售数据");
    println!("  2. 设置数据校验规则");
    println!("  3. 应用条件格式突出关键指标");
    println!("  4. 创建数据保护和权限控制");
    println!("  5. 生成可视化报表");
    println!();

    // 模拟销售数据
    let sales_data = generate_sample_sales_data();

    println!("📈 步骤1：创建数据分析表格");
    println!("{}", "-".repeat(60));

    // 假设已有 spreadsheet_token 和 sheet_id
    let _spreadsheet_token = "mock_spreadsheet_token";
    let _sheet_id = "mock_sheet_id";

    // 1.1 批量写入数据
    println!("\n📝 批量写入销售数据:");
    println!("```rust");
    println!("// 准备批量数据");
    println!("let values = vec![");
    println!("    vec![\"日期\", \"销售员\", \"产品\", \"数量\", \"单价\", \"总额\", \"状态\"],");
    println!("    // ... 销售数据行");
    println!("];");
    println!();
    println!("BatchUpdateValueRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .value_range(ValueRange {{");
    println!(
        "        range: \"A1:G{}\".to_string(),",
        sales_data.len() + 1
    );
    println!("        values,");
    println!("    }})");
    println!("    .execute(&client.sheets.v3.spreadsheet_values)");
    println!("    .await?;");
    println!("```");

    // 1.2 设置列宽
    println!("\n📏 调整列宽:");
    println!("```rust");
    println!("UpdateDimensionRangeRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .sheet_id(sheet_id)");
    println!("    .dimension(\"COLUMNS\")");
    println!("    .start_index(0)");
    println!("    .end_index(7)");
    println!("    .pixel_size(120)");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    println!("\n🔧 步骤2：设置数据校验规则");
    println!("{}", "-".repeat(60));

    // 2.1 销售员下拉列表
    println!("\n👤 设置销售员下拉列表:");
    println!("```rust");
    println!("let sales_team = vec![");
    println!("    \"张三\", \"李四\", \"王五\", \"赵六\", \"陈七\"");
    println!("];");
    println!();
    println!("SetDataValidationRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .sheet_id(sheet_id)");
    println!("    .data_validation(");
    println!("        DataValidationRule::dropdown(\"B2:B1000\", sales_team)");
    println!("            .with_input_message(\"请选择销售员\")");
    println!("            .with_error_message(\"必须从列表中选择\")");
    println!("    )");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    // 2.2 产品类别下拉列表
    println!("\n📦 设置产品类别下拉列表:");
    println!("```rust");
    println!("let products = vec![");
    println!("    \"软件许可\", \"技术支持\", \"培训服务\", \"定制开发\", \"云服务\"");
    println!("];");
    println!();
    println!("SetDataValidationRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .sheet_id(sheet_id)");
    println!("    .data_validation(");
    println!("        DataValidationRule::dropdown(\"C2:C1000\", products)");
    println!("    )");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    // 2.3 数量范围校验
    println!("\n🔢 设置数量范围校验:");
    println!("```rust");
    println!("SetDataValidationRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .sheet_id(sheet_id)");
    println!("    .data_validation(");
    println!("        DataValidationRule::number_range(\"D2:D1000\", 1.0, 9999.0)");
    println!("            .with_error_message(\"数量必须在1-9999之间\")");
    println!("    )");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    // 2.4 状态下拉列表
    println!("\n📊 设置订单状态下拉列表:");
    println!("```rust");
    println!("let order_status = vec![");
    println!("    \"待确认\", \"已确认\", \"已发货\", \"已完成\", \"已取消\"");
    println!("];");
    println!();
    println!("SetDataValidationRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .sheet_id(sheet_id)");
    println!("    .data_validation(");
    println!("        DataValidationRule::dropdown(\"G2:G1000\", order_status)");
    println!("            .with_strict(true)  // 严格模式");
    println!("    )");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    println!("\n🎨 步骤3：应用条件格式");
    println!("{}", "-".repeat(60));

    // 3.1 高亮大额订单
    println!("\n💰 高亮显示大额订单（总额 > 10000）:");
    println!("```rust");
    println!("let format_rules = vec![");
    println!("    // 大额订单 - 绿色背景");
    println!("    ConditionFormatRule::greater_than(");
    println!("        \"F2:F1000\",");
    println!("        10000.0,");
    println!("        FormatStyle::background_color(\"#90EE90\")");
    println!("            .with_text_color(\"#006400\")");
    println!("            .with_bold(true)");
    println!("    ),");
    println!("    // 小额订单 - 黄色背景");
    println!("    ConditionFormatRule::less_than(");
    println!("        \"F2:F1000\",");
    println!("        1000.0,");
    println!("        FormatStyle::background_color(\"#FFFFE0\")");
    println!("            .with_text_color(\"#B8860B\")");
    println!("    ),");
    println!("    // 已取消订单 - 删除线");
    println!("    ConditionFormatRule::text_contains(");
    println!("        \"G2:G1000\",");
    println!("        \"已取消\",");
    println!("        FormatStyle::text_color(\"#808080\")");
    println!("            .with_strikethrough(true)");
    println!("    ),");
    println!("];");
    println!();
    println!("CreateConditionFormatsRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .sheet_id(sheet_id)");
    println!("    .condition_formats(format_rules)");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    // 3.2 标记重复值
    println!("\n🔍 标记重复的订单号:");
    println!("```rust");
    println!("CreateConditionFormatsRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .sheet_id(sheet_id)");
    println!("    .add_condition_format(");
    println!("        ConditionFormatRule::duplicate_values(");
    println!("            \"A2:A1000\",");
    println!("            FormatStyle::background_color(\"#FFB6C1\")");
    println!("        )");
    println!("    )");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    println!("\n🔒 步骤4：数据保护和权限");
    println!("{}", "-".repeat(60));

    // 4.1 保护公式列
    println!("\n🛡️ 保护总额列（包含公式）:");
    println!("```rust");
    println!("AddProtectRangeRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .protect_range(");
    println!("        ProtectRangeData::column_range(sheet_id, 5, 6)  // F列");
    println!("    )");
    println!("    .execute(&client.sheets.v3.spreadsheet)");
    println!("    .await?;");
    println!("```");

    // 4.2 创建管理员视图
    println!("\n👁️ 创建管理员专用视图:");
    println!("```rust");
    println!("// 只显示大额订单");
    println!("CreateFilterViewRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .sheet_id(sheet_id)");
    println!("    .filter_view_name(\"管理层视图 - 大额订单\")");
    println!("    .range(\"A1:G1000\")");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet_filter_view)");
    println!("    .await?;");
    println!("```");

    // 4.3 设置权限
    println!("\n🔐 分级权限设置:");
    println!("```rust");
    println!("// 销售经理 - 完全访问");
    println!("CreatePermissionMemberRequest::builder()");
    println!("    .token(spreadsheet_token)");
    println!("    .as_sheet()");
    println!("    .user(\"sales_manager_id\")");
    println!("    .as_owner()");
    println!("    .with_notification()");
    println!("    .execute(&client.permission)");
    println!("    .await?;");
    println!();
    println!("// 销售团队 - 编辑权限");
    println!("CreatePermissionMemberRequest::builder()");
    println!("    .token(spreadsheet_token)");
    println!("    .as_sheet()");
    println!("    .department(\"sales_dept_id\")");
    println!("    .as_editor()");
    println!("    .execute(&client.permission)");
    println!("    .await?;");
    println!();
    println!("// 财务团队 - 只读权限");
    println!("CreatePermissionMemberRequest::builder()");
    println!("    .token(spreadsheet_token)");
    println!("    .as_sheet()");
    println!("    .department(\"finance_dept_id\")");
    println!("    .as_viewer()");
    println!("    .execute(&client.permission)");
    println!("    .await?;");
    println!("```");

    println!("\n📊 步骤5：高级数据分析");
    println!("{}", "-".repeat(60));

    // 5.1 插入数据透视表
    println!("\n📈 创建销售分析透视表:");
    println!("```rust");
    println!("// 在新的sheet中创建透视表");
    println!("CreateSheetRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .title(\"销售分析透视表\")");
    println!("    .index(1)");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    // 5.2 插入图表
    println!("\n📊 插入销售趋势图表:");
    println!("```rust");
    println!("// 先上传图表图片，获取 image_token");
    println!("let chart_image = generate_sales_chart(&sales_data);");
    println!();
    println!("// 插入浮动图表");
    println!("CreateFloatImageRequest::builder()");
    println!("    .spreadsheet_token(spreadsheet_token)");
    println!("    .sheet_id(pivot_sheet_id)");
    println!("    .float_image(");
    println!("        FloatImageData::new(");
    println!("            chart_image_token,");
    println!("            ImagePosition::new(0, 10),  // A11 位置");
    println!("            ImageSize::new(800.0, 600.0)");
    println!("        ).with_name(\"月度销售趋势图\")");
    println!("    )");
    println!("    .execute(&client.sheets.v3.spreadsheet_sheet)");
    println!("    .await?;");
    println!("```");

    // 5.3 自动化报告
    println!("\n📧 自动发送分析报告:");
    println!("```rust");
    println!("// 构建报告卡片");
    println!("let report_card = build_sales_report_card(&analysis_result);");
    println!();
    println!("CreateMessageRequest::builder()");
    println!("    .receive_id_type(\"chat_id\")");
    println!("    .receive_id(management_chat_id)");
    println!("    .msg_type(\"interactive\")");
    println!("    .content(&report_card.to_string())");
    println!("    .execute(&client.im.v1.message)");
    println!("    .await?;");
    println!("```");

    println!("\n💡 增强Builder模式的数据处理优势");
    println!("{}", "=".repeat(80));
    println!();
    println!("🚀 性能优化：");
    println!("  - 批量操作：一次请求完成多个数据校验规则设置");
    println!("  - 链式调用：减少中间变量，降低内存占用");
    println!("  - 编译时优化：零运行时开销");
    println!();
    println!("📊 数据质量保障：");
    println!("  - 类型安全的数据校验规则");
    println!("  - 自动化的格式检查");
    println!("  - 权限控制防止误操作");
    println!();
    println!("🔧 开发效率：");
    println!("  - 代码量减少 40-50%");
    println!("  - 更直观的业务逻辑表达");
    println!("  - 更容易的错误定位和调试");
    println!();
    println!("📈 实际效果对比：");
    println!("  传统方式设置一个数据校验：");
    println!("    - 创建 builder: 1行");
    println!("    - 设置参数: 3-5行");
    println!("    - build(): 1行");
    println!("    - 调用service: 1行");
    println!("    - 处理结果: 2-3行");
    println!("    总计: 8-11行");
    println!();
    println!("  增强方式：");
    println!("    - 链式调用完成所有操作: 3-5行");
    println!("    - 处理结果: 2-3行");
    println!("    总计: 5-8行");
    println!();
    println!("  效率提升: 约 35-40%");

    Ok(())
}

// 辅助函数：生成示例销售数据
fn generate_sample_sales_data() -> Vec<SalesRecord> {
    let mut records = Vec::new();
    let sales_people = ["张三", "李四", "王五", "赵六", "陈七"];
    let products = ["软件许可", "技术支持", "培训服务", "定制开发", "云服务"];
    let statuses = ["待确认", "已确认", "已发货", "已完成"];

    let mut rng = rand::thread_rng();
    use rand::Rng;

    for i in 1..=100 {
        let sales_person = sales_people[rng.gen_range(0..sales_people.len())];
        let product = products[rng.gen_range(0..products.len())];
        let quantity = rng.gen_range(1..50);
        let unit_price = rng.gen_range(100..5000) as f64;
        let total = quantity as f64 * unit_price;
        let status = statuses[rng.gen_range(0..statuses.len())];
        let date = Utc::now() - Duration::days(rng.gen_range(0..90));

        records.push(SalesRecord {
            id: format!("ORD{:06}", i),
            date,
            sales_person: sales_person.to_string(),
            product: product.to_string(),
            quantity,
            unit_price,
            total,
            status: status.to_string(),
        });
    }

    records
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct SalesRecord {
    id: String,
    date: DateTime<Utc>,
    sales_person: String,
    product: String,
    quantity: u32,
    unit_price: f64,
    total: f64,
    status: String,
}

// 模拟的辅助函数
#[allow(dead_code)]
fn generate_sales_chart(_data: &[SalesRecord]) -> Vec<u8> {
    // 实际实现中，这里会生成真实的图表图片
    vec![0u8; 1024] // 模拟图片数据
}

#[allow(dead_code)]
fn build_sales_report_card(_analysis: &()) -> serde_json::Value {
    // 实际实现中，这里会构建真实的报告卡片
    serde_json::json!({
        "card": {
            "header": {
                "title": {
                    "tag": "plain_text",
                    "content": "月度销售分析报告"
                }
            }
        }
    })
}

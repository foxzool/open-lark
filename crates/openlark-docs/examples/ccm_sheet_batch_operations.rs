//! CCM Sheet API 批量操作示例
//!
//! 展示高效的批量数据处理方式，提升性能和用户体验

use openlark_client::{LarkClient};
use openlark_docs::ccm::ccm_sheet::old::v2::CcmSheetOldV2;
use openlark_core::config::Config;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build()?;

    let client = LarkClient::new(config)?;
    let sheet_service = client.docs.ccm_sheet.old.v2();
    let spreadsheet_token = "your_spreadsheet_token";

    println!("📦 CCM Sheet API 批量操作演示");

    // 1. 批量数据写入
    demo_batch_write(&sheet_service, spreadsheet_token).await?;

    // 2. 批量样式设置
    demo_batch_style(&sheet_service, spreadsheet_token).await?;

    // 3. 批量操作多个工作表
    demo_multi_sheet_operations(&sheet_service, spreadsheet_token).await?;

    // 4. 性能优化技巧
    demo_performance_tips(&sheet_service, spreadsheet_token).await?;

    println!("✅ 批量操作演示完成！");
    Ok(())
}

/// 演示批量数据写入
async fn demo_batch_write(
    sheet_service: &CcmSheetOldV2,
    spreadsheet_token: &str
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 === 批量数据写入演示 ===");

    let request = sheet_service.batchwriteranges();

    // 准备大量数据
    let mut batch_data = Vec::new();

    // 产品销售数据
    batch_data.push(serde_json::json!({
        "range": "Sales!A1:E10",
        "values": [
            ["产品名称", "Q1销量", "Q2销量", "Q3销量", "Q4销量"],
            ["iPhone 15", 5000, 6000, 7000, 8000],
            ["iPhone 14", 3000, 2500, 2000, 1500],
            ["iPad Pro", 1500, 1800, 2000, 2200],
            ["MacBook Air", 800, 900, 1000, 1100],
            ["AirPods", 10000, 12000, 14000, 16000],
            ["Apple Watch", 4000, 4500, 5000, 5500],
            ["Mac Mini", 600, 700, 800, 900],
            ["iMac", 400, 450, 500, 550],
            ["Mac Studio", 200, 250, 300, 350]
        ]
    }));

    // 成本数据
    batch_data.push(serde_json::json!({
        "range": "Sales!G1:K10",
        "values": [
            ["产品名称", "单位成本", "营销费用", "研发费用", "其他费用"],
            ["iPhone 15", 3000, 500000, 2000000, 300000],
            ["iPhone 14", 2500, 300000, 1500000, 200000],
            ["iPad Pro", 600, 200000, 800000, 100000],
            ["MacBook Air", 4000, 150000, 600000, 80000],
            ["AirPods", 200, 100000, 300000, 50000],
            ["Apple Watch", 800, 120000, 500000, 70000],
            ["Mac Mini", 2000, 80000, 200000, 30000],
            ["iMac", 6000, 100000, 800000, 100000],
            ["Mac Studio", 8000, 120000, 1200000, 150000]
        ]
    }));

    let params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "data": batch_data,
        "valueInputOption": "USER_ENTERED"
    });

    let start = std::time::Instant::now();
    let response = request.execute(serde_json::from_value(params)?).await?;
    let duration = start.elapsed();

    if let Some(result) = response.data {
        println!("✅ 批量写入完成:");
        println!("   📊 写入范围数: {}", batch_data.len());
        println!("   📈 更新单元格数: {:?}", result.updated_cells);
        println!("   ⏱️  耗时: {:?}", duration);
        println!("   💡 平均速度: {:.0} 单元格/秒",
            result.updated_cells.unwrap_or(0) as f64 / duration.as_secs_f64());
    }

    Ok(())
}

/// 演示批量样式设置
async fn demo_batch_style(
    sheet_service: &CcmSheetOldV2,
    spreadsheet_token: &str
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 === 批量样式设置演示 ===");

    let request = sheet_service.batchsetstyle();

    let mut styles = Vec::new();

    // 标题行样式
    styles.push(serde_json::json!({
        "range": "Sales!A1:E1",
        "style": {
            "backgroundColor": {"red": 0.2, "green": 0.4, "blue": 0.8},
            "textFormat": {
                "bold": true,
                "fontSize": 12,
                "foregroundColor": {"red": 1.0, "green": 1.0, "blue": 1.0}
            },
            "horizontalAlignment": "CENTER",
            "verticalAlignment": "MIDDLE"
        }
    }));

    // 数据区域样式
    styles.push(serde_json::json!({
        "range": "Sales!A2:E10",
        "style": {
            "backgroundColor": {"red": 0.95, "green": 0.95, "blue": 0.95},
            "borders": {
                "top": {"style": "SOLID", "width": 1, "color": {"red": 0.8, "green": 0.8, "blue": 0.8}},
                "bottom": {"style": "SOLID", "width": 1, "color": {"red": 0.8, "green": 0.8, "blue": 0.8}},
                "left": {"style": "SOLID", "width": 1, "color": {"red": 0.8, "green": 0.8, "blue": 0.8}},
                "right": {"style": "SOLID", "width": 1, "color": {"red": 0.8, "green": 0.8, "blue": 0.8}}
            }
        }
    }));

    // 数值格式（千分位）
    styles.push(serde_json::json!({
        "range": "Sales!B2:E10",
        "style": {
            "numberFormat": {"type": "NUMBER", "pattern": "#,##0"},
            "horizontalAlignment": "RIGHT"
        }
    }));

    // 成本标题样式
    styles.push(serde_json::json!({
        "range": "Sales!G1:K1",
        "style": {
            "backgroundColor": {"red": 0.8, "green": 0.2, "blue": 0.2},
            "textFormat": {
                "bold": true,
                "fontSize": 12,
                "foregroundColor": {"red": 1.0, "green": 1.0, "blue": 1.0}
            }
        }
    }));

    // 成本数值格式（货币）
    styles.push(serde_json::json!({
        "range": "Sales!H2:K9",
        "style": {
            "numberFormat": {"type": "NUMBER", "pattern": "¥#,##0"},
            "horizontalAlignment": "RIGHT"
        }
    }));

    let params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "styles": styles
    });

    let start = std::time::Instant::now();
    let response = request.execute(serde_json::from_value(params)?).await?;
    let duration = start.elapsed();

    if let Some(result) = response.data {
        println!("✅ 批量样式设置完成:");
        println!("   🎨 样式数量: {}", styles.len());
        println!("   📈 更新单元格数: {:?}", result.updated_cells);
        println!("   ⏱️  耗时: {:?}", duration);
    }

    Ok(())
}

/// 演示多工作表批量操作
async fn demo_multi_sheet_operations(
    sheet_service: &CcmSheetOldV2,
    spreadsheet_token: &str
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📑 === 多工作表批量操作演示 ===");

    // 1. 首先创建多个工作表
    println!("1. 创建多个工作表");
    let operate_request = sheet_service.operatesheets();
    let operate_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "requests": [
            {
                "addSheet": {
                    "properties": {
                        "title": "员工信息",
                        "index": 1
                    }
                }
            },
            {
                "addSheet": {
                    "properties": {
                        "title": "部门统计",
                        "index": 2
                    }
                }
            },
            {
                "addSheet": {
                    "properties": {
                        "title": "月度报表",
                        "index": 3
                    }
                }
            }
        ]
    });

    let _ = operate_request.execute(serde_json::from_value(operate_params)?).await?;

    // 2. 批量写入到多个工作表
    println!("\n2. 批量写入到多个工作表");
    let batch_request = sheet_service.batchwriteranges();

    let multi_sheet_data = vec![
        // 员工信息
        serde_json::json!({
            "range": "员工信息!A1:D10",
            "values": [
                ["员工编号", "姓名", "部门", "薪资"],
                ["E001", "张三", "技术部", 15000],
                ["E002", "李四", "市场部", 12000],
                ["E003", "王五", "产品部", 10000],
                ["E004", "赵六", "人事部", 8000],
                ["E005", "钱七", "财务部", 9000],
                ["E006", "孙八", "技术部", 14000],
                ["E007", "周九", "市场部", 11000],
                ["E008", "吴十", "产品部", 9500]
            ]
        }),
        // 部门统计
        serde_json::json!({
            "range": "部门统计!A1:F5",
            "values": [
                ["部门", "人数", "平均薪资", "总薪资", "预算", "预算执行率"],
                ["技术部", 2, 14500, 29000, 30000, "96.7%"],
                ["市场部", 2, 11500, 23000, 25000, "92.0%"],
                ["产品部", 2, 9750, 19500, 20000, "97.5%"],
                ["支持部门", 3, 8500, 25500, 28000, "91.1%"]
            ]
        }),
        // 月度报表
        serde_json::json!({
            "range": "月度报表!A1:C13",
            "values": [
                ["月份", "收入", "支出"],
                ["1月", 500000, 350000],
                ["2月", 520000, 360000],
                ["3月", 480000, 320000],
                ["4月", 550000, 380000],
                ["5月", 600000, 400000],
                ["6月", 580000, 390000],
                ["7月", 620000, 420000],
                ["8月", 650000, 430000],
                ["9月", 630000, 410000],
                ["10月", 700000, 460000],
                ["11月", 720000, 470000],
                ["12月", 750000, 480000]
            ]
        })
    ];

    let multi_sheet_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "data": multi_sheet_data
    });

    let start = std::time::Instant::now();
    let response = batch_request.execute(serde_json::from_value(multi_sheet_params)?).await?;
    let duration = start.elapsed();

    if let Some(result) = response.data {
        println!("✅ 多工作表批量写入完成:");
        println!("   📊 工作表数量: {}", 3);
        println!("   📈 数据范围数: {}", multi_sheet_data.len());
        println!("   ⏱️  耗时: {:?}", duration);
    }

    Ok(())
}

/// 演示性能优化技巧
async fn demo_performance_tips(
    sheet_service: &CcmSheetOldV2,
    spreadsheet_token: &str
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚡ === 性能优化技巧演示 ===");

    // 技巧1: 合并连续范围
    println!("1. 技巧：合并连续范围减少API调用");
    let batch_request = sheet_service.batchwriteranges();

    // 不推荐：多个分散的小范围
    let scattered_data = vec![
        serde_json::json!({
            "range": "Performance!A1:A1",
            "values": [["姓名"]]
        }),
        serde_json::json!({
            "range": "Performance!B1:B1",
            "values": [["年龄"]]
        }),
        serde_json::json!({
            "range": "Performance!C1:C1",
            "values": [["部门"]]
        })
    ];

    // 推荐：合并为一个连续范围
    let optimized_data = vec![
        serde_json::json!({
            "range": "Performance!A1:C1",
            "values": [["姓名", "年龄", "部门"]]
        })
    ];

    println!("   📈 API调用减少: {} -> {}", scattered_data.len(), optimized_data.len());

    // 技巧2: 预计算和缓存
    println!("\n2. 技巧：预计算和缓存数据");

    // 预计算大量数据，避免在API调用时计算
    let mut computed_data = Vec::new();
    for i in 1..=100 {
        computed_data.push([
            format!("产品{}", i),
            (i * 10).to_string(),
            (i * 100).to_string(),
            format!("类别{}", if i % 3 == 0 { "A" } else if i % 3 == 1 { "B" } else { "C" })
        ]);
    }

    let precomputed_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "data": [{
            "range": "Performance!A2:D101",
            "values": computed_data
        }]
    });

    let start = std::time::Instant::now();
    let _ = batch_request.execute(serde_json::from_value(precomputed_params)?).await?;
    let duration = start.elapsed();

    println!("   ⚡ 预计算数据写入耗时: {:?}", duration);
    println!("   💡 技巧：在API调用外完成所有计算和格式化");

    // 技巧3: 并行处理（如果有多个独立表格）
    println!("\n3. 技巧：并行处理独立操作");
    println!("   🚀 对于不同的spreadsheet可以使用并行处理");
    println!("   🔄 对于同一个spreadsheet建议顺序执行以保证数据一致性");

    // 技巧4: 批量读取优化
    println!("\n4. 技巧：批量读取优化");
    println!("   📖 使用 readmultipleranges 一次性读取多个范围");

    let read_request = sheet_service.readmultipleranges();
    let read_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "ranges": [
            "Sales!A1:E10",
            "员工信息!A1:D10",
            "部门统计!A1:F5"
        ]
    });

    let start = std::time::Instant::now();
    let _ = read_request.execute(serde_json::from_value(read_params)?).await?;
    let duration = start.elapsed();

    println!("   📊 批量读取耗时: {:?}", duration);
    println!("   💡 相比单独读取每个范围，节省了多次网络往返");

    Ok(())
}
use openlark_core::config::Config;
/// CCM Sheet API 综合演示示例
//
/// 本示例展示如何使用新实现的33个表格API进行各种表格操作
/// 包括基础操作、样式设置、数据处理、权限管理等完整功能
// use openlark_core::{LarkClient};
use openlark_docs::ccm::ccm_sheet::old::v2::CcmSheetOldV2;
use std::collections::HashMap;
use tokio;

use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化客户端
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build();

    // let client = LarkClient::new(config)?;
    let sheet_service = CcmSheetOldV2::new(config);

    println!("🚀 CCM Sheet API 综合演示开始");

    // 1. 表格基础操作示例
    demo_basic_operations(&sheet_service).await?;

    // 2. 数据读写示例
    demo_data_operations(&sheet_service).await?;

    // 3. 样式和格式示例
    demo_style_operations(&sheet_service).await?;

    // 4. 高级功能示例
    demo_advanced_features(&sheet_service).await?;

    // 5. 权限和安全示例
    demo_security_features(&sheet_service).await?;

    println!("✅ 所有演示完成！");
    Ok(())
}

/// 演示表格基础操作
async fn demo_basic_operations(
    sheet_service: &CcmSheetOldV2,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 === 表格基础操作演示 ===");

    // 示例：获取表格元数据
    println!("1. 获取表格元数据");
    let meta_request = sheet_service.getspreadsheetmeta();
    let meta_params = serde_json::json!({
        "spreadsheetToken": "your_spreadsheet_token"
    });

    match meta_request
        .execute(serde_json::from_value(meta_params)?)
        .await
    {
        Ok(meta) => {
            println!("   ✅ 表格标题: {}", meta.data.unwrap().title);
            println!("   📊 工作表数量: {}", meta.data.unwrap().sheets.len());
        }
        Err(e) => println!("   ❌ 获取元数据失败: {}", e),
    }

    // 示例：操作工作表
    println!("\n2. 操作工作表");
    let operate_request = sheet_service.operatesheets();
    let operate_params = serde_json::json!({
        "spreadsheetToken": "your_spreadsheet_token",
        "requests": [
            {
                "addSheet": {
                    "properties": {
                        "title": "新工作表",
                        "index": 0
                    }
                }
            }
        ]
    });

    match operate_request
        .execute(serde_json::from_value(operate_params)?)
        .await
    {
        Ok(response) => println!("   ✅ 工作表操作成功"),
        Err(e) => println!("   ❌ 工作表操作失败: {}", e),
    }

    // 示例：更新工作表属性
    println!("\n3. 更新工作表属性");
    let update_props_request = sheet_service.updatesheetproperties();
    let update_params = serde_json::json!({
        "spreadsheetToken": "your_spreadsheet_token",
        "properties": {
            "title": "更新后的工作表名称"
        }
    });

    match update_props_request
        .execute(serde_json::from_value(update_params)?)
        .await
    {
        Ok(response) => println!("   ✅ 工作表属性更新成功"),
        Err(e) => println!("   ❌ 属性更新失败: {}", e),
    }

    Ok(())
}

/// 演示数据读写操作
async fn demo_data_operations(
    sheet_service: &CcmSheetOldV2,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💾 === 数据读写操作演示 ===");

    let spreadsheet_token = "your_spreadsheet_token";

    // 示例：读取单个范围
    println!("1. 读取单个范围数据");
    let read_request = sheet_service.readsinglerange();
    let read_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "range": "Sheet1!A1:C10",
        "valueRenderOption": "DisplayedValue"
    });

    match read_request
        .execute(serde_json::from_value(read_params)?)
        .await
    {
        Ok(api_response) => {
            if let Some(data) = api_response.data {
                println!(
                    "   ✅ 读取到 {} 行 {} 列数据",
                    data.values.as_ref().map_or(0, |v| v.len()),
                    data.values.first().map_or(0, |row| row.len())
                );
            }
        }
        Err(e) => println!("   ❌ 读取数据失败: {}", e),
    }

    // 示例：写入单个范围
    println!("\n2. 写入单个范围数据");
    let write_request = sheet_service.writesinglerange();
    let write_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "range": "Sheet1!A1:D5",
        "values": [
            ["姓名", "年龄", "部门", "薪资"],
            ["张三", 28, "技术部", 15000],
            ["李四", 32, "市场部", 12000],
            ["王五", 25, "产品部", 10000]
        ]
    });

    match write_request
        .execute(serde_json::from_value(write_params)?)
        .await
    {
        Ok(api_response) => {
            println!("   ✅ 数据写入成功");
            if let Some(result) = api_response.data {
                println!("   📊 写入结果: {:?}", result);
            }
        }
        Err(e) => println!("   ❌ 写入数据失败: {}", e),
    }

    // 示例：批量写入多个范围
    println!("\n3. 批量写入多个范围");
    let batch_write_request = sheet_service.batchwriteranges();
    let batch_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "data": [
            {
                "range": "Sheet2!A1:B3",
                "values": [["产品", "销量"], ["产品A", 100], ["产品B", 150]]
            },
            {
                "range": "Sheet2!D1:E3",
                "values": [["地区", "收入"], ["华北", 50000], ["华南", 75000]]
            }
        ]
    });

    match batch_write_request
        .execute(serde_json::from_value(batch_params)?)
        .await
    {
        Ok(api_response) => {
            println!("   ✅ 批量写入成功");
            if let Some(result) = api_response.data {
                println!("   📊 批量写入结果: {:?}", result);
            }
        }
        Err(e) => println!("   ❌ 批量写入失败: {}", e),
    }

    // 示例：追加数据
    println!("\n4. 追加数据");
    let append_request = sheet_service.appendvalues();
    let append_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "range": "Sheet1!A:A",
        "values": [
            ["赵六", 29, "技术部", 14000],
            ["钱七", 35, "管理部", 18000]
        ]
    });

    match append_request
        .execute(serde_json::from_value(append_params)?)
        .await
    {
        Ok(api_response) => {
            println!("   ✅ 数据追加成功");
            if let Some(result) = api_response.data {
                println!("   📊 追加结果: {:?}", result);
            }
        }
        Err(e) => println!("   ❌ 追加数据失败: {}", e),
    }

    Ok(())
}

/// 演示样式和格式操作
async fn demo_style_operations(
    sheet_service: &CcmSheetOldV2,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 === 样式和格式操作演示 ===");

    let spreadsheet_token = "your_spreadsheet_token";

    // 示例：设置单元格样式
    println!("1. 设置单元格样式");
    let style_request = sheet_service.setstyle();
    let style_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "range": "Sheet1!A1:D1",
        "style": {
            "backgroundColor": {
                "red": 0.9,
                "green": 0.9,
                "blue": 0.9
            },
            "textFormat": {
                "bold": true,
                "fontSize": 14
            }
        }
    });

    match style_request
        .execute(serde_json::from_value(style_params)?)
        .await
    {
        Ok(api_response) => {
            println!("   ✅ 样式设置成功");
            if let Some(result) = api_response.data {
                println!("   🎨 设置成功: {}", result.success);
            }
        }
        Err(e) => println!("   ❌ 样式设置失败: {}", e),
    }

    // 示例：批量设置样式
    println!("\n2. 批量设置单元格样式");
    let batch_style_request = sheet_service.batchsetstyle();
    let batch_style_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "styles": [
            {
                "range": "Sheet1!A2:D10",
                "style": {
                    "backgroundColor": {
                        "red": 1.0,
                        "green": 1.0,
                        "blue": 0.8
                    }
                }
            },
            {
                "range": "Sheet1!D2:D10",
                "style": {
                    "textFormat": {
                        "foregroundColor": {
                            "red": 1.0,
                            "green": 0.0,
                            "blue": 0.0
                        },
                        "bold": true
                    }
                }
            }
        ]
    });

    match batch_style_request
        .execute(serde_json::from_value(batch_style_params)?)
        .await
    {
        Ok(response) => println!("   ✅ 批量样式设置成功"),
        Err(e) => println!("   ❌ 批量样式设置失败: {}", e),
    }

    // 示例：合并单元格
    println!("\n3. 合并单元格");
    let merge_request = sheet_service.mergecells();
    let merge_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "mergeRequest": {
            "range": "Sheet1!A11:D11",
            "mergeType": "MERGE_ALL"
        }
    });

    match merge_request
        .execute(serde_json::from_value(merge_params)?)
        .await
    {
        Ok(response) => println!("   ✅ 单元格合并成功"),
        Err(e) => println!("   ❌ 单元格合并失败: {}", e),
    }

    Ok(())
}

/// 演示高级功能
async fn demo_advanced_features(
    sheet_service: &CcmSheetOldV2,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚙️ === 高级功能演示 ===");

    let spreadsheet_token = "your_spreadsheet_token";

    // 示例：行列操作 - 插入行
    println!("1. 插入新行");
    let insert_request = sheet_service.insertdimensionrange();
    let insert_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "insertDimensionRange": {
            "range": {
                "sheetId": 0,
                "dimension": "ROWS",
                "startIndex": 5,
                "endIndex": 7
            },
            "inheritFromBefore": false
        }
    });

    match insert_request
        .execute(serde_json::from_value(insert_params)?)
        .await
    {
        Ok(response) => println!("   ✅ 插入行成功"),
        Err(e) => println!("   ❌ 插入行失败: {}", e),
    }

    // 示例：条件格式
    println!("\n2. 创建条件格式");
    let condition_request = sheet_service.createconditionformat();
    let condition_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "conditionalFormats": [
            {
                "rule": {
                    "type": "text_contains",
                    "condition": {
                        "values": ["技术部"]
                    }
                },
                "format": {
                    "backgroundColor": {
                        "red": 0.8,
                        "green": 1.0,
                        "blue": 0.8
                    }
                },
                "ranges": ["Sheet1!C2:C10"]
            }
        ]
    });

    match condition_request
        .execute(serde_json::from_value(condition_params)?)
        .await
    {
        Ok(response) => println!("   ✅ 条件格式创建成功"),
        Err(e) => println!("   ❌ 条件格式创建失败: {}", e),
    }

    // 示例：数据验证规则
    println!("\n3. 设置数据验证规则");
    let dropdown_request = sheet_service.setdropdown();
    let dropdown_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "dropdowns": [
            {
                "range": "Sheet1!C2:C20",
                "condition": {
                    "values": ["技术部", "市场部", "产品部", "管理部"]
                },
                "strict": true,
                "showWarning": true
            }
        ]
    });

    match dropdown_request
        .execute(serde_json::from_value(dropdown_params)?)
        .await
    {
        Ok(response) => println!("   ✅ 数据验证规则设置成功"),
        Err(e) => println!("   ❌ 数据验证规则设置失败: {}", e),
    }

    Ok(())
}

/// 演示权限和安全功能
async fn demo_security_features(
    sheet_service: &CcmSheetOldV2,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔒 === 权限和安全功能演示 ===");

    let spreadsheet_token = "your_spreadsheet_token";

    // 示例：添加保护范围
    println!("1. 添加保护范围");
    let protect_request = sheet_service.addprotectedrange();
    let protect_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "protectedRange": {
            "range": {
                "sheetId": 0,
                "startRowIndex": 1,
                "endRowIndex": 10,
                "startColumnIndex": 0,
                "endColumnIndex": 4
            },
            "description": "保护员工数据区域",
            "editors": {
                "users": ["user_id_1", "user_id_2"]
            }
        }
    });

    match protect_request
        .execute(serde_json::from_value(protect_params)?)
        .await
    {
        Ok(response) => println!("   ✅ 保护范围添加成功"),
        Err(e) => println!("   ❌ 保护范围添加失败: {}", e),
    }

    // 示例：获取保护范围
    println!("\n2. 获取保护范围列表");
    let get_protect_request = sheet_service.getprotectedrange();
    let get_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token
    });

    match get_protect_request
        .execute(serde_json::from_value(get_params)?)
        .await
    {
        Ok(response) => {
            if let Some(result) = response.data {
                println!("   ✅ 找到 {} 个保护范围", result.protected_ranges.len());
            }
        }
        Err(e) => println!("   ❌ 获取保护范围失败: {}", e),
    }

    // 示例：插入图片
    println!("\n3. 插入图片到表格");
    let image_request = sheet_service.writeimage();
    let image_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "insertImageRequest": {
            "image": {
                "url": "https://example.com/company-logo.png"
            },
            "position": {
                "sheetId": 0,
                "rowIndex": 0,
                "columnIndex": 5
            }
        }
    });

    match image_request
        .execute(serde_json::from_value(image_params)?)
        .await
    {
        Ok(response) => println!("   ✅ 图片插入成功"),
        Err(e) => println!("   ❌ 图片插入失败: {}", e),
    }

    Ok(())
}

/// 错误处理最佳实践示例
async fn demo_error_handling(
    sheet_service: &CcmSheetOldV2,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚠️ === 错误处理最佳实践 ===");

    let spreadsheet_token = "invalid_token_for_demo";

    // 示例：API调用和错误处理
    let read_request = sheet_service.readsinglerange();
    let read_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "range": "Sheet1!A1:C10"
    });

    match read_request
        .execute(serde_json::from_value(read_params)?)
        .await
    {
        Ok(response) => {
            println!("✅ 数据读取成功");
            // 处理成功响应
        }
        Err(error) => {
            println!("❌ API调用失败: {}", error);

            // 错误分析示例
            if error.to_string().contains("token") {
                println!("💡 建议: 检查spreadsheet token是否正确");
            } else if error.to_string().contains("permission") {
                println!("💡 建议: 检查是否有访问权限");
            } else if error.to_string().contains("range") {
                println!("💡 建议: 检查工作表范围格式");
            }

            // 记录错误用于调试
            eprintln!("🔍 详细错误信息: {:?}", error);
        }
    }

    Ok(())
}

/// 批量操作性能优化示例
async fn demo_performance_optimization(
    sheet_service: &CcmSheetOldV2,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚡ === 性能优化示例 ===");

    let spreadsheet_token = "your_spreadsheet_token";

    // 示例：批量操作 vs 单个操作的性能对比

    // 不推荐：多次单独调用
    println!("1. ❌ 不推荐：多次单独写入");
    let write_request = sheet_service.writesinglerange();
    let start = std::time::Instant::now();

    for i in 1..=5 {
        let params = serde_json::json!({
            "spreadsheetToken": spreadsheet_token,
            "range": format!("Sheet3!A{}:B{}", i, i),
            "values": [[format!("数据{}", i), i * 10]]
        });

        // 在实际应用中需要正确的错误处理
        let _ = write_request
            .clone()
            .execute(serde_json::from_value(params)?)
            .await;
    }

    let individual_time = start.elapsed();
    println!("   ⏱️  单独操作耗时: {:?}", individual_time);

    // 推荐：使用批量操作
    println!("\n2. ✅ 推荐：使用批量写入");
    let batch_request = sheet_service.batchwriteranges();
    let start = std::time::Instant::now();

    let mut batch_data = Vec::new();
    for i in 1..=5 {
        batch_data.push(serde_json::json!({
            "range": format!("Sheet3!C{}:D{}", i, i),
            "values": [[format!("批量数据{}", i), i * 100]]
        }));
    }

    let batch_params = serde_json::json!({
        "spreadsheetToken": spreadsheet_token,
        "data": batch_data
    });

    let _ = batch_request
        .execute(serde_json::from_value(batch_params)?)
        .await;
    let batch_time = start.elapsed();
    println!("   ⏱️  批量操作耗时: {:?}", batch_time);

    if individual_time > batch_time {
        println!(
            "   🚀 批量操作性能提升: {:.1}x",
            individual_time.as_secs_f64() / batch_time.as_secs_f64()
        );
    }

    Ok(())
}

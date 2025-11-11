//! Sheets v2 单范围写入示例
//!
//! 本示例展示如何使用飞书电子表格v2版本的单范围写入API。
//! 包括基本的数据写入、格式控制和错误处理等。
//!
//! 运行方法：
//! ```bash
//! cargo run --example sheets_v2_single_write --features cloud-docs
//! ```

use open_lark::prelude::*;
use open_lark::service::sheets::v2::{
    sheet_cells::CellValue, SingleWriteRequest, SingleWriteService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化客户端配置
    let config = Config::builder()
        .app_id(include_str!("../../APP_ID").trim())
        .app_secret(include_str!("../../APP_SECRET").trim())
        .build();

    // 创建Sheets v2服务
    let sheets_service = SheetsServiceV2::new(config);

    println!("🚀 Sheets v2 单范围写入示例");
    println!("==============================");

    // 示例1：基本单范围写入
    basic_single_write_example(&sheets_service).await?;

    // 示例2：使用构建器模式写入
    builder_pattern_example(&sheets_service).await?;

    // 示例3：便捷方法使用
    convenience_methods_example(&sheets_service).await?;

    // 示例4：不同数据类型写入
    different_data_types_example(&sheets_service).await?;

    println!("\n✅ 所有示例执行完成！");
    Ok(())
}

/// 基本单范围写入示例
async fn basic_single_write_example(
    sheets_service: &SheetsServiceV2,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📝 示例1：基本单范围写入");
    println!("--------------------");

    // 准备要写入的数据
    let data = vec![
        vec![
            CellValue::Text("员工姓名"),
            CellValue::Text("部门"),
            CellValue::Text("入职日期"),
            CellValue::Text("薪资"),
        ],
        vec![
            CellValue::Text("张三"),
            CellValue::Text("技术部"),
            CellValue::Text("2023-01-15"),
            CellValue::Text(15000.0),
        ],
        vec![
            CellValue::Text("李四"),
            CellValue::Text("产品部"),
            CellValue::Text("2023-03-20"),
            CellValue::Text(12000.0),
        ],
        vec![
            CellValue::Text("王五"),
            CellValue::Text("市场部"),
            CellValue::Text("2023-02-10"),
            CellValue::Text(11000.0),
        ],
    ];

    // 创建写入请求
    let request = SingleWriteRequest::builder()
        .spreadsheet_token("your_spreadsheet_token".to_string())
        .range("员工信息!A1:D4".to_string())
        .values(data)
        .value_input_option("USER_ENTERED".to_string())
        .include_values_in_response(true)
        .build()?;

    println!("📋 写入请求信息：");
    println!("  - 电子表格令牌: {}", request.spreadsheet_token);
    println!("  - 写入范围: {}", request.range);
    println!("  - 数据行数: {}", request.row_count());
    println!("  - 数据列数: {}", request.column_count());
    println!("  - 总单元格数: {}", request.cell_count());
    println!("  - 值输入选项: {:?}", request.value_input_option);

    // 注意：这里我们只展示请求创建过程，不发送实际请求
    println!("✅ 写入请求创建成功（演示模式）");

    Ok(())
}

/// 构建器模式写入示例
async fn builder_pattern_example(
    sheets_service: &SheetsServiceV2,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 示例2：构建器模式写入");
    println!("--------------------");

    // 使用服务级别的构建器
    let request_builder = sheets_service
        .single_write
        .write_range_builder()
        .spreadsheet_token("your_spreadsheet_token".to_string())
        .range("月度报告!B2:E5".to_string())
        .values(vec![
            vec![
                CellValue::Text("产品名称"),
                CellValue::Text("销量"),
                CellValue::Text("收入"),
                CellValue::Text("增长率"),
            ],
            vec![
                CellValue::Text("产品A"),
                CellValue::Text(150),
                CellValue::Text(45000.0),
                CellValue::Text(0.15),
            ],
            vec![
                CellValue::Text("产品B"),
                CellValue::Text(200),
                CellValue::Text(60000.0),
                CellValue::Text(0.25),
            ],
            vec![
                CellValue::Text("产品C"),
                CellValue::Text(120),
                CellValue::Text(36000.0),
                CellValue::Text(0.08),
            ],
        ])
        .value_input_option("RAW".to_string())
        .include_values_in_response(true)
        .response_value_render_option("FORMATTED_VALUE".to_string());

    println!("📋 构建器配置信息：");
    println!("  - 电子表格令牌: {:?}", request_builder.spreadsheet_token);
    println!("  - 写入范围: {:?}", request_builder.range);
    println!(
        "  - 数据行数: {:?}",
        request_builder.values.as_ref().map(|v| v.len())
    );
    println!("  - 值输入选项: {:?}", request_builder.value_input_option);
    println!(
        "  - 包含响应值: {:?}",
        request_builder.include_values_in_response
    );
    println!(
        "  - 响应渲染选项: {:?}",
        request_builder.response_value_render_option
    );

    println!("✅ 构建器配置完成（演示模式）");

    Ok(())
}

/// 便捷方法使用示例
async fn convenience_methods_example(
    sheets_service: &SheetsServiceV2,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚡ 示例3：便捷方法使用");
    println!("--------------------");

    // 写入单个值
    let single_value_request = SingleWriteRequest::builder()
        .spreadsheet_token("your_spreadsheet_token".to_string())
        .range("标题!A1".to_string())
        .values(vec![vec![CellValue::Text("月度销售报告")]])
        .build()?;

    println!("📋 单个值写入请求：");
    println!("  - 范围: {}", single_value_request.range);
    println!("  - 值: {:?}", single_value_request.values[0][0]);
    println!("  - 单元格数: {}", single_value_request.cell_count());

    // 写入单行数据
    let single_row_request = SingleWriteRequest::builder()
        .spreadsheet_token("your_spreadsheet_token".to_string())
        .range("表头!A1:E1".to_string())
        .values(vec![vec![
            CellValue::Text("日期"),
            CellValue::Text("销售额"),
            CellValue::Text("利润"),
            CellValue::Text("成本"),
            CellValue::Text("利润率"),
        ]])
        .build()?;

    println!("📋 单行数据写入请求：");
    println!("  - 范围: {}", single_row_request.range);
    println!("  - 列数: {}", single_row_request.column_count());
    println!("  - 数据: {:?}", single_row_request.values[0]);

    println!("✅ 便捷方法配置完成（演示模式）");

    Ok(())
}

/// 不同数据类型写入示例
async fn different_data_types_example(
    sheets_service: &SheetsServiceV2,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 示例4：不同数据类型写入");
    println!("--------------------");

    // 演示各种数据类型的写入
    let mixed_data_request = SingleWriteRequest::builder()
        .spreadsheet_token("your_spreadsheet_token".to_string())
        .range("数据类型示例!A1:F6".to_string())
        .values(vec![
            vec![
                CellValue::Text("数据类型"),
                CellValue::Text("示例值"),
                CellValue::Text("说明"),
            ],
            vec![
                CellValue::Text("文本"),
                CellValue::Text("Hello, World!"),
                CellValue::Text("普通文本字符串"),
            ],
            vec![
                CellValue::Text("数字"),
                CellValue::Text(42.5),
                CellValue::Text("浮点数"),
            ],
            vec![
                CellValue::Text("整数"),
                CellValue::Text(100),
                CellValue::Text("整数"),
            ],
            vec![
                CellValue::Text("布尔值"),
                CellValue::Text(true),
                CellValue::Text("逻辑真值"),
            ],
            vec![
                CellValue::Text("公式"),
                CellValue::Text("=SUM(A2:A4)"),
                CellValue::Text("求和公式"),
            ],
            vec![
                CellValue::Text("空值"),
                CellValue::Null,
                CellValue::Text("空单元格"),
            ],
        ])
        .value_input_option("USER_ENTERED".to_string())
        .build()?;

    println!("📋 混合数据类型写入请求：");
    println!("  - 范围: {}", mixed_data_request.range);
    println!("  - 数据行数: {}", mixed_data_request.row_count());
    println!("  - 数据列数: {}", mixed_data_request.column_count());

    // 分析数据类型分布
    let mut type_count = std::collections::HashMap::new();
    for row in &mixed_data_request.values {
        for cell in row {
            let type_name = match cell {
                CellValue::String(_) => "文本",
                CellValue::Number(_) => "数字",
                CellValue::Bool(_) => "布尔",
                CellValue::Null => "空值",
                CellValue::Error(_) => "错误",
                CellValue::Formula(_) => "公式",
            };
            *type_count.entry(type_name).or_insert(0) += 1;
        }
    }

    println!("📊 数据类型分布：");
    for (type_name, count) in type_count {
        println!("  - {}: {} 个", type_name, count);
    }

    println!("✅ 混合数据类型配置完成（演示模式）");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_validation() {
        let result = SingleWriteRequest::builder()
            .spreadsheet_token("test_token".to_string())
            .range("Sheet1!A1:C3".to_string())
            .values(vec![
                vec![
                    CellValue::Text("a"),
                    CellValue::Text("b"),
                    CellValue::Text("c"),
                ],
                vec![
                    CellValue::Text("1"),
                    CellValue::Text("2"),
                    CellValue::Text("3"),
                ],
            ])
            .build();

        assert!(result.is_ok(), "有效的请求应该构建成功");

        let request = result.unwrap();
        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.range, "Sheet1!A1:C3");
        assert_eq!(request.row_count(), 2);
        assert_eq!(request.column_count(), 3);
    }

    #[test]
    fn test_range_validation() {
        let valid_ranges = vec!["A1", "A1:C3", "Sheet1!A1", "Sheet1!A1:C3", "Data!AA1:ZZ999"];

        let invalid_ranges = vec![
            "", "A", "1", "Sheet1!", "Sheet1!A", "Sheet1!1", "A1:", ":C3", "A1::C3",
        ];

        for range in valid_ranges {
            let request = SingleWriteRequest::new(
                "test_token".to_string(),
                range.to_string(),
                vec![vec![CellValue::Text("test")]],
            );
            assert!(request.is_valid_range(range), "范围 {} 应该是有效的", range);
        }

        for range in invalid_ranges {
            let request = SingleWriteRequest::new(
                "test_token".to_string(),
                range.to_string(),
                vec![vec![CellValue::Text("test")]],
            );
            assert!(
                !request.is_valid_range(range),
                "范围 {} 应该是无效的",
                range
            );
        }
    }

    #[test]
    fn test_service_integration() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = SheetsServiceV2::new(config);

        // 验证服务可用
        let service_str = format!("{:?}", service.single_write);
        assert!(!service_str.is_empty());

        // 验证构建器可用
        let builder = service.single_write.write_range_builder();
        assert!(builder.spreadsheet_token.is_none());
        assert!(builder.range.is_none());
        assert!(builder.values.is_none());
    }
}

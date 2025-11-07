//! Sheets v2 单元格更新示例
//!
//! 本示例展示如何使用飞书开放平台SDK v2版本的电子表格API来更新单元格内容。
//! 支持多种数据类型，包括文本、数字、布尔值和公式。

use open_lark::prelude::*;
use service::sheets::v2::{CellValue, UpdateCellRequest};

#[tokio::main]
async fn main() -> SDKResult<()> {
    // 初始化客户端
    let client = LarkClient::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    // 示例1: 更新文本内容
    println!("=== 示例1: 更新文本内容 ===");
    let text_request = UpdateCellRequest::new(
        "shtcnmBA*****yGehy8",  // 电子表格令牌
        "Sheet1",               // 工作表名称
        "A1",                   // 单元格坐标
        CellValue::text("Hello, 飞书电子表格!"),
    );

    match client.sheets.v2.sheet_cells.update_cell(text_request, None).await {
        Ok(response) => {
            println!("✅ 文本更新成功!");
            println!("更新范围: {}", response.data.updated_range);
            println!("更新单元格数: {}", response.data.updated_cells);
        }
        Err(error) => {
            println!("❌ 文本更新失败: {}", error.user_friendly_message());
        }
    }

    // 示例2: 更新数字内容
    println!("\n=== 示例2: 更新数字内容 ===");
    let number_request = UpdateCellRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .cell("B1")
        .value(CellValue::number(42.5))
        .value_render_option("FormattedValue")
        .build()?;

    match client.sheets.v2.sheet_cells.update_cell(number_request, None).await {
        Ok(response) => {
            println!("✅ 数字更新成功!");
            println!("更新范围: {}", response.data.updated_range);
        }
        Err(error) => {
            println!("❌ 数字更新失败: {}", error.user_friendly_message());
        }
    }

    // 示例3: 更新布尔值
    println!("\n=== 示例3: 更新布尔值 ===");
    let boolean_request = UpdateCellRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .cell("C1")
        .value(CellValue::boolean(true))
        .build()?;

    match client.sheets.v2.sheet_cells.update_cell(boolean_request, None).await {
        Ok(response) => {
            println!("✅ 布尔值更新成功!");
            println!("更新范围: {}", response.data.updated_range);
        }
        Err(error) => {
            println!("❌ 布尔值更新失败: {}", error.user_friendly_message());
        }
    }

    // 示例4: 更新公式
    println!("\n=== 示例4: 更新公式 ===");
    let formula_request = UpdateCellRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .cell("D1")
        .value(CellValue::formula("SUM(A1:B1)"))
        .value_render_option("Formula")
        .build()?;

    match client.sheets.v2.sheet_cells.update_cell(formula_request, None).await {
        Ok(response) => {
            println!("✅ 公式更新成功!");
            println!("更新范围: {}", response.data.updated_range);
        }
        Err(error) => {
            println!("❌ 公式更新失败: {}", error.user_friendly_message());
        }
    }

    // 示例5: 批量更新不同类型的单元格
    println!("\n=== 示例5: 批量更新不同类型的单元格 ===");

    let updates = vec![
        ("A2", CellValue::text("销售额")),
        ("B2", CellValue::number(1500.0)),
        ("C2", CellValue::text("元")),
        ("D2", CellValue::formula("B2*1.1")), // 计算含税金额
        ("A3", CellValue::text("成本")),
        ("B3", CellValue::number(800.0)),
        ("C3", CellValue::text("元")),
        ("D3", CellValue::formula("B2-B3")), // 计算利润
    ];

    for (cell, value) in updates {
        let request = UpdateCellRequest::new(
            "shtcnmBA*****yGehy8",
            "Sheet1",
            cell,
            value,
        );

        match client.sheets.v2.sheet_cells.update_cell(request, None).await {
            Ok(response) => {
                println!("✅ 单元格 {} 更新成功!", cell);
            }
            Err(error) => {
                println!("❌ 单元格 {} 更新失败: {}", cell, error.user_friendly_message());
            }
        }
    }

    // 示例6: 错误处理演示
    println!("\n=== 示例6: 错误处理演示 ===");

    // 故意使用无效的单元格坐标
    let invalid_request = UpdateCellRequest::new(
        "shtcnmBA*****yGehy8",
        "Sheet1",
        "INVALID_COORD",  // 无效坐标
        CellValue::text("测试"),
    );

    match client.sheets.v2.sheet_cells.update_cell(invalid_request, None).await {
        Ok(_) => {
            println!("意外成功，应该失败");
        }
        Err(error) => {
            println!("✅ 正确捕获错误: {}", error.user_friendly_message());
        }
    }

    // 示例7: 空值处理
    println!("\n=== 示例7: 空值处理 ===");
    let blank_request = UpdateCellRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .sheet_id("Sheet1")
        .cell("E1")
        .value(CellValue::default()) // 空值
        .build()?;

    match client.sheets.v2.sheet_cells.update_cell(blank_request, None).await {
        Ok(response) => {
            println!("✅ 空值更新成功!");
            println!("更新范围: {}", response.data.updated_range);
        }
        Err(error) => {
            println!("❌ 空值更新失败: {}", error.user_friendly_message());
        }
    }

    println!("\n=== 所有示例执行完成 ===");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use service::sheets::v2::{CellCoordinate, CellValue};

    #[test]
    fn test_cell_coordinate_operations() {
        // 测试单元格坐标创建
        let coord = CellCoordinate::new("A", 1);
        assert_eq!(coord.to_string(), "A1");
        assert!(coord.is_valid());

        // 测试从字符串解析
        let parsed = CellCoordinate::from_string("B10").unwrap();
        assert_eq!(parsed.column, "B");
        assert_eq!(parsed.row, 10);

        // 测试无效坐标
        let invalid = CellCoordinate::from_string("INVALID");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_cell_value_operations() {
        // 测试各种值类型
        let text = CellValue::text("Hello");
        assert_eq!(text.as_string(), "Hello");
        assert!(!text.is_blank());
        assert!(!text.is_formula());

        let number = CellValue::number(42.5);
        assert_eq!(number.as_string(), "42.5");

        let boolean = CellValue::boolean(true);
        assert_eq!(boolean.as_string(), "true");

        let formula = CellValue::formula("SUM(A1:A10)");
        assert_eq!(formula.as_string(), "=SUM(A1:A10)");
        assert!(formula.is_formula());

        let blank = CellValue::default();
        assert!(blank.is_blank());
        assert_eq!(blank.as_string(), "");
    }

    #[test]
    fn test_request_validation() {
        // 测试有效请求
        let valid_request = UpdateCellRequest::new(
            "token123",
            "Sheet1",
            "A1",
            CellValue::text("Test"),
        );
        assert!(valid_request.validate().is_ok());

        // 测试无效请求（空令牌）
        let invalid_request = UpdateCellRequest::new(
            "",
            "Sheet1",
            "A1",
            CellValue::text("Test"),
        );
        assert!(invalid_request.validate().is_err());

        // 测试无效请求（无效坐标）
        let invalid_coord_request = UpdateCellRequest::new(
            "token123",
            "Sheet1",
            "INVALID",
            CellValue::text("Test"),
        );
        assert!(invalid_coord_request.validate().is_err());
    }
}
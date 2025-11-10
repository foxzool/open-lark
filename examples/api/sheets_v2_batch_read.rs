//! Sheets v2 批量范围读取示例
//!
//! 本示例展示如何使用飞书开放平台SDK v2版本的电子表格API来批量读取多个范围的数据。
//! 支持一次性读取多个单元格范围，提高数据获取效率。

use open_lark::core::config::Config;
use open_lark::prelude::*;
use open_lark::service::sheets::v2::{BatchReadService, ReadMultipleRangesRequest, ValueRange};

#[tokio::main]
async fn main() -> SDKResult<()> {
    // 初始化配置和服务
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build();

    let batch_service = BatchReadService::new(config);

    // 示例1: 批量读取两个范围
    println!("=== 示例1: 批量读取两个范围 ===");
    let request = ReadMultipleRangesRequest::new(
        "shtcnmBA*****yGehy8",       // 电子表格令牌
        "Sheet1!A1:C3,Sheet2!B2:D4", // 两个范围，逗号分隔
    );

    match batch_service.read_multiple_ranges(request, None).await {
        Ok(response) => {
            println!("✅ 批量读取成功!");
            if let Some(data) = &response.data {
                println!("表格版本: {}", data.revision);
                println!("电子表格令牌: {}", data.spreadsheet_token);
                println!("总单元格数: {}", data.total_cells);
                println!("读取的范围数: {}", data.value_ranges.len());
            }
        }
        Err(error) => {
            println!("❌ 批量读取失败: {}", error.user_friendly_message());
        }
    }

    // 示例2: 使用Builder模式批量读取多个范围
    println!("\n=== 示例2: 使用Builder模式批量读取多个范围 ===");
    let builder_request = ReadMultipleRangesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .range("销售数据!A1:F10")
        .range("库存数据!A1:H20")
        .range("财务数据!A1:J15")
        .value_render_option("FormattedValue")
        .date_time_render_option("FormattedString")
        .user_id_type("open_id")
        .build();

    match batch_service
        .read_multiple_ranges(builder_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ Builder模式批量读取成功!");
            if let Some(data) = &response.data {
                println!("读取了 {} 个范围", data.value_ranges.len());
                println!("总计 {} 个单元格", data.total_cells);
            }
        }
        Err(error) => {
            println!(
                "❌ Builder模式批量读取失败: {}",
                error.user_friendly_message()
            );
        }
    }

    // 示例3: 从向量构建范围列表
    println!("\n=== 示例3: 从向量构建范围列表 ===");
    let ranges = vec![
        "项目跟踪!A1:E50",
        "任务分配!A1:G30",
        "进度报告!A1:D20",
        "资源统计!A1:F25",
    ];

    match batch_service
        .read_ranges_from_vec("shtcnmBA*****yGehy8", ranges, None)
        .await
    {
        Ok(response) => {
            println!("✅ 向量范围读取成功!");
            if let Some(data) = &response.data {
                println!("读取范围数: {}", data.value_ranges.len());
            }
        }
        Err(error) => {
            println!("❌ 向量范围读取失败: {}", error.user_friendly_message());
        }
    }

    // 示例4: 便捷方法读取单个范围
    println!("\n=== 示例4: 便捷方法读取单个范围 ===");
    match batch_service
        .read_single_range("shtcnmBA*****yGehy8", "摘要数据!A1:Z100", None)
        .await
    {
        Ok(response) => {
            println!("✅ 单个范围读取成功!");
            if let Some(data) = &response.data {
                if let Some(value_range) = data.value_ranges.first() {
                    println!("范围: {}", value_range.range);
                    println!("单元格数: {}", count_cells_in_range(&value_range.values));
                }
            }
        }
        Err(error) => {
            println!("❌ 单个范围读取失败: {}", error.user_friendly_message());
        }
    }

    // 示例5: 动态添加范围
    println!("\n=== 示例5: 动态添加范围 ===");
    let mut dynamic_request =
        ReadMultipleRangesRequest::new("shtcnmBA*****yGehy8", "基础数据!A1:C10");

    // 根据条件动态添加范围
    let additional_ranges = vec!["扩展数据1!D1:F20", "扩展数据2!G1:I30", "扩展数据3!J1:L40"];

    for range in additional_ranges {
        dynamic_request = dynamic_request.add_range(range);
    }

    println!("最终范围列表: {}", dynamic_request.ranges);
    println!("范围数量: {}", dynamic_request.range_count());

    match batch_service
        .read_multiple_ranges(dynamic_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 动态范围读取成功!");
            if let Some(data) = &response.data {
                println!("实际读取了 {} 个范围", data.value_ranges.len());
            }
        }
        Err(error) => {
            println!("❌ 动态范围读取失败: {}", error.user_friendly_message());
        }
    }

    // 示例6: 错误处理演示
    println!("\n=== 示例6: 错误处理演示 ===");

    // 测试无效范围格式
    let invalid_request =
        ReadMultipleRangesRequest::new("shtcnmBA*****yGehy8", "InvalidRangeWithoutSheet");

    match batch_service
        .read_multiple_ranges(invalid_request, None)
        .await
    {
        Ok(_) => {
            println!("意外成功，应该失败");
        }
        Err(error) => {
            println!("✅ 正确捕获无效范围错误: {}", error.user_friendly_message());
        }
    }

    // 测试空电子表格令牌
    let empty_token_request = ReadMultipleRangesRequest::new("", "Sheet1!A1:B2");

    match batch_service
        .read_multiple_ranges(empty_token_request, None)
        .await
    {
        Ok(_) => {
            println!("意外成功，应该失败");
        }
        Err(error) => {
            println!("✅ 正确捕获空令牌错误: {}", error.user_friendly_message());
        }
    }

    // 示例7: 不同渲染选项对比
    println!("\n=== 示例7: 不同渲染选项对比 ===");
    let test_range = "测试数据!A1:C5";

    // 默认渲染
    let default_request = ReadMultipleRangesRequest::new("shtcnmBA*****yGehy8", test_range);

    // 格式化值渲染
    let formatted_request = ReadMultipleRangesRequest::new("shtcnmBA*****yGehy8", test_range)
        .value_render_option("FormattedValue");

    // 公式渲染
    let formula_request = ReadMultipleRangesRequest::new("shtcnmBA*****yGehy8", test_range)
        .value_render_option("Formula");

    println!("已创建不同渲染选项的请求，可根据需要选择合适的渲染方式");

    // 示例8: 大规模范围读取
    println!("\n=== 示例8: 大规模范围读取 ===");

    // 生成大量范围用于性能测试
    let mut large_ranges = Vec::new();
    for i in 1..=10 {
        large_ranges.push(format!("数据表{}!A1:Z100", i));
    }

    let large_request = ReadMultipleRangesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8")
        .ranges(large_ranges)
        .value_render_option("UnformattedValue") // 使用未格式化值以提高性能
        .build();

    println!("准备读取 {} 个范围", large_request.range_count());

    match batch_service
        .read_multiple_ranges(large_request, None)
        .await
    {
        Ok(response) => {
            println!("✅ 大规模读取成功!");
            if let Some(data) = &response.data {
                println!("读取范围数: {}", data.value_ranges.len());
                println!("总单元格数: {}", data.total_cells);
                println!(
                    "平均每范围单元格数: {}",
                    data.total_cells as f64 / data.value_ranges.len() as f64
                );
            }
        }
        Err(error) => {
            println!("❌ 大规模读取失败: {}", error.user_friendly_message());
        }
    }

    println!("\n=== 所有示例执行完成 ===");
    println!("注意：当前实现返回模拟数据，实际使用时需要配置有效的访问令牌");
    Ok(())
}

/// 辅助函数：统计范围内的单元格数量
fn count_cells_in_range(values: &serde_json::Value) -> i32 {
    match values {
        serde_json::Value::Array(rows) => {
            rows.iter()
                .map(|row| match row {
                    serde_json::Value::Array(cells) => cells.len() as i32,
                    _ => 1, // 单个值
                })
                .sum()
        }
        _ => 1, // 单个值
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark::service::sheets::v2::{ReadMultipleRangesRequest, ValueRange};

    #[test]
    fn test_request_creation() {
        let request = ReadMultipleRangesRequest::new("test_token", "Sheet1!A1:B2,Sheet2!C1:D1");

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.ranges, "Sheet1!A1:B2,Sheet2!C1:D1");
        assert_eq!(request.range_count(), 2);
    }

    #[test]
    fn test_add_range() {
        let request = ReadMultipleRangesRequest::new("token", "Sheet1!A1:B2")
            .add_range("Sheet2!C1:D1")
            .add_range("Sheet3!E1:F5");

        assert_eq!(request.range_count(), 3);
        assert!(request.get_ranges().contains(&"Sheet1!A1:B2"));
        assert!(request.get_ranges().contains(&"Sheet2!C1:D1"));
        assert!(request.get_ranges().contains(&"Sheet3!E1:F5"));
    }

    #[test]
    fn test_builder_pattern() {
        let request = ReadMultipleRangesRequest::builder()
            .spreadsheet_token("test_token")
            .range("Sheet1!A1:B2")
            .range("Sheet2!C1:D1")
            .value_render_option("FormattedValue")
            .user_id_type("open_id")
            .build();

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.range_count(), 2);
        assert_eq!(
            request.value_render_option,
            Some("FormattedValue".to_string())
        );
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_request_validation() {
        // 测试有效请求
        let valid_request = ReadMultipleRangesRequest::new("token123", "Sheet1!A1:B2,Sheet2!C1:D1");
        assert!(valid_request.validate().is_ok());

        // 测试无效请求（空令牌）
        let invalid_request = ReadMultipleRangesRequest::new("", "Sheet1!A1:B2");
        assert!(invalid_request.validate().is_err());

        // 测试无效请求（空范围）
        let invalid_request = ReadMultipleRangesRequest::new("token123", "");
        assert!(invalid_request.validate().is_err());

        // 测试无效请求（缺少工作表标识符）
        let invalid_request = ReadMultipleRangesRequest::new("token123", "A1:B2");
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_render_options() {
        // 测试值渲染选项
        let request = ReadMultipleRangesRequest::new("token", "Sheet1!A1:B2")
            .value_render_option("FormattedValue");
        assert!(request.validate().is_ok());

        // 测试无效的值渲染选项
        let invalid_request = ReadMultipleRangesRequest::new("token", "Sheet1!A1:B2")
            .value_render_option("InvalidOption");
        assert!(invalid_request.validate().is_err());

        // 测试日期时间渲染选项
        let request = ReadMultipleRangesRequest::new("token", "Sheet1!A1:B2")
            .date_time_render_option("FormattedString");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_user_id_types() {
        let valid_types = ["open_id", "user_id", "union_id", "lark_id"];

        for user_id_type in &valid_types {
            let request =
                ReadMultipleRangesRequest::new("token", "Sheet1!A1:B2").user_id_type(*user_id_type);
            assert!(request.validate().is_ok());
        }

        // 测试无效的用户ID类型
        let invalid_request =
            ReadMultipleRangesRequest::new("token", "Sheet1!A1:B2").user_id_type("invalid_type");
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_complex_ranges() {
        let complex_ranges = vec![
            "工作表1!A1:Z100",
            "Data Sheet!AA1:BB200",
            "Sheet with spaces!C1:D50",
            "Sheet1!$A$1:$B$2",
        ];

        let request = ReadMultipleRangesRequest::builder()
            .spreadsheet_token("test_token")
            .ranges(complex_ranges)
            .build();

        assert_eq!(request.range_count(), 4);
        assert!(request.get_ranges().contains(&"工作表1!A1:Z100"));
        assert!(request.get_ranges().contains(&"Sheet with spaces!C1:D50"));
    }

    #[test]
    fn test_ranges_from_string() {
        let ranges_string = "Sheet1!A1:B2,Sheet2!C1:D1,Sheet3!E1:F5";
        let request = ReadMultipleRangesRequest::builder()
            .spreadsheet_token("test_token")
            .ranges_from_string(ranges_string)
            .build();

        assert_eq!(request.range_count(), 3);
        assert_eq!(request.ranges, ranges_string);
    }

    #[test]
    fn test_empty_ranges_filtering() {
        let request = ReadMultipleRangesRequest::builder()
            .spreadsheet_token("token")
            .range("Sheet1!A1:B2")
            .range("") // 空范围会被包含在字符串中
            .range("Sheet2!C1:D1")
            .build();

        // 注意：当前的实现不会过滤空范围，它们会被包含在字符串中
        assert_eq!(request.range_count(), 3);
        assert!(request.ranges.contains("Sheet1!A1:B2,,Sheet2!C1:D1"));
    }

    #[test]
    fn test_unicode_support() {
        let unicode_ranges = vec!["工作表1!A1:Z100", "数据表!B2:AA50"];
        let request = ReadMultipleRangesRequest::builder()
            .spreadsheet_token("测试令牌🔥")
            .ranges(unicode_ranges)
            .build();

        assert_eq!(request.spreadsheet_token, "测试令牌🔥");
        assert_eq!(request.range_count(), 2);
        assert!(request.get_ranges().contains(&"工作表1!A1:Z100"));
        assert!(request.get_ranges().contains(&"数据表!B2:AA50"));
    }

    #[test]
    fn test_count_cells_in_range() {
        // 测试空数据
        let empty_data = serde_json::Value::Array(vec![]);
        assert_eq!(count_cells_in_range(&empty_data), 0);

        // 测试单行数据
        let single_row = serde_json::json!([["A1", "B1", "C1"]]);
        assert_eq!(count_cells_in_range(&single_row), 3);

        // 测试多行数据
        let multi_row = serde_json::json!([["A1", "B1"], ["A2", "B2"], ["A3", "B3"]]);
        assert_eq!(count_cells_in_range(&multi_row), 6);

        // 测试单个值
        let single_value = serde_json::Value::String("test".to_string());
        assert_eq!(count_cells_in_range(&single_value), 1);
    }
}

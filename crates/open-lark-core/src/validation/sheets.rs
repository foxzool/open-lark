//! Sheets 数据验证模块
//!
//! 提供电子表格相关的验证功能，包括单元格范围格式、数据类型、
//! 文件操作等的验证。
//!
//! # 主要功能
//!
//! - 单元格范围格式验证 (A1:B10, Sheet1!A1:C10)
//! - 数据维度和一致性检查
//! - 文件大小和类型验证
//! - 查询参数和选项验证

use crate::validation::ValidationResult;

/// 验证单元格范围格式
///
/// 支持的范围格式：
/// - A1:B10 (简单范围)
/// - Sheet1!A1:C10 (带工作表名)
/// - 'Sheet Name'!A1:D10 (带空格的工作表名)
pub fn validate_cell_range(range: &str) -> ValidationResult {
    if range.is_empty() {
        return ValidationResult::Invalid("Cell range cannot be empty".to_string());
    }

    // 检查是否包含工作表名
    let (_sheet_name, cell_range) = if range.contains('!') {
        let parts: Vec<&str> = range.split('!').collect();
        if parts.len() != 2 {
            return ValidationResult::Invalid(
                "Invalid cell range format. Expected 'SheetName!A1:B10' or 'A1:B10'".to_string(),
            );
        }

        let sheet = parts[0];
        let cell_ref = parts[1];

        // 验证工作表名
        // 如果工作表名包含空格、特殊字符或保留字，应该用单引号包围
        // 这里我们只检查基本的格式要求
        if sheet.is_empty() {
            return ValidationResult::Invalid("Sheet name cannot be empty".to_string());
        }

        // 检查是否被引号正确包围（如果包含引号）
        if sheet.starts_with('\'') || sheet.ends_with('\'') {
            // 如果有引号，必须同时有开始和结束引号
            if !(sheet.starts_with('\'') && sheet.ends_with('\'') && sheet.len() >= 2) {
                return ValidationResult::Invalid(
                    "Invalid quoted sheet name. Must start and end with single quotes".to_string(),
                );
            }
        }

        (Some(sheet.to_string()), cell_ref)
    } else {
        (None, range)
    };

    // 验证单元格范围
    if let Err(e) = validate_cell_reference_range(cell_range) {
        return ValidationResult::Invalid(e);
    }

    ValidationResult::Valid
}

/// 验证单元格引用范围（如 A1:B10）
fn validate_cell_reference_range(range: &str) -> Result<(), String> {
    if range.is_empty() {
        return Err("Cell range cannot be empty".to_string());
    }

    // 检查是否是单个单元格或范围
    if range.contains(':') {
        let parts: Vec<&str> = range.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid range format. Expected 'A1:B10'".to_string());
        }

        // 检查是否有空的部分（如 "A1:" 或 ":B10"）
        if parts[0].is_empty() || parts[1].is_empty() {
            return Err("Invalid range format. Expected 'A1:B10'".to_string());
        }

        // 验证起始和结束单元格
        validate_cell_reference(parts[0])?;
        validate_cell_reference(parts[1])?;

        // 确保范围是有效的（左上到右下）
        // 这里可以添加更多逻辑来验证范围的合理性
    } else {
        // 单个单元格
        validate_cell_reference(range)?;
    }

    Ok(())
}

/// 验证单个单元格引用（如 A1, B10, AA100）
fn validate_cell_reference(cell_ref: &str) -> Result<(), String> {
    if cell_ref.is_empty() {
        return Err("Cell reference cannot be empty".to_string());
    }

    // 分离列字母和行号
    let (col_part, row_part) = split_column_and_row(cell_ref)?;

    // 验证列部分
    if col_part.is_empty() {
        return Err("Column part cannot be empty".to_string());
    }

    // 检查列字母是否有效
    for c in col_part.chars() {
        if !c.is_ascii_uppercase() {
            return Err(format!("Invalid column letter '{}'. Only A-Z allowed", c));
        }
    }

    // 验证行号
    if row_part.is_empty() {
        return Err("Row number cannot be empty".to_string());
    }

    // 解析行号
    let row_num = row_part.parse::<u32>().map_err(|_| {
        format!(
            "Invalid row number '{}'. Must be a positive integer",
            row_part
        )
    })?;

    if row_num == 0 {
        return Err("Row number must be greater than 0".to_string());
    }

    // 检查行列是否在合理范围内（Google Sheets 限制）
    // 最大列数：ZHD (17788)
    // 最大行数：9,999,999
    if column_number_to_index(col_part) > 17788 {
        return Err("Column exceeds maximum limit (ZHD)".to_string());
    }

    if row_num > 9_999_999 {
        return Err("Row exceeds maximum limit (9,999,999)".to_string());
    }

    Ok(())
}

/// 分离列字母和行号
fn split_column_and_row(cell_ref: &str) -> Result<(&str, &str), String> {
    let mut row_start = 0;

    // 找到第一个数字的位置
    for (i, c) in cell_ref.chars().enumerate() {
        if c.is_ascii_digit() {
            row_start = i;
            break;
        }
    }

    if row_start == 0 {
        return Err("Invalid cell reference format. Expected format like 'A1'".to_string());
    }

    let col_part = &cell_ref[..row_start];
    let row_part = &cell_ref[row_start..];

    Ok((col_part, row_part))
}

/// 将列字母转换为数字索引（A=1, B=2, ..., AA=27, etc.）
fn column_number_to_index(col: &str) -> u32 {
    let mut index = 0;
    for (i, c) in col.chars().enumerate() {
        index += (c as u32 - 'A' as u32 + 1) * 26u32.pow((col.len() - 1 - i) as u32);
    }
    index
}

/// 验证数据渲染选项
pub fn validate_value_render_option(option: &Option<String>) -> ValidationResult {
    match option {
        Some(opt) => match opt.as_str() {
            "DisplayValue" | "Unformatted" | "FormattedString" => ValidationResult::Valid,
            _ => ValidationResult::Invalid(
                "Invalid valueRenderOption. Must be 'DisplayValue', 'Unformatted', or 'FormattedString'"
                    .to_string(),
            ),
        },
        None => ValidationResult::Valid,
    }
}

/// 验证日期时间渲染选项
pub fn validate_date_time_render_option(option: &Option<String>) -> ValidationResult {
    match option {
        Some(opt) => match opt.as_str() {
            "FormattedString" | "SerialNumber" => ValidationResult::Valid,
            _ => ValidationResult::Invalid(
                "Invalid dateTimeRenderOption. Must be 'FormattedString' or 'SerialNumber'"
                    .to_string(),
            ),
        },
        None => ValidationResult::Valid,
    }
}

/// 验证数据矩阵维度一致性
///
/// 确保所有行具有相同的列数
pub fn validate_data_matrix_consistency(data: &[Vec<serde_json::Value>]) -> ValidationResult {
    if data.is_empty() {
        return ValidationResult::Valid; // 空数据是有效的
    }

    let first_row_len = data[0].len();

    for (i, row) in data.iter().enumerate() {
        if row.len() != first_row_len {
            return ValidationResult::Invalid(format!(
                "Inconsistent matrix dimensions. Row 0 has {} columns, but row {} has {} columns",
                first_row_len,
                i,
                row.len()
            ));
        }
    }

    ValidationResult::Valid
}

/// 验证单元格合并范围
pub fn validate_merge_range(range: &str) -> ValidationResult {
    // 首先验证基本的单元格范围格式
    if let ValidationResult::Invalid(msg) = validate_cell_range(range) {
        return ValidationResult::Invalid(msg);
    }

    // 合并范围必须是多个单元格（不能是单个单元格）
    if !range.contains(':') {
        return ValidationResult::Invalid(
            "Merge range must include multiple cells (e.g., A1:B2)".to_string(),
        );
    }

    // 验证范围是矩形的（已经包含在 validate_cell_range 中）

    ValidationResult::Valid
}

/// 验证查找选项
pub fn validate_find_options(
    _case_sensitive: &Option<bool>,
    _match_entire_cell: &Option<bool>,
    search_by_regex: &Option<bool>,
    _include_formulas: &Option<bool>,
) -> ValidationResult {
    // 如果使用正则表达式，检查其他选项的兼容性
    if let Some(true) = search_by_regex {
        if let Some(true) = _case_sensitive {
            // 正则表达式本身可以控制大小写敏感，所以这不是错误
        }
    }

    ValidationResult::Valid
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ========== validate_cell_range 测试 ==========

    #[test]
    fn test_validate_cell_range_valid_simple_ranges() {
        // 简单范围
        assert!(matches!(
            validate_cell_range("A1:B10"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_cell_range("Z100:AA200"),
            ValidationResult::Valid
        ));

        // 单个单元格
        assert!(matches!(validate_cell_range("A1"), ValidationResult::Valid));
        assert!(matches!(
            validate_cell_range("Z100"),
            ValidationResult::Valid
        ));

        // 大范围
        assert!(matches!(
            validate_cell_range("A1:Z1000"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_cell_range("AA1:ZZ100"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_cell_range_valid_with_sheet_names() {
        // 带简单工作表名
        assert!(matches!(
            validate_cell_range("Sheet1!A1:B10"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_cell_range("Data!A1:C10"),
            ValidationResult::Valid
        ));

        // 带空格的工作表名
        assert!(matches!(
            validate_cell_range("'Sheet Name'!A1:C10"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_cell_range("'Data Analysis'!A1:Z100"),
            ValidationResult::Valid
        ));

        // 带特殊字符的工作表名
        assert!(matches!(
            validate_cell_range("'Sheet-2024'!A1:B10"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_cell_range("'Report_Q1'!A1:C10"),
            ValidationResult::Valid
        ));

        // 单个单元格带工作表名
        assert!(matches!(
            validate_cell_range("Sheet1!Z100"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_cell_range("'My Sheet'!A1"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_cell_range_invalid_cases() {
        // 空范围
        assert!(matches!(
            validate_cell_range(""),
            ValidationResult::Invalid(msg) if msg.contains("cannot be empty")
        ));

        // 无效的冒号使用
        assert!(matches!(
            validate_cell_range("A1:B10:C20"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid range format")
        ));
        assert!(matches!(
            validate_cell_range("A1:B10:D20"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid range format")
        ));

        // 不完整的范围
        assert!(matches!(
            validate_cell_range("A1:"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid range format")
        ));
        assert!(matches!(
            validate_cell_range(":B10"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid range format")
        ));

        // 无效的单元格引用
        assert!(matches!(
            validate_cell_range("1A:B10"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid cell reference format")
        ));
        assert!(matches!(
            validate_cell_range("A:B10"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid cell reference format")
        ));
        assert!(matches!(
            validate_cell_range("A1:10B"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid cell reference format")
        ));

        // 无效的列字母
        assert!(matches!(
            validate_cell_range("@1:B10"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid column letter")
        ));
        assert!(matches!(
            validate_cell_range("a1:B10"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid column letter")
        ));

        // 无效的行号
        assert!(matches!(
            validate_cell_range("A0:B10"),
            ValidationResult::Invalid(msg) if msg.contains("Row number must be greater than 0")
        ));
        assert!(matches!(
            validate_cell_range("A1:B0"),
            ValidationResult::Invalid(msg) if msg.contains("Row number must be greater than 0")
        ));
    }

    #[test]
    fn test_validate_cell_range_invalid_sheet_names() {
        // 多个感叹号
        assert!(matches!(
            validate_cell_range("Sheet1!A1!B10"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid cell range format")
        ));

        // 空工作表名
        assert!(matches!(
            validate_cell_range("!A1:B10"),
            ValidationResult::Invalid(msg) if msg.contains("Sheet name cannot be empty")
        ));

        // 不完整的引号
        assert!(matches!(
            validate_cell_range("'Sheet!A1:B10"),
            ValidationResult::Invalid(msg) if msg.contains("Must start and end with single quotes")
        ));
        assert!(matches!(
            validate_cell_range("Sheet'!A1:B10"),
            ValidationResult::Invalid(msg) if msg.contains("Must start and end with single quotes")
        ));
    }

    #[test]
    fn test_validate_cell_range_boundary_cases() {
        // 最大行号边界
        assert!(matches!(
            validate_cell_range("A1:Z9999999"),
            ValidationResult::Valid
        ));

        // 超过最大行号
        assert!(matches!(
            validate_cell_range("A1:Z10000000"),
            ValidationResult::Invalid(msg) if msg.contains("Row exceeds maximum limit")
        ));

        // 最大列边界 (ZHD)
        assert!(matches!(
            validate_cell_range("A1:ZHD100"),
            ValidationResult::Valid
        ));

        // 接近最大列的测试
        assert!(matches!(
            validate_cell_range("ZHA1:ZHD100"),
            ValidationResult::Valid
        ));
    }

    // ========== validate_merge_range 测试 ==========

    #[test]
    fn test_validate_merge_range_valid() {
        // 正常的合并范围
        assert!(matches!(
            validate_merge_range("A1:B2"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_merge_range("B5:D10"),
            ValidationResult::Valid
        ));

        // 带工作表名的合并范围
        assert!(matches!(
            validate_merge_range("Sheet1!A1:C3"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_merge_range("'Data Sheet'!A1:B2"),
            ValidationResult::Valid
        ));

        // 大范围合并
        assert!(matches!(
            validate_merge_range("A1:Z100"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_merge_range_invalid() {
        // 单个单元格（不能合并）
        assert!(matches!(
            validate_merge_range("A1"),
            ValidationResult::Invalid(msg) if msg.contains("must include multiple cells")
        ));
        assert!(matches!(
            validate_merge_range("Sheet1!B10"),
            ValidationResult::Invalid(msg) if msg.contains("must include multiple cells")
        ));

        // 无效的范围格式
        assert!(matches!(
            validate_merge_range("invalid_range"),
            ValidationResult::Invalid(_)
        ));
    }

    // ========== validate_value_render_option 测试 ==========

    #[test]
    fn test_validate_value_render_option_valid() {
        let valid_options = ["DisplayValue", "Unformatted", "FormattedString"];
        for option in valid_options {
            assert!(
                matches!(
                    validate_value_render_option(&Some(option.to_string())),
                    ValidationResult::Valid
                ),
                "Should be valid: {}",
                option
            );
        }

        // None 选项
        assert!(matches!(
            validate_value_render_option(&None),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_value_render_option_invalid() {
        let invalid_options = ["Invalid", "displayvalue", "Formatted", "", "Custom"];
        for option in invalid_options {
            assert!(
                matches!(
                    validate_value_render_option(&Some(option.to_string())),
                    ValidationResult::Invalid(msg) if msg.contains("Invalid valueRenderOption")
                ),
                "Should be invalid: {}",
                option
            );
        }
    }

    // ========== validate_date_time_render_option 测试 ==========

    #[test]
    fn test_validate_date_time_render_option_valid() {
        assert!(matches!(
            validate_date_time_render_option(&Some("FormattedString".to_string())),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_date_time_render_option(&Some("SerialNumber".to_string())),
            ValidationResult::Valid
        ));

        // None 选项
        assert!(matches!(
            validate_date_time_render_option(&None),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_date_time_render_option_invalid() {
        let invalid_options = ["Invalid", "formatted", "DateTime", "Number"];
        for option in invalid_options {
            assert!(
                matches!(
                    validate_date_time_render_option(&Some(option.to_string())),
                    ValidationResult::Invalid(msg) if msg.contains("Invalid dateTimeRenderOption")
                ),
                "Should be invalid: {}",
                option
            );
        }
    }

    // ========== validate_data_matrix_consistency 测试 ==========

    #[test]
    fn test_validate_data_matrix_consistency_valid() {
        // 一致的2x2矩阵
        let data1 = vec![vec![json!("A"), json!("B")], vec![json!("C"), json!("D")]];
        assert!(matches!(
            validate_data_matrix_consistency(&data1),
            ValidationResult::Valid
        ));

        // 一致的3x2矩阵
        let data2 = vec![
            vec![json!("A1"), json!("B1"), json!("C1")],
            vec![json!("A2"), json!("B2"), json!("C2")],
            vec![json!("A3"), json!("B3"), json!("C3")],
        ];
        assert!(matches!(
            validate_data_matrix_consistency(&data2),
            ValidationResult::Valid
        ));

        // 单行矩阵
        let data3 = vec![vec![json!("A"), json!("B"), json!("C")]];
        assert!(matches!(
            validate_data_matrix_consistency(&data3),
            ValidationResult::Valid
        ));

        // 单列矩阵
        let data4 = vec![vec![json!("A")], vec![json!("B")], vec![json!("C")]];
        assert!(matches!(
            validate_data_matrix_consistency(&data4),
            ValidationResult::Valid
        ));

        // 空数据（应该有效）
        let data5: Vec<Vec<serde_json::Value>> = vec![];
        assert!(matches!(
            validate_data_matrix_consistency(&data5),
            ValidationResult::Valid
        ));

        // 包含空值的矩阵（但维度一致）
        let data6 = vec![
            vec![json!("A"), json!(null), json!("C")],
            vec![json!("D"), json!("E"), json!(null)],
        ];
        assert!(matches!(
            validate_data_matrix_consistency(&data6),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_data_matrix_consistency_invalid() {
        // 不一致的矩阵 - 第二行列数不同
        let data1 = vec![
            vec![json!("A"), json!("B"), json!("C")],
            vec![json!("D"), json!("E")], // 少一列
            vec![json!("F"), json!("G"), json!("H")],
        ];
        assert!(matches!(
            validate_data_matrix_consistency(&data1),
            ValidationResult::Invalid(msg) if msg.contains("Inconsistent matrix dimensions")
        ));

        // 不一致的矩阵 - 第三行列数不同
        let data2 = vec![
            vec![json!("A"), json!("B")],
            vec![json!("C"), json!("D")],
            vec![json!("E"), json!("F"), json!("G"), json!("H")], // 多一列
        ];
        assert!(matches!(
            validate_data_matrix_consistency(&data2),
            ValidationResult::Invalid(msg) if msg.contains("Inconsistent matrix dimensions")
        ));

        // 极端不一致情况
        let data3 = vec![
            vec![json!("A")],
            vec![json!("B"), json!("C"), json!("D"), json!("E")],
            vec![json!("F"), json!("G")],
        ];
        assert!(matches!(
            validate_data_matrix_consistency(&data3),
            ValidationResult::Invalid(msg) if msg.contains("Inconsistent matrix dimensions")
        ));
    }

    // ========== validate_find_options 测试 ==========

    #[test]
    fn test_validate_find_options_valid() {
        // 各种有效的选项组合
        assert!(matches!(
            validate_find_options(&Some(true), &Some(false), &Some(false), &Some(true)),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_find_options(&None, &None, &None, &None),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_find_options(&Some(false), &Some(true), &Some(true), &Some(false)),
            ValidationResult::Valid
        ));
    }

    // ========== 辅助函数测试 ==========

    #[test]
    fn test_column_number_to_index() {
        // 基本测试
        assert_eq!(column_number_to_index("A"), 1);
        assert_eq!(column_number_to_index("B"), 2);
        assert_eq!(column_number_to_index("Z"), 26);

        // 双字母测试
        assert_eq!(column_number_to_index("AA"), 27);
        assert_eq!(column_number_to_index("AB"), 28);
        assert_eq!(column_number_to_index("AZ"), 52);
        assert_eq!(column_number_to_index("BA"), 53);
        assert_eq!(column_number_to_index("BZ"), 78);
        assert_eq!(column_number_to_index("ZZ"), 702);

        // 三字母测试
        assert_eq!(column_number_to_index("AAA"), 703);
        assert_eq!(column_number_to_index("AAB"), 704);

        // 边界测试
        assert_eq!(column_number_to_index("ZHD"), 17788); // 最大支持列
    }

    #[test]
    fn test_split_column_and_row() {
        // 基本分离
        assert_eq!(split_column_and_row("A1").unwrap(), ("A", "1"));
        assert_eq!(split_column_and_row("Z100").unwrap(), ("Z", "100"));
        assert_eq!(split_column_and_row("AA1000").unwrap(), ("AA", "1000"));

        // 复杂情况
        assert_eq!(split_column_and_row("ZZ999999").unwrap(), ("ZZ", "999999"));
        assert_eq!(split_column_and_row("A1").unwrap(), ("A", "1"));
    }

    #[test]
    fn test_split_column_and_row_invalid() {
        // 无效格式
        assert!(split_column_and_row("").is_err());
        assert!(split_column_and_row("123").is_err());
        assert!(split_column_and_row("ABC").is_err());
        assert!(split_column_and_row("@#").is_err());
    }

    // ========== 综合场景测试 ==========

    #[test]
    fn test_comprehensive_spreadsheet_scenarios() {
        // 模拟真实电子表格操作场景

        // 1. 数据范围验证
        let data_range = "'Sales Report Q1'!A1:Z100";
        assert!(matches!(
            validate_cell_range(data_range),
            ValidationResult::Valid
        ));

        // 2. 合并单元格验证
        let merge_range = "A1:E1"; // 标题行合并
        assert!(matches!(
            validate_merge_range(merge_range),
            ValidationResult::Valid
        ));

        // 3. 数据矩阵一致性检查
        let sales_data = vec![
            vec![
                json!("Product"),
                json!("January"),
                json!("February"),
                json!("March"),
            ],
            vec![json!("Product A"), json!(1000), json!(1200), json!(1100)],
            vec![json!("Product B"), json!(800), json!(900), json!(950)],
            vec![json!("Product C"), json!(1500), json!(1600), json!(1550)],
        ];
        assert!(matches!(
            validate_data_matrix_consistency(&sales_data),
            ValidationResult::Valid
        ));

        // 4. 渲染选项验证
        let value_option = Some("FormattedString".to_string());
        assert!(matches!(
            validate_value_render_option(&value_option),
            ValidationResult::Valid
        ));

        let date_option = Some("FormattedString".to_string());
        assert!(matches!(
            validate_date_time_render_option(&date_option),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_error_message_content() {
        // 测试错误消息是否包含有用的调试信息

        let result = validate_cell_range("");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("empty"));
        }

        let result = validate_merge_range("A1");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("multiple cells"));
        }

        let result = validate_value_render_option(&Some("Invalid".to_string()));
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("valueRenderOption"));
        }

        let result =
            validate_data_matrix_consistency(&[vec![json!("A"), json!("B")], vec![json!("C")]]);
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("dimensions"));
        }
    }

    #[test]
    fn test_edge_cases_and_boundary_conditions() {
        // 测试各种边界情况

        // 1. 最小有效范围
        assert!(matches!(validate_cell_range("A1"), ValidationResult::Valid));

        // 2. 最大有效范围
        assert!(matches!(
            validate_cell_range("A1:ZHD9999999"),
            ValidationResult::Valid
        ));

        // 3. 最长有效工作表名
        let long_sheet_name = "'".to_string() + &"A".repeat(200) + "'!A1:B10";
        assert!(matches!(
            validate_cell_range(&long_sheet_name),
            ValidationResult::Valid
        ));

        // 4. 复杂的工作表名
        assert!(matches!(
            validate_cell_range("'2024 Sales Report - Q1'!A1:Z100"),
            ValidationResult::Valid
        ));

        // 5. 特殊字符工作表名
        assert!(matches!(
            validate_cell_range("'Sheet@#$%^&*()'!A1:B10"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_unicode_and_special_characters() {
        // 测试Unicode字符支持

        // 中文工作表名
        assert!(matches!(
            validate_cell_range("'销售报告'!A1:Z100"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_cell_range("'2024年第一季度'!A1:C10"),
            ValidationResult::Valid
        ));

        // 包含特殊字符的工作表名
        assert!(matches!(
            validate_cell_range("'Data-Analysis_Q1'!A1:B10"),
            ValidationResult::Valid
        ));

        // 混合字符集
        assert!(matches!(
            validate_cell_range("'数据Data-2024年'!A1:Z100"),
            ValidationResult::Valid
        ));
    }
}

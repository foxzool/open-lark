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

use crate::core::validation::ValidationResult;

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
        if sheet.starts_with('\'') && (!sheet.ends_with('\'') || sheet.len() < 2) {
            return ValidationResult::Invalid(
                "Invalid quoted sheet name. Must start and end with single quotes".to_string(),
            );
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
    // 最大列数：ZHD (18278)
    // 最大行数：9,999,999
    if column_number_to_index(col_part) > 18278 {
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

    #[test]
    fn test_validate_cell_range_valid() {
        // 简单范围
        assert!(matches!(
            validate_cell_range("A1:B10"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_cell_range("Z100:AA200"),
            ValidationResult::Valid
        ));

        // 带工作表名
        assert!(matches!(
            validate_cell_range("Sheet1!A1:B10"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_cell_range("'Sheet Name'!A1:C10"),
            ValidationResult::Valid
        ));

        // 单个单元格
        assert!(matches!(validate_cell_range("A1"), ValidationResult::Valid));
        assert!(matches!(
            validate_cell_range("Sheet1!Z100"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_cell_range_invalid() {
        // 空范围
        assert!(matches!(
            validate_cell_range(""),
            ValidationResult::Invalid(_)
        ));

        // 无效格式
        assert!(matches!(
            validate_cell_range("A1:B10:C20"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_cell_range("Sheet1!A1:"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_cell_range("Sheet1!"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_value_render_option() {
        assert!(matches!(
            validate_value_render_option(&Some("DisplayValue".to_string())),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_value_render_option(&Some("Invalid".to_string())),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_value_render_option(&None),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_column_number_to_index() {
        assert_eq!(column_number_to_index("A"), 1);
        assert_eq!(column_number_to_index("Z"), 26);
        assert_eq!(column_number_to_index("AA"), 27);
        assert_eq!(column_number_to_index("AZ"), 52);
        assert_eq!(column_number_to_index("BA"), 53);
    }

    #[test]
    fn test_validate_data_matrix_consistency() {
        // 一致的数据
        let data1 = vec![
            vec![
                serde_json::Value::String("A".to_string()),
                serde_json::Value::String("B".to_string()),
            ],
            vec![
                serde_json::Value::String("C".to_string()),
                serde_json::Value::String("D".to_string()),
            ],
        ];
        assert!(matches!(
            validate_data_matrix_consistency(&data1),
            ValidationResult::Valid
        ));

        // 不一致的数据
        let data2 = vec![
            vec![
                serde_json::Value::String("A".to_string()),
                serde_json::Value::String("B".to_string()),
            ],
            vec![serde_json::Value::String("C".to_string())],
        ];
        assert!(matches!(
            validate_data_matrix_consistency(&data2),
            ValidationResult::Invalid(_)
        ));
    }
}

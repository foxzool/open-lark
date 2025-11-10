//! 数据前置插入服务
//!
//! 提供SHEETS v2数据前置插入功能，支持：
//! - 在指定工作表的起始位置插入数据
//! - 智能移动现有数据，为插入的数据腾出空间
//! - 支持多种数据格式：直接数组、HashMap转换、CSV
//! - 大规模数据操作支持
//! - 构建器模式和传统请求结构两种API调用方式

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::core::request::Transport;
use crate::core::SDKResult;
use crate::core::config::Config;
use crate::core::trait_system::Service;
use crate::core::error::LarkAPIError;

/// 数据前置插入请求
#[derive(Debug, Clone)]
pub struct ValuesPrependRequest {
    /// 电子表格令牌
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 要插入的数据
    pub values: ValuesData,
    /// 写入选项
    pub options: Option<WriteOptions>,
}

/// 写入选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOptions {
    /// 是否以公式形式写入
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_formula: Option<bool>,
    /// 是否覆盖已有数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    /// 是否在写入后清除格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_format: Option<bool>,
}

/// 数据容器
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValuesData {
    /// 二维数组格式
    Array(Vec<Vec<String>>),
    /// HashMap格式（行索引 -> 数据）
    HashMap(HashMap<i32, Vec<String>>),
}

impl ValuesPrependRequest {
    /// 创建新的数据前置插入请求
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格令牌
    /// - `sheet_id`: 工作表ID
    /// - `values`: 要插入的数据
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::values_prepend::*;
    ///
    /// // 使用二维数组
    /// let data = vec![
    ///     vec!["姓名".to_string(), "年龄".to_string(), "城市".to_string()],
    ///     vec!["张三".to_string(), "25".to_string(), "北京".to_string()],
    /// ];
    /// let request = ValuesPrependRequest::new("token", "sheet_id", data);
    ///
    /// // 使用HashMap
    /// let mut hashmap = HashMap::new();
    /// hashmap.insert(0, vec!["标题1".to_string(), "标题2".to_string()]);
    /// hashmap.insert(1, vec!["数据1".to_string(), "数据2".to_string()]);
    /// let request = ValuesPrependRequest::new("token", "sheet_id", hashmap);
    /// ```
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        values: impl Into<ValuesData>,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            values: values.into(),
            options: None,
        }
    }

    /// 设置写入选项
    pub fn options(mut self, options: WriteOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// 设置公式模式
    pub fn as_formula(mut self, as_formula: bool) -> Self {
        let mut opts = self.options.unwrap_or_default();
        opts.as_formula = Some(as_formula);
        self.options = Some(opts);
        self
    }

    /// 设置覆盖模式
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        let mut opts = self.options.unwrap_or_default();
        opts.overwrite = Some(overwrite);
        self.options = Some(opts);
        self
    }

    /// 设置清除格式
    pub fn clear_format(mut self, clear_format: bool) -> Self {
        let mut opts = self.options.unwrap_or_default();
        opts.clear_format = Some(clear_format);
        self.options = Some(opts);
        self
    }

    /// 转换为API请求体
    pub fn to_request_body(&self) -> SDKResult<Value> {
        let mut map = Map::new();

        // 设置工作表ID
        map.insert("sheetId".to_string(), Value::String(self.sheet_id.clone()));

        // 转换数据
        let values_value = match &self.values {
            ValuesData::Array(arr) => {
                Value::Array(arr.iter().map(|row| {
                    Value::Array(row.iter().map(|cell| Value::String(cell.clone())).collect())
                }).collect())
            }
            ValuesData::HashMap(hashmap) => {
                let mut rows = Vec::new();
                // 找出最大的行索引
                let max_row = hashmap.keys().max().unwrap_or(&0);
                for i in 0..=*max_row {
                    let row = hashmap.get(&i).unwrap_or(&vec![]);
                    rows.push(Value::Array(row.iter().map(|cell| Value::String(cell.clone())).collect()));
                }
                Value::Array(rows)
            }
        };
        map.insert("values".to_string(), values_value);

        // 设置选项
        if let Some(ref options) = self.options {
            let options_value = serde_json::to_value(options)
                .map_err(|e| LarkAPIError::InvalidParameter(format!("写入选项序列化失败: {}", e)))?;
            map.insert("options".to_string(), options_value);
        }

        Ok(Value::Object(map))
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证电子表格令牌
        if self.spreadsheet_token.trim().is_empty() {
            return Err(LarkAPIError::InvalidParameter("电子表格令牌不能为空".to_string()));
        }

        // 验证工作表ID
        if self.sheet_id.trim().is_empty() {
            return Err(LarkAPIError::InvalidParameter("工作表ID不能为空".to_string()));
        }

        // 验证数据
        match &self.values {
            ValuesData::Array(rows) => {
                if rows.is_empty() {
                    return Err(LarkAPIError::InvalidParameter("插入数据不能为空".to_string()));
                }

                if rows.len() > 5000 {
                    return Err(LarkAPIError::InvalidParameter("单次插入数据不能超过5000行".to_string()));
                }

                for (row_index, row) in rows.iter().enumerate() {
                    if row.len() > 100 {
                        return Err(LarkAPIError::InvalidParameter(
                            format!("第{}行数据列数不能超过100列", row_index + 1)
                        ));
                    }

                    for (col_index, cell) in row.iter().enumerate() {
                        if cell.len() > 500 {
                            return Err(LarkAPIError::InvalidParameter(
                                format!("第{}行第{}列单元格内容长度不能超过500字符", row_index + 1, col_index + 1)
                            ));
                        }
                    }
                }
            }
            ValuesData::HashMap(hashmap) => {
                if hashmap.is_empty() {
                    return Err(LarkAPIError::InvalidParameter("插入数据不能为空".to_string()));
                }

                // 检查HashMap的最大行数
                let max_row = hashmap.keys().max().unwrap_or(&0);
                if *max_row >= 5000 {
                    return Err(LarkAPIError::InvalidParameter("行索引不能超过5000".to_string()));
                }

                for (row_index, row) in hashmap {
                    if *row_index < 0 {
                        return Err(LarkAPIError::InvalidParameter("行索引不能为负数".to_string()));
                    }

                    if row.len() > 100 {
                        return Err(LarkAPIError::InvalidParameter(
                            format!("第{}行数据列数不能超过100列", row_index)
                        ));
                    }

                    for (col_index, cell) in row.iter().enumerate() {
                        if cell.len() > 500 {
                            return Err(LarkAPIError::InvalidParameter(
                                format!("第{}行第{}列单元格内容长度不能超过500字符", row_index, col_index)
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// 从CSV字符串创建数据前置插入请求
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格令牌
    /// - `sheet_id`: 工作表ID
    /// - `csv_content`: CSV内容字符串
    /// - `delimiter`: CSV分隔符，默认为逗号
    ///
    /// # 示例
    ///
    /// ```rust
    /// let csv_data = "姓名,年龄,城市\n张三,25,北京\n李四,30,上海";
    /// let request = ValuesPrependRequest::from_csv("token", "sheet_id", csv_data, ",");
    /// ```
    pub fn from_csv(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        csv_content: impl AsRef<str>,
        delimiter: char,
    ) -> SDKResult<Self> {
        let mut rows = Vec::new();
        let mut line_number = 0;

        for line in csv_content.as_ref().lines() {
            line_number += 1;

            // 处理CSV解析，支持引号包围的字段和转义引号
            let fields = Self::parse_csv_line(line, delimiter, line_number)?;

            // 移除字段的引号（如果有）
            let processed_fields: Vec<String> = fields
                .into_iter()
                .map(|field| {
                    if field.starts_with('"') && field.ends_with('"') && field.len() >= 2 {
                        // 移除首尾引号并处理转义引号
                        field[1..field.len() - 1].replace("\"\"", "\"")
                    } else {
                        field.replace("\"\"", "\"")
                    }
                })
                .collect();

            rows.push(processed_fields);
        }

        if rows.is_empty() {
            return Err(LarkAPIError::InvalidParameter("CSV内容不能为空".to_string()));
        }

        Ok(Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            values: ValuesData::Array(rows),
            options: None,
        })
    }

    /// 解析CSV行，处理引号和转义字符
    fn parse_csv_line(line: &str, delimiter: char, line_number: usize) -> SDKResult<Vec<String>> {
        let mut fields = Vec::new();
        let mut current_field = String::new();
        let mut in_quotes = false;
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    if in_quotes {
                        // 检查下一个字符是否是引号（转义引号）
                        if let Some(&next_c) = chars.peek() {
                            if next_c == '"' {
                                // 转义引号，跳过下一个引号
                                current_field.push('"');
                                chars.next();
                            } else {
                                // 结束引号
                                in_quotes = false;
                            }
                        } else {
                            // 行结束，结束引号
                            in_quotes = false;
                        }
                    } else {
                        // 开始引号
                        in_quotes = true;
                    }
                }
                c if c == delimiter && !in_quotes => {
                    // 分隔符且不在引号内，结束当前字段
                    fields.push(current_field.clone());
                    current_field.clear();
                }
                _ => {
                    current_field.push(c);
                }
            }
        }

        // 添加最后一个字段
        fields.push(current_field);

        if in_quotes {
            return Err(LarkAPIError::InvalidParameter(
                format!("第{}行：引号未正确关闭", line_number)
            ));
        }

        Ok(fields)
    }
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            as_formula: Some(false),
            overwrite: Some(false),
            clear_format: Some(false),
        }
    }
}

impl From<Vec<Vec<String>>> for ValuesData {
    fn from(value: Vec<Vec<String>>) -> Self {
        ValuesData::Array(value)
    }
}

impl From<HashMap<i32, Vec<String>>> for ValuesData {
    fn from(value: HashMap<i32, Vec<String>>) -> Self {
        ValuesData::HashMap(value)
    }
}

/// 数据前置插入响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesPrependResponse {
    /// 写入的行数
    #[serde(rename = "row_count")]
    pub row_count: i32,
    /// 写入的列数
    #[serde(rename = "column_count")]
    pub column_count: i32,
    /// 移动的现有行数
    #[serde(rename = "moved_rows")]
    pub moved_rows: Option<i32>,
}

/// API响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesPrependResponseBody {
    /// 数据前置插入结果
    pub data: ValuesPrependResponse,
}

/// 基础API响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    /// 响应码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: T,
}

/// 数据前置插入服务
#[derive(Debug, Clone)]
pub struct ValuesPrependService {
    config: Config,
}

impl ValuesPrependService {
    /// 创建数据前置插入服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 数据前置插入
    ///
    /// 在指定工作表的起始位置插入数据，自动移动现有数据为插入数据腾出空间
    ///
    /// # 参数
    /// - `request`: 数据前置插入请求
    ///
    /// # 返回
    /// 插入结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::values_prepend::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = ValuesPrependService::new(config);
    ///
    /// // 准备数据
    /// let data = vec![
    ///     vec!["标题".to_string(), "值".to_string()],
    ///     vec!["名称".to_string(), "100".to_string()],
    /// ];
    ///
    /// let request = ValuesPrependRequest::new("spreadsheet_token", "sheet_id", data)
    ///     .as_formula(false)
    ///     .overwrite(false);
    ///
    /// let response = service.prepend(request).await?;
    /// println!("成功写入 {} 行 {} 列数据", response.data.row_count, response.data.column_count);
    /// ```
    pub async fn prepend(&self, request: ValuesPrependRequest) -> SDKResult<ValuesPrependResponse> {
        // 验证请求参数
        request.validate()?;

        // 构建请求体
        let body = request.to_request_body()?;

        // 发送HTTP请求
        let url = format!(
            "{}/open-apis/sheets/v2/spreadsheets/{}/values_prepend",
            self.config.base_url, request.spreadsheet_token
        );

        let response = self.config
            .transport
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LarkAPIError::NetworkError(format!("网络请求失败: {}", e)))?;

        // 处理响应
        if response.status().is_success() {
            let base_response: BaseResponse<ValuesPrependResponseBody> = response.json().await
                .map_err(|e| LarkAPIError::JsonParseError(format!("响应解析失败: {}", e)))?;

            if base_response.code == 0 {
                Ok(base_response.data.data)
            } else {
                Err(LarkAPIError::APIError(base_response.code, base_response.msg))
            }
        } else {
            Err(LarkAPIError::HTTPError(response.status().as_u16(), "数据前置插入失败".to_string()))
        }
    }

    /// 创建数据前置插入构建器
    pub fn prepend_builder(&self, spreadsheet_token: &str, sheet_id: &str) -> ValuesPrependBuilder {
        ValuesPrependBuilder::new(self.clone(), spreadsheet_token, sheet_id)
    }
}

impl Service for ValuesPrependService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ValuesPrependService"
    }
}

/// 数据前置插入构建器
pub struct ValuesPrependBuilder {
    service: ValuesPrependService,
    spreadsheet_token: String,
    sheet_id: String,
    values: Option<ValuesData>,
    options: WriteOptions,
}

impl ValuesPrependBuilder {
    /// 创建新的构建器
    pub fn new(service: ValuesPrependService, spreadsheet_token: &str, sheet_id: &str) -> Self {
        Self {
            service,
            spreadsheet_token: spreadsheet_token.to_string(),
            sheet_id: sheet_id.to_string(),
            values: None,
            options: WriteOptions::default(),
        }
    }

    /// 设置二维数组数据
    pub fn data_array(mut self, data: Vec<Vec<String>>) -> Self {
        self.values = Some(ValuesData::Array(data));
        self
    }

    /// 设置HashMap格式数据
    pub fn data_hashmap(mut self, data: HashMap<i32, Vec<String>>) -> Self {
        self.values = Some(ValuesData::HashMap(data));
        self
    }

    /// 设置CSV数据
    pub fn data_csv(self, csv_content: impl AsRef<str>, delimiter: char) -> SDKResult<Self> {
        let values_data = ValuesPrependRequest::from_csv(
            &self.spreadsheet_token,
            &self.sheet_id,
            csv_content,
            delimiter,
        )?;

        Ok(Self {
            service: self.service,
            spreadsheet_token: self.spreadsheet_token,
            sheet_id: self.sheet_id,
            values: Some(values_data.values),
            options: self.options,
        })
    }

    /// 设置公式模式
    pub fn as_formula(mut self, as_formula: bool) -> Self {
        self.options.as_formula = Some(as_formula);
        self
    }

    /// 设置覆盖模式
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.options.overwrite = Some(overwrite);
        self
    }

    /// 设置清除格式
    pub fn clear_format(mut self, clear_format: bool) -> Self {
        self.options.clear_format = Some(clear_format);
        self
    }

    /// 设置写入选项
    pub fn options(mut self, options: WriteOptions) -> Self {
        self.options = options;
        self
    }

    /// 执行前置插入
    pub async fn execute(self) -> SDKResult<ValuesPrependResponse> {
        let request = ValuesPrependRequest {
            spreadsheet_token: self.spreadsheet_token,
            sheet_id: self.sheet_id,
            values: self.values.ok_or_else(|| ValuesData::Array(Vec::new())),
            options: Some(self.options),
        };

        self.service.prepend(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values_prepend_request_validation() {
        // 测试正常请求
        let data = vec![
            vec!["标题".to_string(), "值".to_string()],
            vec!["名称".to_string(), "100".to_string()],
        ];
        let request = ValuesPrependRequest::new("token", "sheet_id", data);
        assert!(request.validate().is_ok());

        // 测试空令牌
        let request = ValuesPrependRequest::new("", "sheet_id", data);
        assert!(request.validate().is_err());

        // 测试空工作表ID
        let request = ValuesPrependRequest::new("token", "", data);
        assert!(request.validate().is_err());

        // 测试空数据
        let empty_data: Vec<Vec<String>> = vec![];
        let request = ValuesPrependRequest::new("token", "sheet_id", empty_data);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_values_prepend_request_large_data() {
        // 测试超过最大行数
        let large_data: Vec<Vec<String>> = (0..5001).map(|_| vec!["测试".to_string()]).collect();
        let request = ValuesPrependRequest::new("token", "sheet_id", large_data);
        assert!(request.validate().is_err());

        // 测试超过最大列数
        let wide_data: Vec<Vec<String>> = vec![vec!["测试".to_string(); 101]];
        let request = ValuesPrependRequest::new("token", "sheet_id", wide_data);
        assert!(request.validate().is_err());

        // 测试单元格内容过长
        let long_data = vec![vec![{
            let mut s = "a".repeat(501);
            s
        }]];
        let request = ValuesPrependRequest::new("token", "sheet_id", long_data);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_hashmap_data_validation() {
        let mut hashmap = HashMap::new();
        hashmap.insert(0, vec!["标题".to_string()]);
        hashmap.insert(1, vec!["数据".to_string()]);

        let request = ValuesPrependRequest::new("token", "sheet_id", hashmap);
        assert!(request.validate().is_ok());

        // 测试负数行索引
        let mut invalid_hashmap = HashMap::new();
        invalid_hashmap.insert(-1, vec!["无效".to_string()]);
        let request = ValuesPrependRequest::new("token", "sheet_id", invalid_hashmap);
        assert!(request.validate().is_err());

        // 测试超大行索引
        let mut large_hashmap = HashMap::new();
        large_hashmap.insert(5001, vec!["超大".to_string()]);
        let request = ValuesPrependRequest::new("token", "sheet_id", large_hashmap);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_csv_parsing() {
        // 测试简单CSV
        let csv = "姓名,年龄,城市
张三,25,北京
李四,30,上海";
        let request = ValuesPrependRequest::from_csv("token", "sheet_id", csv, ',').unwrap();

        match request.values {
            ValuesData::Array(rows) => {
                assert_eq!(rows.len(), 3);
                assert_eq!(rows[0], vec!["姓名", "年龄", "城市"]);
                assert_eq!(rows[1], vec!["张三", "25", "北京"]);
                assert_eq!(rows[2], vec!["李四", "30", "上海"]);
            }
            _ => panic!("期望数组格式"),
        }

        // 测试带引号的CSV
        let csv_with_quotes = r#"名称,"描述","数量"
"产品A","包含引号的,内容",100
"产品B","普通内容",200"#;
        let request = ValuesPrependRequest::from_csv("token", "sheet_id", csv_with_quotes, ',').unwrap();

        match request.values {
            ValuesData::Array(rows) => {
                assert_eq!(rows.len(), 3);
                assert_eq!(rows[0], vec!["名称", "描述", "数量"]);
                assert_eq!(rows[1], vec!["产品A", "包含引号的,内容", "100"]);
                assert_eq!(rows[2], vec!["产品B", "普通内容", "200"]);
            }
            _ => panic!("期望数组格式"),
        }

        // 测试转义引号
        let csv_escape = r#"名称,"包含""引号""的文本"
"测试","""引号""""#;
        let request = ValuesPrependRequest::from_csv("token", "sheet_id", csv_escape, ',').unwrap();

        match request.values {
            ValuesData::Array(rows) => {
                assert_eq!(rows.len(), 2);
                assert_eq!(rows[0], vec!["名称", "包含\"引号\"的文本"]);
                assert_eq!(rows[1], vec!["测试", "\"引号\""]);
            }
            _ => panic!("期望数组格式"),
        }
    }

    #[test]
    fn test_csv_parsing_invalid() {
        // 测试未闭合的引号
        let csv_invalid = "名称,描述
"产品A","未闭合的字段";
        let result = ValuesPrependRequest::from_csv("token", "sheet_id", csv_invalid, ',');
        assert!(result.is_err());

        // 测试空内容
        let csv_empty = "";
        let result = ValuesPrependRequest::from_csv("token", "sheet_id", csv_empty, ',');
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_pattern() {
        let config = Config::default();
        let service = ValuesPrependService::new(config);

        let data = vec![
            vec!["标题".to_string(), "值".to_string()],
            vec!["名称".to_string(), "数据".to_string()],
        ];

        let builder = service.prepend_builder("token", "sheet_id")
            .data_array(data)
            .as_formula(true)
            .overwrite(false)
            .clear_format(true);

        assert_eq!(builder.spreadsheet_token, "token");
        assert_eq!(builder.sheet_id, "sheet_id");
        assert_eq!(builder.options.as_formula, Some(true));
        assert_eq!(builder.options.overwrite, Some(false));
        assert_eq!(builder.options.clear_format, Some(true));
    }

    #[test]
    fn test_builder_with_csv() {
        let config = Config::default();
        let service = ValuesPrependService::new(config);

        let csv_data = "Name,Age
John,25
Jane,30";
        let builder = service.prepend_builder("token", "sheet_id")
            .data_csv(csv_data, ',')
            .unwrap()
            .as_formula(false);

        assert_eq!(builder.spreadsheet_token, "token");
        assert_eq!(builder.sheet_id, "sheet_id");
        assert_eq!(builder.options.as_formula, Some(false));
    }

    #[test]
    fn test_write_options_default() {
        let options = WriteOptions::default();
        assert_eq!(options.as_formula, Some(false));
        assert_eq!(options.overwrite, Some(false));
        assert_eq!(options.clear_format, Some(false));
    }

    #[test]
    fn test_complex_data_operations() {
        // 测试HashMap格式数据
        let mut hashmap = HashMap::new();
        hashmap.insert(0, vec!["ID".to_string(), "名称".to_string(), "价格".to_string()]);
        hashmap.insert(2, vec!["1".to_string(), "产品A".to_string(), "100.00".to_string()]);
        hashmap.insert(5, vec!["2".to_string(), "产品B".to_string(), "200.50".to_string()]);

        let request = ValuesPrependRequest::new("token", "sheet_id", hashmap)
            .as_formula(true)
            .clear_format(true);

        assert!(request.validate().is_ok());
        assert_eq!(request.options.as_ref().unwrap().as_formula, Some(true));
        assert_eq!(request.options.as_ref().unwrap().clear_format, Some(true));

        // 测试CSV复杂数据
        let complex_csv = "ID,Name,Price
001,Item1,10.50
002,Item2,25.00";
        let csv_request = ValuesPrependRequest::from_csv("token", "sheet_id", complex_csv, ',').unwrap();

        match csv_request.values {
            ValuesData::Array(rows) => {
                assert_eq!(rows.len(), 3);
                assert_eq!(rows[0], vec!["ID", "Name", "Price"]);
                assert_eq!(rows[1], vec!["001", "Item1", "10.50"]);
                assert_eq!(rows[2], vec!["002", "Item2", "25.00"]);
            }
            _ => panic!("Expected array format")
        }
    }

    #[test]
    fn test_values_prepend_service() {
        let config = Config::default();
        let service = ValuesPrependService::new(config);

        assert_eq!(service.service_name(), "ValuesPrependService");
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_request_body_serialization() {
        let data = vec![
            vec!["A1".to_string(), "B1".to_string()],
            vec!["A2".to_string(), "B2".to_string()],
        ];

        let options = WriteOptions {
            as_formula: Some(true),
            overwrite: Some(false),
            clear_format: Some(true),
        };

        let request = ValuesPrependRequest::new("token", "sheet_id", data)
            .options(options);

        let body = request.to_request_body().unwrap();

        assert_eq!(body["sheetId"], "sheet_id");
        assert!(body["values"].is_array());
        assert_eq!(body["values"].as_array().unwrap().len(), 2);
        assert_eq!(body["options"]["as_formula"], true);
        assert_eq!(body["options"]["overwrite"], false);
        assert_eq!(body["options"]["clear_format"], true);
    }
}
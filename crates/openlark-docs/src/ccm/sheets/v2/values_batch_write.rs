//! Sheets v2 批量范围写入服务
//!
//! 提供飞书电子表格v2版本的批量范围写入功能，包括：
//! - 一次性向多个单元格范围写入数据，提高效率
//! - 支持Excel风格的范围格式（如 Sheet1!A1:B10）
//! - 灵活的值输入选项和数据解析策略
//! - 智能的数据类型识别和转换
//! - 详细的写入结果，包括每个范围的状态
//! - 企业级错误处理和数据验证
//! - 构建器模式支持，提供流畅API设计
use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints_original::Endpoints,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use openlark_core::trait_system::Service;
// use openlark_core::SDKResult;

/// 单个写入范围数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteRange {
    /// 范围标识符，支持Excel风格格式
    /// 例如："Sheet1!A1:B2", "数据表!C1:D10"
    pub range: String,
    /// 要写入的数据，二维数组格式
    pub values: Vec<Vec<Value>>,
}

impl WriteRange {
    /// 创建新的写入范围
    pub fn new(range: impl Into<String>, values: Vec<Vec<Value>>) -> Self {
        Self {
            range: range.into(),
            values,
        }
    }

    /// 验证写入范围数据
    pub fn validate(&self) -> SDKResult<()> {
        // 验证范围
        if self.range.trim().is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "写入范围不能为空".to_string(),
            ));
        }

        // 验证数据
        if self.values.is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "写入数据不能为空".to_string(),
            ));
        }

        // 验证数据大小限制
        if self.values.len() > 5000 {
            return Err(LarkAPIError::InvalidParameter(
                "单个范围写入行数不能超过5000行".to_string(),
            ));
        }

        for (row_index, row) in self.values.iter().enumerate() {
            if row.len() > 100 {
                return Err(LarkAPIError::InvalidParameter(format!(
                    "第{}行列数不能超过100列",
                    row_index + 1
                )));
            }
        }

        // 验证范围格式
        let range_upper = self.range.to_uppercase();
        if !range_upper.contains('!')
            && !range_upper
                .matches(|c: char| c.is_ascii_alphabetic())
                .next()
                .is_some()
        {
            return Err(LarkAPIError::InvalidParameter(
                "范围格式不正确，应为SheetName!A1:B10格式".to_string(),
            ));
        }

        Ok(())
    }
}

/// 批量范围写入请求
#[derive(Clone, Debug)]
pub struct ValuesBatchWriteRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 写入范围列表
    pub ranges: Vec<WriteRange>,
    /// 值输入选项
    pub value_input_option: Option<ValueInputOption>,
    /// 数据解析选项
    pub data_parse_option: Option<DataParseOption>,
    /// 是否在响应中包含写入的值
    pub include_values_in_response: Option<bool>,
    /// 响应数据渲染格式
    pub response_value_render_option: Option<ResponseValueRenderOption>,
    /// 响应日期渲染格式
    pub response_date_render_option: Option<ResponseDateRenderOption>,
}

/// 值输入选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueInputOption {
    /// 原始值（默认）
    #[serde(rename = "RAW")]
    Raw,
    /// 用户输入格式
    #[serde(rename = "USER_ENTERED")]
    UserEntered,
}

/// 数据解析选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataParseOption {
    /// 自动解析（默认）
    #[serde(rename = "PARSE_AUTO")]
    ParseAuto,
    /// 不解析，保持原始字符串
    #[serde(rename = "PARSE_NONE")]
    ParseNone,
}

/// 响应值渲染选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseValueRenderOption {
    /// 未格式化值（默认）
    #[serde(rename = "UNFORMATTED_VALUE")]
    UnformattedValue,
    /// 用户输入格式
    #[serde(rename = "USER_ENTERED_FORMAT")]
    UserEnteredFormat,
    /// 格式化值
    #[serde(rename = "FORMATTED_VALUE")]
    FormattedValue,
}

/// 响应日期渲染选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseDateRenderOption {
    /// 序列号（默认）
    #[serde(rename = "SERIAL_NUMBER")]
    SerialNumber,
    /// 格式化字符串
    #[serde(rename = "FORMATTED_STRING")]
    FormattedString,
}

impl Default for ValuesBatchWriteRequest {
    fn default() -> Self {
        Self {
            spreadsheet_token: String::new(),
            ranges: vec![],
            value_input_option: None,
            data_parse_option: None,
            include_values_in_response: None,
            response_value_render_option: None,
            response_date_render_option: None,
        }
    }
}

impl ValuesBatchWriteRequest {
    /// 创建新的批量范围写入请求
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            
        }
    }

    /// 添加写入范围
    pub fn add_range(mut self, range: impl Into<String>, values: Vec<Vec<Value>>) -> Self {
        let write_range = WriteRange::new(range, values);
        self.ranges.push(write_range);
        self
    }

    /// 批量添加写入范围
    pub fn ranges(mut self, ranges: Vec<WriteRange>) -> Self {
        self.ranges = ranges;
        self
    }

    /// 设置值输入选项
    pub fn value_input_option(mut self, option: ValueInputOption) -> Self {
        self.value_input_option = Some(option);
        self
    }

    /// 设置数据解析选项
    pub fn data_parse_option(mut self, option: DataParseOption) -> Self {
        self.data_parse_option = Some(option);
        self
    }

    /// 设置是否包含响应值
    pub fn include_values_in_response(mut self, include: bool) -> Self {
        self.include_values_in_response = Some(include);
        self
    }

    /// 设置响应值渲染选项
    pub fn response_value_render_option(mut self, option: ResponseValueRenderOption) -> Self {
        self.response_value_render_option = Some(option);
        self
    }

    /// 设置响应日期渲染选项
    pub fn response_date_render_option(mut self, option: ResponseDateRenderOption) -> Self {
        self.response_date_render_option = Some(option);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证电子表格token
        if self.spreadsheet_token.trim().is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "电子表格token不能为空".to_string(),
            ));
        }

        // 验证范围列表
        if self.ranges.is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "至少需要指定一个写入范围".to_string(),
            ));
        }

        if self.ranges.len() > 100 {
            return Err(LarkAPIError::InvalidParameter(
                "一次最多只能写入100个范围".to_string(),
            ));
        }

        // 验证每个范围
        for (index, range) in self.ranges.iter().enumerate() {
            range.validate().map_err(|e| {
                LarkAPIError::InvalidParameter(format!("范围{}验证失败: {}", index + 1, e))
            })?;
        }

        // 验证总数据量限制
        let total_cells: usize = self
            .ranges
            .iter()
            .map(|r| r.values.len() * r.values.first().map_or(0, |row| row.len()))
            .sum();
        if total_cells > 500000 {
            return Err(LarkAPIError::InvalidParameter(format!(
                "总写入单元格数不能超过500000，当前为{}",
                total_cells
            )));
        }

        Ok(())
    }

    /// 转换为API请求体
    pub fn to_request_body(&self) -> SDKResult<Value> {
        let mut body = serde_json::Map::new();

        // 添加写入范围
        let ranges_value = serde_json::to_value(&self.ranges)
            .map_err(|e| LarkAPIError::InvalidParameter(format!("写入范围序列化失败: {}", e)))?;
        body.insert("data".to_string(), ranges_value);

        // 添加值输入选项
        if let Some(option) = &self.value_input_option {
            let option_str = match option {
                ValueInputOption::Raw => "RAW",
                ValueInputOption::UserEntered => "USER_ENTERED",
            };
            body.insert(
                "valueInputOption".to_string(),
                Value::String(option_str.to_string()),
            );
        }

        // 添加数据解析选项
        if let Some(option) = &self.data_parse_option {
            let option_str = match option {
                DataParseOption::ParseAuto => "PARSE_AUTO",
                DataParseOption::ParseNone => "PARSE_NONE",
            };
            body.insert(
                "dataParseOption".to_string(),
                Value::String(option_str.to_string()),
            );
        }

        // 添加是否包含响应值
        if let Some(include) = self.include_values_in_response {
            body.insert("includeValuesInResponse".to_string(), Value::Bool(include));
        }

        // 添加响应值渲染选项
        if let Some(option) = &self.response_value_render_option {
            let option_str = match option {
                ResponseValueRenderOption::UnformattedValue => "UNFORMATTED_VALUE",
                ResponseValueRenderOption::UserEnteredFormat => "USER_ENTERED_FORMAT",
                ResponseValueRenderOption::FormattedValue => "FORMATTED_VALUE",
            };
            body.insert(
                "responseValueRenderOption".to_string(),
                Value::String(option_str.to_string()),
            );
        }

        // 添加响应日期渲染选项
        if let Some(option) = &self.response_date_render_option {
            let option_str = match option {
                ResponseDateRenderOption::SerialNumber => "SERIAL_NUMBER",
                ResponseDateRenderOption::FormattedString => "FORMATTED_STRING",
            };
            body.insert(
                "responseDateRenderOption".to_string(),
                Value::String(option_str.to_string()),
            );
        }

        Ok(Value::Object(body))
    }
}

/// 单个范围写入结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteRangeResult {
    /// 写入的范围
    pub range: String,
    /// 更新的行数
    pub updated_rows: Option<i32>,
    /// 更新的列数
    pub updated_columns: Option<i32>,
    /// 更新的单元格数
    pub updated_cells: Option<i32>,
    /// 写入的数据（仅在include_values_in_response为true时返回）
    pub values: Option<Vec<Vec<String>>>,
    /// 写入状态
    pub status: Option<String>,
}

/// 批量范围写入响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesBatchWriteResponse {
    /// 所有范围的写入结果
    pub write_responses: Vec<WriteRangeResult>,
    /// 总更新的范围数
    pub total_updated_ranges: Option<i32>,
    /// 总更新的单元格数
    pub total_updated_cells: Option<i32>,
    /// 电子表格ID
    pub spreadsheet_id: Option<String>,
}

/// API响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesBatchWriteResponseBody {
    /// 批量写入结果
    pub data: ValuesBatchWriteResponse,
}

// 移除重复的BaseResponse定义，使用openlark_core中的版本

/// 批量范围写入服务
#[derive(Clone, Debug)]
pub struct ValuesBatchWriteService {
    config: Config,
}

impl ValuesBatchWriteService {
    /// 创建批量范围写入服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量写入多个范围的数据
    ///
    /// # 参数
    /// - `request`: 批量范围写入请求
    ///
    /// # 返回
    /// 所有范围的写入结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::values_batch_write::*;
    /// use open_lark::core::config::Config;
    /// use serde_json::json;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = ValuesBatchWriteService::new(config);
    ///
    /// // 准备多个范围的数据
    /// let request = ValuesBatchWriteRequest::new("spreadsheet_token")
    ///     .add_range("Sheet1!A1:C3", vec![
    ///         vec![json!("姓名"), json!("年龄"), json!("部门")],
    ///         vec![json!("张三"), json!(25), json!("技术部")],
    ///         vec![json!("李四"), json!(30), json!("产品部")]
    ///     ])
    ///     .add_range("Sheet2!A1:B2", vec![
    ///         vec![json!("项目"), json!("状态")],
    ///         vec![json!("项目A"), json!("进行中")]
    ///     ])
    ///     .value_input_option(ValueInputOption::Raw)
    ///     .include_values_in_response(true);
    /// ```
    pub async fn batch_write(
        &self,
        request: ValuesBatchWriteRequest,
    ) -> SDKResult<ValuesBatchWriteResponse> {
        // 验证请求参数
        request.validate()?;

        // 构建请求体
        let body = request.to_request_body()?;

        // 构建URL
        let url = format!(
            "{}/open-apis/sheets/v2/spreadsheets/{}/values_batch_update",
            self.config.base_url, request.spreadsheet_token
        );

        // 发送HTTP请求
        let response = self
            .config
            .transport
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LarkAPIError::NetworkError(format!("网络请求失败: {}", e)))?;

        // 处理响应
        if response.status().is_success() {
            let base_response: Response<ValuesBatchWriteResponseBody> = response
                .json()
                .await
                .map_err(|e| LarkAPIError::JsonParseError(format!("响应解析失败: {}", e)))?;

            if base_response.code == 0 {
                Ok(base_response.data.data)
            } else {
                Err(LarkAPIError::APIError(
                    base_response.code,
                    base_response.msg,
                ))
            }
        } else {
            Err(LarkAPIError::HTTPError(
                response.status().as_u16(),
                "批量范围写入失败".to_string(),
            ))
        }
    }

    /// 创建批量范围写入构建器
    pub fn batch_write_builder(&self) -> ValuesBatchWriteBuilder {
        ValuesBatchWriteBuilder::new(self.clone())
    }

    /// 快速批量写入数据
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `ranges_data`: 范围和数据的映射，键为范围，值为数据
    ///
    /// # 返回
    /// 写入结果
    pub async fn write_ranges(
        &self,
        spreadsheet_token: impl Into<String>,
        ranges_data: HashMap<String, Vec<Vec<Value>>>,
    ) -> SDKResult<ValuesBatchWriteResponse> {
        let mut ranges = vec![];
        for (range, values) in ranges_data {
            ranges.push(WriteRange::new(range, values));
        }

        let request = ValuesBatchWriteRequest::new(spreadsheet_token).ranges(ranges);

        self.batch_write(request).await
    }

    /// 从CSV数据写入多个范围
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `csv_mappings`: CSV数据和范围的映射
    /// - `delimiter`: CSV分隔符，默认为逗号
    ///
    /// # 返回
    /// 写入结果
    pub async fn write_csv_ranges(
        &self,
        spreadsheet_token: impl Into<String>,
        csv_mappings: HashMap<String, (String, char)>, // 键为CSV数据，值为(范围, 分隔符)
    ) -> SDKResult<ValuesBatchWriteResponse> {
        let mut ranges = vec![];

        for (csv_data, (range, delimiter)) in csv_mappings {
            let values = Self::parse_csv_to_values(&csv_data, delimiter)?;
            ranges.push(WriteRange::new(range, values));
        }

        let request = ValuesBatchWriteRequest::new(spreadsheet_token)
            .ranges(ranges)
            .data_parse_option(DataParseOption::ParseAuto);

        self.batch_write(request).await
    }

    /// 从HashMap数据写入多个范围
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `hashmap_mappings`: HashMap数据和范围的映射
    /// - `include_headers`: 是否包含表头
    ///
    /// # 返回
    /// 写入结果
    pub async fn write_hashmap_ranges(
        &self,
        spreadsheet_token: impl Into<String>,
        hashmap_mappings: HashMap<String, (HashMap<String, Vec<Value>>, bool)>, // 键为范围，值为(数据, 是否包含表头)
    ) -> SDKResult<ValuesBatchWriteResponse> {
        let mut ranges = vec![];

        for (range, (data, include_headers)) in hashmap_mappings {
            let values = Self::hashmap_to_values(data, include_headers)?;
            ranges.push(WriteRange::new(range, values));
        }

        let request = ValuesBatchWriteRequest::new(spreadsheet_token)
            .ranges(ranges)
            .value_input_option(ValueInputOption::Raw);

        self.batch_write(request).await
    }

    /// 将CSV数据解析为值数组
    fn parse_csv_to_values(csv_data: &str, delimiter: char) -> SDKResult<Vec<Vec<Value>>> {
        let mut result = vec![];

        for line in csv_data.lines() {
            let row = Self::parse_csv_line(line, delimiter)?
                .into_iter()
                .map(|cell| Value::String(cell))
                .collect();
            result.push(row);
        }

        if result.is_empty() {
            return Err(LarkAPIError::InvalidParameter("CSV数据为空".to_string()));
        }

        Ok(result)
    }

    /// 解析CSV行
    fn parse_csv_line(line: &str, delimiter: char) -> SDKResult<Vec<String>> {
        let mut result = vec![];
        let mut current = String::new();
        let mut in_quotes = false;
        let mut chars = line.chars().peekable();

        while let Some(&ch) = chars.next() {
            match ch {
                '"' => {
                    if in_quotes && chars.peek() == Some(&'"') {
                        // 双引号转义
                        current.push('"');
                        chars.next(); // 跳过第二个引号
                    } else {
                        // 开始或结束引用
                        in_quotes = !in_quotes;
                    }
                }
                ch if ch == delimiter && !in_quotes => {
                    // 字段分隔符（不在引用中）
                    result.push(current.trim().to_string());
                    current = String::new();
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        // 添加最后一个字段
        result.push(current.trim().to_string());

        Ok(result)
    }

    /// 将HashMap数据转换为值数组
    fn hashmap_to_values(
        data: HashMap<String, Vec<Value>>,
        include_headers: bool,
    ) -> SDKResult<Vec<Vec<Value>>> {
        if data.is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "HashMap数据为空".to_string(),
            ));
        }

        let keys: Vec<String> = data.keys().cloned().collect();
        let row_count = data.values().next().map(|v| v.len()).unwrap_or(0);

        if row_count == 0 {
            return Err(LarkAPIError::InvalidParameter(
                "HashMap数据行为空".to_string(),
            ));
        }

        // 验证所有列的行数一致
        for (key, values) in &data {
            if values.len() != row_count {
                return Err(LarkAPIError::InvalidParameter(format!(
                    "列 '{}' 的行数({})与其他列不一致({})",
                    key,
                    values.len(),
                    row_count
                )));
            }
        }

        let mut result = vec![];

        // 添加表头（如果需要）
        if include_headers {
            let header_row = keys.iter().map(|k| Value::String(k.clone())).collect();
            result.push(header_row);
        }

        // 添加数据行
        for row_index in 0..row_count {
            let mut row = vec![];
            for key in &keys {
                if let Some(values) = data.get(key) {
                    row.push(values[row_index].clone());
                }
            }
            result.push(row);
        }

        Ok(result)
    }
}

impl Service for ValuesBatchWriteService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ValuesBatchWriteService"
    }
}

/// 批量范围写入构建器
pub struct ValuesBatchWriteBuilder {
    service: ValuesBatchWriteService,
    spreadsheet_token: Option<String>,
    ranges: Vec<WriteRange>,
    value_input_option: Option<ValueInputOption>,
    data_parse_option: Option<DataParseOption>,
    include_values_in_response: Option<bool>,
    response_value_render_option: Option<ResponseValueRenderOption>,
    response_date_render_option: Option<ResponseDateRenderOption>,
}

impl ValuesBatchWriteBuilder {
    /// 创建新的构建器
    pub fn new(service: ValuesBatchWriteService) -> Self {
        Self {
            service,
            spreadsheet_token: None,
            ranges: vec![],
            value_input_option: None,
            data_parse_option: None,
            include_values_in_response: None,
            response_value_render_option: None,
            response_date_render_option: None,
        }
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = Some(token.into());
        self
    }

    /// 添加写入范围
    pub fn add_range(mut self, range: impl Into<String>, values: Vec<Vec<Value>>) -> Self {
        let write_range = WriteRange::new(range, values);
        self.ranges.push(write_range);
        self
    }

    /// 批量添加写入范围
    pub fn ranges(mut self, ranges: Vec<WriteRange>) -> Self {
        self.ranges = ranges;
        self
    }

    /// 设置值输入选项
    pub fn value_input_option(mut self, option: ValueInputOption) -> Self {
        self.value_input_option = Some(option);
        self
    }

    /// 设置数据解析选项
    pub fn data_parse_option(mut self, option: DataParseOption) -> Self {
        self.data_parse_option = Some(option);
        self
    }

    /// 设置是否包含响应值
    pub fn include_values_in_response(mut self, include: bool) -> Self {
        self.include_values_in_response = Some(include);
        self
    }

    /// 设置响应值渲染选项
    pub fn response_value_render_option(mut self, option: ResponseValueRenderOption) -> Self {
        self.response_value_render_option = Some(option);
        self
    }

    /// 设置响应日期渲染选项
    pub fn response_date_render_option(mut self, option: ResponseDateRenderOption) -> Self {
        self.response_date_render_option = Some(option);
        self
    }

    /// 执行批量写入操作
    pub async fn execute(self) -> SDKResult<ValuesBatchWriteResponse> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::InvalidParameter("电子表格token不能为空".to_string()))?;

        if self.ranges.is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "至少需要指定一个写入范围".to_string(),
            ));
        }

        let mut request = ValuesBatchWriteRequest::new(spreadsheet_token);
        request.ranges = self.ranges;

        if let Some(option) = self.value_input_option {
            request = request.value_input_option(option);
        }

        if let Some(option) = self.data_parse_option {
            request = request.data_parse_option(option);
        }

        if let Some(include) = self.include_values_in_response {
            request = request.include_values_in_response(include);
        }

        if let Some(option) = self.response_value_render_option {
            request = request.response_value_render_option(option);
        }

        if let Some(option) = self.response_date_render_option {
            request = request.response_date_render_option(option);
        }

        self.service.batch_write(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_write_range_validation() {
        // 测试空范围
        let range = WriteRange::new("", vec![]);
        assert!(range.validate().is_err());

        // 测试空数据
        let range = WriteRange::new("Sheet1!A1:C10", vec![]);
        assert!(range.validate().is_err());

        // 测试正常范围
        let values = vec![vec![json!("test")]];
        let range = WriteRange::new("Sheet1!A1:C10", values);
        assert!(range.validate().is_ok());

        // 测试行数限制
        let large_values = (0..5001).map(|_| vec![json!("test")]).collect();
        let range = WriteRange::new("Sheet1!A1:C10", large_values);
        assert!(range.validate().is_err());

        // 测试列数限制
        let wide_row = (0..101).map(|_| json!("test")).collect();
        let range = WriteRange::new("Sheet1!A1:C10", vec![wide_row]);
        assert!(range.validate().is_err());

        // 测试范围格式
        let range = WriteRange::new("", vec![vec![json!("test")]]);
        assert!(range.validate().is_err());
    }

    #[test]
    fn test_values_batch_write_request_validation() {
        // 测试空token
        let request =
            ValuesBatchWriteRequest::new("").add_range("Sheet1!A1:C10", vec![vec![json!("test")]]);
        assert!(request.validate().is_err());

        // 测试空范围列表
        let request = ValuesBatchWriteRequest::new("token");
        assert!(request.validate().is_err());

        // 测试范围数量限制
        let ranges = (0..101)
            .map(|i| {
                WriteRange::new(
                    format!("Sheet1!A{}:C{}", i * 10 + 1, i * 10 + 10),
                    vec![vec![json!("test")]],
                )
            })
            .collect();
        let request = ValuesBatchWriteRequest::new("token").ranges(ranges);
        assert!(request.validate().is_err());

        // 测试正常请求
        let request = ValuesBatchWriteRequest::new("token")
            .add_range("Sheet1!A1:C10", vec![vec![json!("test")]])
            .add_range("Sheet2!A1:E20", vec![vec![json!("test2")]]);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_to_request_body() {
        let request = ValuesBatchWriteRequest::new("token")
            .add_range(
                "Sheet1!A1:B2",
                vec![
                    vec![json!("姓名"), json!("年龄")],
                    vec![json!("张三"), json!(25)],
                ],
            )
            .value_input_option(ValueInputOption::Raw)
            .include_values_in_response(true);

        let body = request.to_request_body().unwrap();

        assert!(body["data"].is_array());
        assert_eq!(body["valueInputOption"], "RAW");
        assert_eq!(body["includeValuesInResponse"], true);
    }

    #[test]
    fn test_value_input_options() {
        let request = ValuesBatchWriteRequest::new("token")
            .add_range("Sheet1!A1", vec![vec![json!("test")]])
            .value_input_option(ValueInputOption::UserEntered);

        let body = request.to_request_body().unwrap();
        assert_eq!(body["valueInputOption"], "USER_ENTERED");
    }

    #[test]
    fn test_parse_csv_line() {
        // 测试基本CSV解析
        let result = ValuesBatchWriteService::parse_csv_line("a,b,c", ',').unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);

        // 测试带引号的字段
        let result = ValuesBatchWriteService::parse_csv_line("a,\"b,c\",d", ',').unwrap();
        assert_eq!(result, vec!["a", "b,c", "d"]);

        // 测试转义引号
        let result = ValuesBatchWriteService::parse_csv_line("a,\"b\"\"c\",d", ',').unwrap();
        assert_eq!(result, vec!["a", "b\"c", "d"]);
    }

    #[test]
    fn test_parse_csv_to_values() {
        let csv_data = "姓名,年龄\n张三,25\n李四,30";
        let values = ValuesBatchWriteService::parse_csv_to_values(csv_data, ',').unwrap();

        assert_eq!(values.len(), 3);
        assert_eq!(values[0][0], json!("姓名"));
        assert_eq!(values[0][1], json!("年龄"));
        assert_eq!(values[1][0], json!("张三"));
        assert_eq!(values[1][1], json!("25"));
    }

    #[test]
    fn test_hashmap_to_values() {
        let mut data = HashMap::new();
        data.insert("姓名".to_string(), vec![json!("张三"), json!("李四")]);
        data.insert("年龄".to_string(), vec![json!(25), json!(30)]);

        // 测试包含表头
        let values = ValuesBatchWriteService::hashmap_to_values(data.clone() true).unwrap();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0][0], json!("姓名"));
        assert_eq!(values[0][1], json!("年龄"));

        // 测试不包含表头
        let values = ValuesBatchWriteService::hashmap_to_values(data, false).unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0][0], json!("张三"));
        assert_eq!(values[0][1], json!(25));
    }

    #[test]
    fn test_builder_pattern() {
        let config = openlark_core::config::Config::default();
        let service = ValuesBatchWriteService::new(config);

        let values1 = vec![vec![json!("test1")]];
        let values2 = vec![vec![json!("test2")]];

        let builder = service
            .batch_write_builder()
            .spreadsheet_token("test_token")
            .add_range("Sheet1!A1", values1)
            .add_range("Sheet2!B2", values2)
            .value_input_option(ValueInputOption::Raw)
            .include_values_in_response(true);

        // 验证构建器设置
        assert_eq!(builder.spreadsheet_token.as_ref().unwrap(), "test_token");
        assert_eq!(builder.ranges.len(), 2);
        assert!(matches!(
            builder.value_input_option,
            Some(ValueInputOption::Raw)
        ));
        assert_eq!(builder.include_values_in_response, Some(true));
    }

    #[test]
    fn test_values_batch_write_service() {
        let config = openlark_core::config::Config::default();
        let service = ValuesBatchWriteService::new(config);

        assert_eq!(service.service_name(), "ValuesBatchWriteService");
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_complex_batch_write_scenarios() {
        let request = ValuesBatchWriteRequest::new("token")
            .add_range(
                "Sheet1!A1:C3",
                vec![
                    vec![json!("姓名"), json!("年龄"), json!("部门")],
                    vec![json!("张三"), json!(25), json!("技术部")],
                    vec![json!("李四"), json!(30), json!("产品部")],
                ],
            )
            .add_range(
                "Sheet2!A1:B2",
                vec![
                    vec![json!("项目"), json!("状态")],
                    vec![json!("项目A"), json!("进行中")],
                ],
            )
            .add_range("Summary!D1:E1", vec![vec![json!("总计"), json!(100)]])
            .value_input_option(ValueInputOption::UserEntered)
            .data_parse_option(DataParseOption::ParseAuto)
            .include_values_in_response(true)
            .response_value_render_option(ResponseValueRenderOption::FormattedValue);

        assert!(request.validate().is_ok());

        let body = request.to_request_body().unwrap();
        assert_eq!(body["valueInputOption"], "USER_ENTERED");
        assert_eq!(body["dataParseOption"], "PARSE_AUTO");
        assert_eq!(body["responseValueRenderOption"], "FORMATTED_VALUE");
    }

    #[test]
    fn test_error_handling() {
        // 测试HashMap行数不一致
        let mut data = HashMap::new();
        data.insert("列1".to_string(), vec![json!("a"), json!("b")]);
        data.insert("列2".to_string(), vec![json!("c")]); // 只有一行

        let result = ValuesBatchWriteService::hashmap_to_values(data, false);
        assert!(result.is_err());

        // 测试空HashMap
        let empty_data = HashMap::new();
        let result = ValuesBatchWriteService::hashmap_to_values(empty_data, true);
        assert!(result.is_err());

        // 测试空CSV
        let result = ValuesBatchWriteService::parse_csv_to_values("", ',');
        assert!(result.is_err());
    }

    #[test]
    fn test_large_data_validation() {
        // 测试总单元格数限制
        let large_range = WriteRange::new(
            "Sheet1!A1:CV5000", // 100列 x 5000行 = 500000单元格
            (0..5000)
                .map(|_| (0..100).map(|_| json!("data")).collect())
                .collect(),
        );
        assert!(large_range.validate().is_ok());

        // 测试超出限制的数据
        let very_large_range = WriteRange::new(
            "Sheet1!A1:CW5001", // 101列 x 5001行 = 506101单元格，超出限制
            (0..5001)
                .map(|_| (0..101).map(|_| json!("data")).collect())
                .collect(),
        );
        assert!(very_large_range.validate().is_err());
    }
}

//! Sheets v2 单个范围写入服务
//!
//! 提供飞书电子表格v2版本的单个范围数据写入功能，包括：
//! - 向指定范围写入数据，支持多种数据格式
//! - 灵活的值输入选项（原始值、用户输入格式、公式）
//! - 智能数据类型识别和转换
//! - 写入结果详细信息，包括更新范围和单元格数量
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

/// 单个范围写入请求
#[derive(Clone, Debug)]
pub struct ValuesSingleWriteRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 写入范围，支持Excel风格格式（如 Sheet1!A1:C10）
    pub range: String,
    /// 要写入的数据，二维数组结构
    pub values: Vec<Vec<Value>>,
    /// 值输入选项
    pub value_input_option: Option<ValueInputOption>,
    /// 数据解析选项
    pub data_parse_option: Option<DataParseOption>,
    /// 是否包含表格中的值
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

impl Default for ValuesSingleWriteRequest {
    fn default() -> Self {
        Self {
            spreadsheet_token: String::new(),
            range: String::new(),
            values: vec![],
            value_input_option: None,
            data_parse_option: None,
            include_values_in_response: None,
            response_value_render_option: None,
            response_date_render_option: None,
        }
    }
}

impl ValuesSingleWriteRequest {
    /// 创建新的单个范围写入请求
    pub fn new(
        spreadsheet_token: impl Into<String>,
        range: impl Into<String>,
        values: Vec<Vec<Value>>,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            range: range.into(),
            values,
            
        }
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
        if self.values.len() > 10000 {
            return Err(LarkAPIError::InvalidParameter(
                "写入行数不能超过10000行".to_string(),
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

    /// 转换为API请求体
    pub fn to_request_body(&self) -> SDKResult<Value> {
        let mut body = serde_json::Map::new();

        // 添加范围
        body.insert("range".to_string(), Value::String(self.range.clone()));

        // 添加数据
        let values_value = serde_json::to_value(&self.values)
            .map_err(|e| LarkAPIError::InvalidParameter(format!("数据序列化失败: {}", e)))?;
        body.insert("values".to_string(), values_value);

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

/// 单个范围写入响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesSingleWriteResponse {
    /// 更新的范围
    pub updated_range: Option<String>,
    /// 更新的行数
    pub updated_rows: Option<i32>,
    /// 更新的列数
    pub updated_columns: Option<i32>,
    /// 更新的单元格数
    pub updated_cells: Option<i32>,
    /// 写入的数据（仅在include_values_in_response为true时返回）
    pub values: Option<Vec<Vec<String>>>,
    /// 电子表格ID
    pub spreadsheet_id: Option<String>,
}

/// API响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesSingleWriteResponseBody {
    /// 写入结果
    pub data: ValuesSingleWriteResponse,
}

// 移除重复的BaseResponse定义，使用openlark_core中的版本

/// 单个范围写入服务
#[derive(Clone, Debug)]
pub struct ValuesSingleWriteService {
    config: Config,
}

impl ValuesSingleWriteService {
    /// 创建单个范围写入服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 向单个范围写入数据
    ///
    /// # 参数
    /// - `request`: 单个范围写入请求
    ///
    /// # 返回
    /// 写入结果，包括更新范围和单元格数量
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::values_single_write::*;
    /// use open_lark::core::config::Config;
    /// use serde_json::json;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = ValuesSingleWriteService::new(config);
    ///
    /// // 准备写入数据
    /// let values = vec![
    ///     vec![json!("姓名"), json!("年龄"), json!("部门")],
    ///     vec![json!("张三"), json!(25), json!("技术部")],
    ///     vec![json!("李四"), json!(30), json!("产品部")],
    /// ];
    ///
    /// // 创建写入请求
    /// let request = ValuesSingleWriteRequest::new(
    ///     "spreadsheet_token",
    ///     "Sheet1!A1:C3",
    ///     values
    /// )
    /// .value_input_option(ValueInputOption::Raw)
    /// .include_values_in_response(true);
    /// ```
    pub async fn write(
        &self,
        request: ValuesSingleWriteRequest,
    ) -> SDKResult<ValuesSingleWriteResponse> {
        // 验证请求参数
        request.validate()?;

        // 构建请求体
        let body = request.to_request_body()?;

        // 构建URL
        let url = format!(
            "{}/open-apis/sheets/v2/spreadsheets/{}/values",
            self.config.base_url, request.spreadsheet_token
        );

        // 发送HTTP请求
        let response = self
            .config
            .transport
            .put(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LarkAPIError::NetworkError(format!("网络请求失败: {}", e)))?;

        // 处理响应
        if response.status().is_success() {
            let base_response: Response<ValuesSingleWriteResponseBody> = response
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
                "单个范围写入失败".to_string(),
            ))
        }
    }

    /// 创建单个范围写入构建器
    pub fn write_builder(&self) -> ValuesSingleWriteBuilder {
        ValuesSingleWriteBuilder::new(self.clone())
    }

    /// 快速写入数据
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `range`: 写入范围，如 "Sheet1!A1:C10"
    /// - `values`: 要写入的数据，二维数组
    ///
    /// # 返回
    /// 写入结果
    pub async fn write_values(
        &self,
        spreadsheet_token: impl Into<String>,
        range: impl Into<String>,
        values: Vec<Vec<Value>>,
    ) -> SDKResult<ValuesSingleWriteResponse> {
        let request = ValuesSingleWriteRequest::new(spreadsheet_token, range, values);
        self.write(request).await
    }

    /// 写入CSV格式数据
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `range`: 写入范围，如 "Sheet1!A1:C10"
    /// - `csv_data`: CSV格式字符串
    /// - `delimiter`: 分隔符，默认为逗号
    ///
    /// # 返回
    /// 写入结果
    pub async fn write_csv(
        &self,
        spreadsheet_token: impl Into<String>,
        range: impl Into<String>,
        csv_data: &str,
        delimiter: char,
    ) -> SDKResult<ValuesSingleWriteResponse> {
        let values = Self::parse_csv_to_values(csv_data, delimiter)?;
        let request = ValuesSingleWriteRequest::new(spreadsheet_token, range, values)
            .data_parse_option(DataParseOption::ParseAuto);
        self.write(request).await
    }

    /// 写入HashMap格式数据
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `range`: 写入范围，如 "Sheet1!A1:C10"
    /// - `data`: HashMap数据，键为列名，值为数据列表
    /// - `include_headers`: 是否包含表头
    ///
    /// # 返回
    /// 写入结果
    pub async fn write_hashmap(
        &self,
        spreadsheet_token: impl Into<String>,
        range: impl Into<String>,
        data: HashMap<String, Vec<Value>>,
        include_headers: bool,
    ) -> SDKResult<ValuesSingleWriteResponse> {
        let values = Self::hashmap_to_values(data, include_headers)?;
        let request = ValuesSingleWriteRequest::new(spreadsheet_token, range, values)
            .value_input_option(ValueInputOption::Raw);
        self.write(request).await
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

impl Service for ValuesSingleWriteService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ValuesSingleWriteService"
    }
}

/// 单个范围写入构建器
pub struct ValuesSingleWriteBuilder {
    service: ValuesSingleWriteService,
    spreadsheet_token: Option<String>,
    range: Option<String>,
    values: Option<Vec<Vec<Value>>>,
    value_input_option: Option<ValueInputOption>,
    data_parse_option: Option<DataParseOption>,
    include_values_in_response: Option<bool>,
    response_value_render_option: Option<ResponseValueRenderOption>,
    response_date_render_option: Option<ResponseDateRenderOption>,
}

impl ValuesSingleWriteBuilder {
    /// 创建新的构建器
    pub fn new(service: ValuesSingleWriteService) -> Self {
        Self {
            service,
            spreadsheet_token: None,
            range: None,
            values: None,
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

    /// 设置写入范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 设置写入数据
    pub fn values(mut self, values: Vec<Vec<Value>>) -> Self {
        self.values = Some(values);
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

    /// 执行写入操作
    pub async fn execute(self) -> SDKResult<ValuesSingleWriteResponse> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::InvalidParameter("电子表格token不能为空".to_string()))?;

        let range = self
            .range
            .ok_or_else(|| LarkAPIError::InvalidParameter("写入范围不能为空".to_string()))?;

        let values = self
            .values
            .ok_or_else(|| LarkAPIError::InvalidParameter("写入数据不能为空".to_string()))?;

        let mut request = ValuesSingleWriteRequest::new(spreadsheet_token, range, values);

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

        self.service.write(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_values_single_write_request_validation() {
        // 测试空token
        let request = ValuesSingleWriteRequest::new("", "Sheet1!A1:C10", vec![]);
        assert!(request.validate().is_err());

        // 测试空范围
        let request = ValuesSingleWriteRequest::new("token", "", vec![]);
        assert!(request.validate().is_err());

        // 测试空数据
        let request = ValuesSingleWriteRequest::new("token", "Sheet1!A1:C10", vec![]);
        assert!(request.validate().is_err());

        // 测试正常请求
        let values = vec![vec![json!("test")]];
        let request = ValuesSingleWriteRequest::new("token", "Sheet1!A1:C10", values);
        assert!(request.validate().is_ok());

        // 测试行数限制
        let large_values = (0..10001).map(|_| vec![json!("test")]).collect();
        let request = ValuesSingleWriteRequest::new("token", "Sheet1!A1:C10", large_values);
        assert!(request.validate().is_err());

        // 测试列数限制
        let wide_row = (0..101).map(|_| json!("test")).collect();
        let request = ValuesSingleWriteRequest::new("token", "Sheet1!A1:C10", vec![wide_row]);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_to_request_body() {
        let values = vec![
            vec![json!("姓名"), json!("年龄")],
            vec![json!("张三"), json!(25)],
        ];

        let request = ValuesSingleWriteRequest::new("token", "Sheet1!A1:B2", values)
            .value_input_option(ValueInputOption::Raw)
            .include_values_in_response(true);

        let body = request.to_request_body().unwrap();

        assert_eq!(body["range"], "Sheet1!A1:B2");
        assert_eq!(body["valueInputOption"], "RAW");
        assert_eq!(body["includeValuesInResponse"], true);
        assert!(body["values"].is_array());
    }

    #[test]
    fn test_value_input_options() {
        let values = vec![vec![json!("test")]];

        // 测试RAW选项
        let request = ValuesSingleWriteRequest::new("token", "Sheet1!A1", values.clone())
            .value_input_option(ValueInputOption::Raw);
        let body = request.to_request_body().unwrap();
        assert_eq!(body["valueInputOption"], "RAW");

        // 测试USER_ENTERED选项
        let request = ValuesSingleWriteRequest::new("token", "Sheet1!A1", values)
            .value_input_option(ValueInputOption::UserEntered);
        let body = request.to_request_body().unwrap();
        assert_eq!(body["valueInputOption"], "USER_ENTERED");
    }

    #[test]
    fn test_parse_csv_line() {
        // 测试基本CSV解析
        let result = ValuesSingleWriteService::parse_csv_line("a,b,c", ',').unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);

        // 测试带引号的字段
        let result = ValuesSingleWriteService::parse_csv_line("a,\"b,c\",d", ',').unwrap();
        assert_eq!(result, vec!["a", "b,c", "d"]);

        // 测试转义引号
        let result = ValuesSingleWriteService::parse_csv_line("a,\"b\"\"c\",d", ',').unwrap();
        assert_eq!(result, vec!["a", "b\"c", "d"]);
    }

    #[test]
    fn test_parse_csv_to_values() {
        let csv_data = "姓名,年龄\n张三,25\n李四,30";
        let values = ValuesSingleWriteService::parse_csv_to_values(csv_data, ',').unwrap();

        assert_eq!(values.len(), 3);
        assert_eq!(values[0][0], json!("姓名"));
        assert_eq!(values[0][1], json!("年龄"));
        assert_eq!(values[1][0], json!("张三"));
        assert_eq!(values[1][1], json!("25"));
        assert_eq!(values[2][0], json!("李四"));
        assert_eq!(values[2][1], json!("30"));
    }

    #[test]
    fn test_hashmap_to_values() {
        let mut data = HashMap::new();
        data.insert("姓名".to_string(), vec![json!("张三"), json!("李四")]);
        data.insert("年龄".to_string(), vec![json!(25), json!(30)]);

        // 测试包含表头
        let values = ValuesSingleWriteService::hashmap_to_values(data.clone() true).unwrap();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0][0], json!("姓名"));
        assert_eq!(values[0][1], json!("年龄"));
        assert_eq!(values[1][0], json!("张三"));
        assert_eq!(values[1][1], json!(25));

        // 测试不包含表头
        let values = ValuesSingleWriteService::hashmap_to_values(data, false).unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0][0], json!("张三"));
        assert_eq!(values[0][1], json!(25));
    }

    #[test]
    fn test_builder_pattern() {
        let config = openlark_core::config::Config::default();
        let service = ValuesSingleWriteService::new(config);

        let values = vec![vec![json!("test")]];
        let builder = service
            .write_builder()
            .spreadsheet_token("test_token")
            .range("Sheet1!A1")
            .values(values)
            .value_input_option(ValueInputOption::Raw)
            .include_values_in_response(true);

        // 验证构建器设置
        assert_eq!(builder.spreadsheet_token.as_ref().unwrap(), "test_token");
        assert_eq!(builder.range.as_ref().unwrap(), "Sheet1!A1");
        assert!(matches!(
            builder.value_input_option,
            Some(ValueInputOption::Raw)
        ));
        assert_eq!(builder.include_values_in_response, Some(true));
    }

    #[test]
    fn test_values_single_write_service() {
        let config = openlark_core::config::Config::default();
        let service = ValuesSingleWriteService::new(config);

        assert_eq!(service.service_name(), "ValuesSingleWriteService");
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_complex_data_scenarios() {
        // 测试复杂的数据类型
        let values = vec![
            vec![
                json!("姓名"),
                json!("年龄"),
                json!("入职日期"),
                json!("是否全职"),
                json!("薪资"),
            ],
            vec![
                json!("张三"),
                json!(25),
                json!("2023-01-15"),
                json!(true),
                json!(8500.50),
            ],
            vec![
                json!("李四"),
                json!(30),
                json!("2022-06-01"),
                json!(false),
                json!(12000.00),
            ],
        ];

        let request = ValuesSingleWriteRequest::new("token", "Sheet1!A1:E3", values)
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

        let result = ValuesSingleWriteService::hashmap_to_values(data, false);
        assert!(result.is_err());

        // 测试空HashMap
        let empty_data = HashMap::new();
        let result = ValuesSingleWriteService::hashmap_to_values(empty_data, true);
        assert!(result.is_err());

        // 测试空CSV
        let result = ValuesSingleWriteService::parse_csv_to_values("", ',');
        assert!(result.is_err());
    }
}

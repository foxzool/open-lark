//! Sheets v2 批量范围读取服务
//!
//! 提供飞书电子表格v2版本的批量范围读取功能，包括：
//! - 一次性读取多个单元格范围
//! - 支持Excel风格的范围格式（如 Sheet1!A1:B10）
//! - 高效的批量数据获取，减少网络请求次数
//! - 企业级错误处理和数据验证
//! - 支持数值格式、日期渲染等选项

use serde_json::Value;
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

/// 批量范围读取请求
#[derive(Clone, Debug)]
pub struct BatchReadRangesRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 要读取的范围列表，支持Excel风格格式
    /// 例如: ["Sheet1!A1:B10", "Sheet2!A1:C5", "'Sheet 3'!D1:F20"]
    pub ranges: Vec<String>,
    /// 数值格式化选项
    pub value_render_option: Option<ValueRenderOption>,
    /// 日期/时间渲染选项
    pub date_render_option: Option<DateRenderOption>,
    /// 是否返回公式
    pub formula_render_option: Option<FormulaRenderOption>,
}

/// 数值渲染选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueRenderOption {
    /// 未格式化值（默认）
    #[serde(rename = "UnformattedValue")]
    UnformattedValue,
    /// 用户输入格式
    #[serde(rename = "UserEnteredFormat")]
    UserEnteredFormat,
    /// 格式化值
    #[serde(rename = "FormattedValue")]
    FormattedValue,
}

/// 日期渲染选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DateRenderOption {
    /// 序列号（默认）
    #[serde(rename = "SerialNumber")]
    SerialNumber,
    /// 格式化字符串
    #[serde(rename = "FormattedString")]
    FormattedString,
}

/// 公式渲染选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormulaRenderOption {
    /// 渲染计算结果
    #[serde(rename = "CalculatedValue")]
    CalculatedValue,
    /// 渲染公式本身
    #[serde(rename = "Formula")]
    Formula,
}

impl Default for BatchReadRangesRequest {
    fn default() -> Self {
        Self {
            spreadsheet_token: String::new(),
            ranges: vec![],
            value_render_option: None,
            date_render_option: None,
            formula_render_option: None,
        }
    }
}

impl BatchReadRangesRequest {
    /// 创建新的批量范围读取请求
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            
        }
    }

    /// 添加要读取的范围
    pub fn add_range(mut self, range: impl Into<String>) -> Self {
        self.ranges.push(range.into());
        self
    }

    /// 批量添加要读取的范围
    pub fn ranges(mut self, ranges: Vec<impl Into<String>>) -> Self {
        self.ranges = ranges.into_iter().map(|r| r.into()).collect();
        self
    }

    /// 设置数值渲染选项
    pub fn value_render_option(mut self, option: ValueRenderOption) -> Self {
        self.value_render_option = Some(option);
        self
    }

    /// 设置日期渲染选项
    pub fn date_render_option(mut self, option: DateRenderOption) -> Self {
        self.date_render_option = Some(option);
        self
    }

    /// 设置公式渲染选项
    pub fn formula_render_option(mut self, option: FormulaRenderOption) -> Self {
        self.formula_render_option = Some(option);
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
                "至少需要指定一个读取范围".to_string(),
            ));
        }

        if self.ranges.len() > 10 {
            return Err(LarkAPIError::InvalidParameter(
                "一次最多只能读取10个范围".to_string(),
            ));
        }

        // 验证每个范围格式
        for (index, range) in self.ranges.iter().enumerate() {
            if range.trim().is_empty() {
                return Err(LarkAPIError::InvalidParameter(format!(
                    "范围{}不能为空",
                    index + 1
                )));
            }

            // 基本的范围格式验证（A1:B10格式或命名范围）
            let range_upper = range.to_uppercase();
            if !range_upper.contains('!')
                && !range_upper
                    .matches(|c: char| c.is_ascii_alphabetic())
                    .next()
                    .is_some()
            {
                return Err(LarkAPIError::InvalidParameter(format!(
                    "范围{}格式不正确，应为SheetName!A1:B10格式或命名范围",
                    index + 1
                )));
            }
        }

        Ok(())
    }

    /// 构建查询参数
    pub fn build_query_params(&self) -> String {
        let mut params = vec![];

        // 添加范围参数
        for range in &self.ranges {
            params.push(format!("ranges={}", urlencoding::encode(range)));
        }

        // 添加数值渲染选项
        if let Some(option) = &self.value_render_option {
            let option_str = match option {
                ValueRenderOption::UnformattedValue => "UnformattedValue",
                ValueRenderOption::UserEnteredFormat => "UserEnteredFormat",
                ValueRenderOption::FormattedValue => "FormattedValue",
            };
            params.push(format!("valueRenderOption={}", option_str));
        }

        // 添加日期渲染选项
        if let Some(option) = &self.date_render_option {
            let option_str = match option {
                DateRenderOption::SerialNumber => "SerialNumber",
                DateRenderOption::FormattedString => "FormattedString",
            };
            params.push(format!("dateRenderOption={}", option_str));
        }

        // 添加公式渲染选项
        if let Some(option) = &self.formula_render_option {
            let option_str = match option {
                FormulaRenderOption::CalculatedValue => "CalculatedValue",
                FormulaRenderOption::Formula => "Formula",
            };
            params.push(format!("formulaRenderOption={}", option_str));
        }

        params.join("&")
    }
}

/// 单个范围的读取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeData {
    /// 范围标识
    pub range: String,
    /// 该范围的数据
    pub values: Option<Vec<Vec<String>>>,
    /// 范围的行数
    pub major_dimension: Option<String>,
}

/// 批量范围读取响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchReadRangesResponse {
    /// 所有范围的数据
    pub value_ranges: Vec<RangeData>,
    /// 电子表格ID
    pub spreadsheet_id: Option<String>,
}

/// API响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchReadRangesResponseBody {
    /// 读取的范围数据
    pub data: BatchReadRangesResponse,
}

/// 基础API响应
// 移除重复的BaseResponse定义，使用openlark_core中的版本

/// 批量范围读取服务
#[derive(Clone, Debug)]
pub struct BatchReadRangesService {
    config: Config,
}

impl BatchReadRangesService {
    /// 创建批量范围读取服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量读取多个范围的数据
    ///
    /// # 参数
    /// - `request`: 批量范围读取请求
    ///
    /// # 返回
    /// 所有范围的数据
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::batch_read_ranges::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = BatchReadRangesService::new(config);
    ///
    /// // 读取多个范围
    /// let request = BatchReadRangesRequest::new("spreadsheet_token")
    ///     .add_range("Sheet1!A1:C10")
    ///     .add_range("Sheet2!A1:E20")
    ///     .add_range("'Sheet 3'!F1:H30")
    ///     .value_render_option(ValueRenderOption::FormattedValue)
    ///     .date_render_option(DateRenderOption::FormattedString);
    ///
    /// // 或使用ranges方法批量添加
    /// let request = BatchReadRangesRequest::new("spreadsheet_token")
    ///     .ranges(vec![
    ///         "Sheet1!A1:C10",
    ///         "Sheet2!A1:E20",
    ///         "Summary!A1:G15"
    ///     ]);
    /// ```
    pub async fn batch_read(
        &self,
        request: BatchReadRangesRequest,
    ) -> SDKResult<BatchReadRangesResponse> {
        // 验证请求参数
        request.validate()?;

        // 构建查询参数
        let query_params = request.build_query_params();

        // 构建URL
        let url = if query_params.is_empty() {
            format!(
                "{}/open-apis/sheets/v2/spreadsheets/{}/values_batch_get",
                self.config.base_url, request.spreadsheet_token
            )
        } else {
            format!(
                "{}/open-apis/sheets/v2/spreadsheets/{}/values_batch_get?{}",
                self.config.base_url, request.spreadsheet_token, query_params
            )
        };

        // 发送HTTP请求
        let response = self
            .config
            .transport
            .get(&url)
            .send()
            .await
            .map_err(|e| LarkAPIError::NetworkError(format!("网络请求失败: {}", e)))?;

        // 处理响应
        if response.status().is_success() {
            let base_response: Response<BatchReadRangesResponseBody> = response
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
                "批量范围读取失败".to_string(),
            ))
        }
    }

    /// 创建批量范围读取构建器
    pub fn batch_read_builder(&self) -> BatchReadRangesBuilder {
        BatchReadRangesBuilder::new(self.clone())
    }

    /// 快速读取单个范围
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `range`: 范围字符串，如 "Sheet1!A1:C10"
    ///
    /// # 返回
    /// 指定范围的数据
    pub async fn read_range(
        &self,
        spreadsheet_token: impl Into<String>,
        range: impl Into<String>,
    ) -> SDKResult<RangeData> {
        let request = BatchReadRangesRequest::new(spreadsheet_token).add_range(range);

        let response = self.batch_read(request).await?;

        // 返回第一个范围的数据
        response
            .value_ranges
            .into_iter()
            .next()
            .ok_or_else(|| LarkAPIError::APIError(-1, "未找到范围数据".to_string()))
    }
}

impl Service for BatchReadRangesService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "BatchReadRangesService"
    }
}

/// 批量范围读取构建器
pub struct BatchReadRangesBuilder {
    service: BatchReadRangesService,
    spreadsheet_token: Option<String>,
    ranges: Vec<String>,
    value_render_option: Option<ValueRenderOption>,
    date_render_option: Option<DateRenderOption>,
    formula_render_option: Option<FormulaRenderOption>,
}

impl BatchReadRangesBuilder {
    /// 创建新的构建器
    pub fn new(service: BatchReadRangesService) -> Self {
        Self {
            service,
            spreadsheet_token: None,
            ranges: vec![],
            value_render_option: None,
            date_render_option: None,
            formula_render_option: None,
        }
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = Some(token.into());
        self
    }

    /// 添加要读取的范围
    pub fn add_range(mut self, range: impl Into<String>) -> Self {
        self.ranges.push(range.into());
        self
    }

    /// 批量添加要读取的范围
    pub fn ranges(mut self, ranges: Vec<impl Into<String>>) -> Self {
        self.ranges.extend(ranges.into_iter().map(|r| r.into()));
        self
    }

    /// 设置数值渲染选项
    pub fn value_render_option(mut self, option: ValueRenderOption) -> Self {
        self.value_render_option = Some(option);
        self
    }

    /// 设置日期渲染选项
    pub fn date_render_option(mut self, option: DateRenderOption) -> Self {
        self.date_render_option = Some(option);
        self
    }

    /// 设置公式渲染选项
    pub fn formula_render_option(mut self, option: FormulaRenderOption) -> Self {
        self.formula_render_option = Some(option);
        self
    }

    /// 执行读取操作
    pub async fn execute(self) -> SDKResult<BatchReadRangesResponse> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::InvalidParameter("电子表格token不能为空".to_string()))?;

        if self.ranges.is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "至少需要指定一个读取范围".to_string(),
            ));
        }

        let mut request = BatchReadRangesRequest::new(spreadsheet_token);
        request.ranges = self.ranges;

        if let Some(option) = self.value_render_option {
            request = request.value_render_option(option);
        }

        if let Some(option) = self.date_render_option {
            request = request.date_render_option(option);
        }

        if let Some(option) = self.formula_render_option {
            request = request.formula_render_option(option);
        }

        self.service.batch_read(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_read_ranges_request_validation() {
        // 测试空token
        let request = BatchReadRangesRequest::new("").add_range("Sheet1!A1:C10");
        assert!(request.validate().is_err());

        // 测试空范围列表
        let request = BatchReadRangesRequest::new("token");
        assert!(request.validate().is_err());

        // 测试正常请求
        let request = BatchReadRangesRequest::new("token")
            .add_range("Sheet1!A1:C10")
            .add_range("Sheet2!A1:E20");
        assert!(request.validate().is_ok());

        // 测试范围数量限制
        let ranges = (0..11)
            .map(|i| format!("Sheet1!A{}:C{}", i * 10 + 1, i * 10 + 10))
            .collect();
        let request = BatchReadRangesRequest::new("token").ranges(ranges);
        assert!(request.validate().is_err());

        // 测试空范围
        let request = BatchReadRangesRequest::new("token")
            .add_range("Sheet1!A1:C10")
            .add_range("");
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_build_query_params() {
        let request = BatchReadRangesRequest::new("token")
            .add_range("Sheet1!A1:C10")
            .add_range("'Sheet 2'!A1:E20")
            .value_render_option(ValueRenderOption::FormattedValue)
            .date_render_option(DateRenderOption::FormattedString)
            .formula_render_option(FormulaRenderOption::CalculatedValue);

        let params = request.build_query_params();

        assert!(params.contains("ranges=Sheet1%21A1%3AC10"));
        assert!(params.contains("ranges=%27Sheet+2%27%21A1%3AE20"));
        assert!(params.contains("valueRenderOption=FormattedValue"));
        assert!(params.contains("dateRenderOption=FormattedString"));
        assert!(params.contains("formulaRenderOption=CalculatedValue"));
    }

    #[test]
    fn test_value_render_options() {
        let request = BatchReadRangesRequest::new("token")
            .add_range("Sheet1!A1:C10")
            .value_render_option(ValueRenderOption::UserEnteredFormat);

        let params = request.build_query_params();
        assert!(params.contains("valueRenderOption=UserEnteredFormat"));
    }

    #[test]
    fn test_date_render_options() {
        let request = BatchReadRangesRequest::new("token")
            .add_range("Sheet1!A1:C10")
            .date_render_option(DateRenderOption::SerialNumber);

        let params = request.build_query_params();
        assert!(params.contains("dateRenderOption=SerialNumber"));
    }

    #[test]
    fn test_formula_render_options() {
        let request = BatchReadRangesRequest::new("token")
            .add_range("Sheet1!A1:C10")
            .formula_render_option(FormulaRenderOption::Formula);

        let params = request.build_query_params();
        assert!(params.contains("formulaRenderOption=Formula"));
    }

    #[test]
    fn test_builder_pattern() {
        let config = openlark_core::config::Config::default();
        let service = BatchReadRangesService::new(config);

        let builder = service
            .batch_read_builder()
            .spreadsheet_token("test_token")
            .add_range("Sheet1!A1:C10")
            .add_range("Sheet2!A1:E20")
            .value_render_option(ValueRenderOption::FormattedValue)
            .date_render_option(DateRenderOption::FormattedString);

        // 验证构建器设置
        assert_eq!(builder.spreadsheet_token.as_ref().unwrap(), "test_token");
        assert_eq!(builder.ranges.len(), 2);
        assert!(matches!(
            builder.value_render_option,
            Some(ValueRenderOption::FormattedValue)
        ));
        assert!(matches!(
            builder.date_render_option,
            Some(DateRenderOption::FormattedString)
        ));
    }

    #[test]
    fn test_batch_read_ranges_service() {
        let config = openlark_core::config::Config::default();
        let service = BatchReadRangesService::new(config);

        assert_eq!(service.service_name(), "BatchReadRangesService");
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_range_format_validation() {
        // 测试有效的范围格式
        let valid_ranges = vec![
            "Sheet1!A1:C10",
            "'Sheet 2'!A1:E20",
            "Data!$A$1:$C$10",
            "Summary!A:A",
            "Report!1:10",
            "NamedRange",
        ];

        for range in valid_ranges {
            let request = BatchReadRangesRequest::new("token").add_range(range);
            assert!(
                request.validate().is_ok(),
                "Range '{}' should be valid",
                range
            );
        }
    }

    #[test]
    fn test_complex_range_scenarios() {
        // 测试包含特殊字符的范围
        let request = BatchReadRangesRequest::new("token")
            .add_range("2023年度'Q1'!A1:Z100")
            .add_range("Customer_Data!A1:AA500")
            .value_render_option(ValueRenderOption::FormattedValue)
            .date_render_option(DateRenderOption::FormattedString)
            .formula_render_option(FormulaRenderOption::Formula);

        assert!(request.validate().is_ok());

        let params = request.build_query_params();
        assert!(params.len() > 100); // URL编码后的参数应该很长
    }
}

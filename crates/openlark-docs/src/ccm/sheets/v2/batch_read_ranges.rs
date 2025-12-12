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
    standard_response::Response,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use openlark_core::trait_system::Service;
// use openlark_core::SDKResult;

/// 批量范围读取请求
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
    pub fn build(&self) -> Self {
        self.clone()
    }
}

//! Sheets电子表格条件格式服务 v3
//!
//! 提供飞书电子表格v3版本的条件格式管理功能，包括：
//! - 条件格式规则设置和删除
//! - 多种条件类型支持（数据条、色阶、图标集）
//! - 自定义条件表达式
//! - 条件格式优先级管理

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    error::LarkAPIError,
    http::Transport,
};

// 使用统一类型定义
use super::Range;

use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 条件格式规则类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionalFormatType {
    /// 数据条
    #[serde(rename = "DATA_BAR")]
    DataBar,
    /// 色阶
    #[serde(rename = "COLOR_SCALE")]
    ColorScale,
    /// 图标集
    #[serde(rename = "ICON_SET")]
    IconSet,
    /// 自定义公式
    #[serde(rename = "FORMULA")]
    Formula,
    /// 单一条件（等于、大于等）
    #[serde(rename = "SINGLE_CONDITION")]
    SingleCondition,
}

/// 条件格式操作符
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionalFormatOperator {
    /// 等于
    #[serde(rename = "EQUAL")]
    Equal,
    /// 不等于
    #[serde(rename = "NOT_EQUAL")]
    NotEqual,
    /// 大于
    #[serde(rename = "GREATER")]
    Greater,
    /// 大于等于
    #[serde(rename = "GREATER_EQUAL")]
    GreaterEqual,
    /// 小于
    #[serde(rename = "LESS")]
    Less,
    /// 小于等于
    #[serde(rename = "LESS_EQUAL")]
    LessEqual,
    /// 包含
    #[serde(rename = "CONTAINS")]
    Contains,
    /// 不包含
    #[serde(rename = "NOT_CONTAINS")]
    NotContains,
    /// 开始于
    #[serde(rename = "STARTS_WITH")]
    StartsWith,
    /// 结束于
    #[serde(rename = "ENDS_WITH")]
    EndsWith,
    /// 为空
    #[serde(rename = "IS_EMPTY")]
    IsEmpty,
    /// 不为空
    #[serde(rename = "IS_NOT_EMPTY")]
    IsNotEmpty,
}

/// 颜色定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    /// 红色分量 (0-255)
    pub red: u8,
    /// 绿色分量 (0-255)
    pub green: u8,
    /// 蓝色分量 (0-255)
    pub blue: u8,
    /// 透明度 (0-1)
    pub alpha: f32,
}

impl Color {
    /// 创建RGB颜色
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 1.0,
        }
    }

    /// 创建RGBA颜色
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: alpha.clamp(0.0, 1.0),
        }
    }

    /// 创建十六进制颜色
    pub fn hex(hex: &str) -> Result<Self, LarkAPIError> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(LarkAPIError::IllegalParamError(
                "无效的十六进制颜色格式".to_string(),
            ));
        }

        let rgb = u32::from_str_radix(hex, 16)
            .map_err(|_| LarkAPIError::IllegalParamError("无效的十六进制颜色".to_string()))?;

        Ok(Self::rgb(
            ((rgb >> 16) & 0xFF) as u8,
            ((rgb >> 8) & 0xFF) as u8,
            (rgb & 0xFF) as u8,
        ))
    }
}

/// 预定义颜色
impl Color {
    /// 红色
    pub fn red() -> Self {
        Self::rgb(255, 0, 0)
    }
    /// 绿色
    pub fn green() -> Self {
        Self::rgb(0, 255, 0)
    }
    /// 蓝色
    pub fn blue() -> Self {
        Self::rgb(0, 0, 255)
    }
    /// 黄色
    pub fn yellow() -> Self {
        Self::rgb(255, 255, 0)
    }
    /// 橙色
    pub fn orange() -> Self {
        Self::rgb(255, 165, 0)
    }
    /// 紫色
    pub fn purple() -> Self {
        Self::rgb(128, 0, 128)
    }
    /// 灰色
    pub fn gray() -> Self {
        Self::rgb(128, 128, 128)
    }
    /// 浅灰色
    pub fn light_gray() -> Self {
        Self::rgb(211, 211, 211)
    }
    /// 白色
    pub fn white() -> Self {
        Self::rgb(255, 255, 255)
    }
    /// 黑色
    pub fn black() -> Self {
        Self::rgb(0, 0, 0)
    }
}

/// 数据条配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBarFormat {
    /// 最小值颜色
    #[serde(rename = "min_color")]
    pub min_color: Color,
    /// 最大值颜色
    #[serde(rename = "max_color")]
    pub max_color: Color,
    /// 是否显示数据条数值
    #[serde(rename = "show_value")]
    pub show_value: Option<bool>,
    /// 数据条方向
    #[serde(rename = "direction")]
    pub direction: Option<String>,
}

impl DataBarFormat {
    /// 创建数据条格式
    pub fn new(min_color: Color, max_color: Color) -> Self {
        Self {
            min_color,
            max_color,
            show_value: None,
            direction: None,
        }
    }

    /// 设置是否显示数值
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = Some(show);
        self
    }

    /// 设置方向
    pub fn direction(mut self, direction: String) -> Self {
        self.direction = Some(direction);
        self
    }
}

/// 色阶配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScaleFormat {
    /// 颜色渐变点
    #[serde(rename = "colors")]
    pub colors: Vec<ColorScalePoint>,
}

/// 色阶点定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScalePoint {
    /// 颜色值
    #[serde(rename = "color")]
    pub color: Color,
    /// 类型
    #[serde(rename = "type")]
    pub point_type: String,
    /// 值
    #[serde(rename = "value")]
    pub value: String,
}

impl ColorScaleFormat {
    /// 创建双色阶
    pub fn two_color(min_color: Color, max_color: Color) -> Self {
        Self {
            colors: vec![
                ColorScalePoint {
                    color: min_color,
                    point_type: "min".to_string(),
                    value: "".to_string(),
                },
                ColorScalePoint {
                    color: max_color,
                    point_type: "max".to_string(),
                    value: "".to_string(),
                },
            ],
        }
    }

    /// 创建三色阶
    pub fn three_color(min_color: Color, mid_color: Color, max_color: Color) -> Self {
        Self {
            colors: vec![
                ColorScalePoint {
                    color: min_color,
                    point_type: "min".to_string(),
                    value: "".to_string(),
                },
                ColorScalePoint {
                    color: mid_color,
                    point_type: "percentile".to_string(),
                    value: "50".to_string(),
                },
                ColorScalePoint {
                    color: max_color,
                    point_type: "max".to_string(),
                    value: "".to_string(),
                },
            ],
        }
    }

    /// 添加自定义颜色点
    pub fn add_point(mut self, color: Color, point_type: String, value: String) -> Self {
        self.colors.push(ColorScalePoint {
            color,
            point_type,
            value,
        });
        self
    }
}

/// 图标集配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconSetFormat {
    /// 图标集样式
    #[serde(rename = "icon_set")]
    pub icon_set: String,
    /// 图标规则
    #[serde(rename = "icons")]
    pub icons: Vec<IconRule>,
}

/// 图标规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconRule {
    /// 图标索引
    #[serde(rename = "index")]
    pub index: u32,
    /// 操作符
    #[serde(rename = "operator")]
    pub operator: ConditionalFormatOperator,
    /// 值
    #[serde(rename = "value")]
    pub value: String,
}

impl IconSetFormat {
    /// 创建三向箭头图标集
    pub fn three_arrows() -> Self {
        Self {
            icon_set: "3Arrows".to_string(),
            icons: vec![
                IconRule {
                    index: 0,
                    operator: ConditionalFormatOperator::Greater,
                    value: "67".to_string(),
                },
                IconRule {
                    index: 1,
                    operator: ConditionalFormatOperator::Greater,
                    value: "33".to_string(),
                },
                IconRule {
                    index: 2,
                    operator: ConditionalFormatOperator::LessEqual,
                    value: "33".to_string(),
                },
            ],
        }
    }

    /// 创建三色交通灯
    pub fn three_traffic_lights() -> Self {
        Self {
            icon_set: "3TrafficLights1".to_string(),
            icons: vec![
                IconRule {
                    index: 0,
                    operator: ConditionalFormatOperator::Greater,
                    value: "67".to_string(),
                },
                IconRule {
                    index: 1,
                    operator: ConditionalFormatOperator::Greater,
                    value: "33".to_string(),
                },
                IconRule {
                    index: 2,
                    operator: ConditionalFormatOperator::LessEqual,
                    value: "33".to_string(),
                },
            ],
        }
    }
}

/// 单一条件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleConditionFormat {
    /// 操作符
    #[serde(rename = "operator")]
    pub operator: ConditionalFormatOperator,
    /// 比较值
    #[serde(rename = "value")]
    pub value: String,
    /// 背景颜色
    #[serde(rename = "background_color")]
    pub background_color: Option<Color>,
    /// 文字颜色
    #[serde(rename = "text_color")]
    pub text_color: Option<Color>,
}

impl SingleConditionFormat {
    /// 创建单一条件格式
    pub fn new(operator: ConditionalFormatOperator, value: String) -> Self {
        Self {
            operator,
            value,
            background_color: None,
            text_color: None,
        }
    }

    /// 设置背景颜色
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    /// 设置文字颜色
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }
}

/// 公式条件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaFormat {
    /// 公式表达式
    #[serde(rename = "formula")]
    pub formula: String,
    /// 背景颜色
    #[serde(rename = "background_color")]
    pub background_color: Option<Color>,
    /// 文字颜色
    #[serde(rename = "text_color")]
    pub text_color: Option<Color>,
}

impl FormulaFormat {
    /// 创建公式条件格式
    pub fn new(formula: String) -> Self {
        Self {
            formula,
            background_color: None,
            text_color: None,
        }
    }

    /// 设置背景颜色
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    /// 设置文字颜色
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }
}

/// 条件格式规则配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConditionalFormatRule {
    /// 数据条
    #[serde(rename = "DATA_BAR")]
    DataBar {
        #[serde(flatten)]
        config: DataBarFormat,
    },
    /// 色阶
    #[serde(rename = "COLOR_SCALE")]
    ColorScale {
        #[serde(flatten)]
        config: ColorScaleFormat,
    },
    /// 图标集
    #[serde(rename = "ICON_SET")]
    IconSet {
        #[serde(flatten)]
        config: IconSetFormat,
    },
    /// 单一条件
    #[serde(rename = "SINGLE_CONDITION")]
    SingleCondition {
        #[serde(flatten)]
        config: SingleConditionFormat,
    },
    /// 公式
    #[serde(rename = "FORMULA")]
    Formula {
        #[serde(flatten)]
        config: FormulaFormat,
    },
}

impl ConditionalFormatRule {
    /// 创建数据条规则
    pub fn data_bar(min_color: Color, max_color: Color) -> Self {
        Self::DataBar {
            config: DataBarFormat::new(min_color, max_color),
        }
    }

    /// 创建双色阶规则
    pub fn two_color_scale(min_color: Color, max_color: Color) -> Self {
        Self::ColorScale {
            config: ColorScaleFormat::two_color(min_color, max_color),
        }
    }

    /// 创建三色阶规则
    pub fn three_color_scale(min_color: Color, mid_color: Color, max_color: Color) -> Self {
        Self::ColorScale {
            config: ColorScaleFormat::three_color(min_color, mid_color, max_color),
        }
    }

    /// 创建三向箭头图标集规则
    pub fn three_arrows() -> Self {
        Self::IconSet {
            config: IconSetFormat::three_arrows(),
        }
    }

    /// 创建三色交通灯规则
    pub fn three_traffic_lights() -> Self {
        Self::IconSet {
            config: IconSetFormat::three_traffic_lights(),
        }
    }

    /// 创建单一条件规则
    pub fn single_condition(operator: ConditionalFormatOperator, value: String) -> Self {
        Self::SingleCondition {
            config: SingleConditionFormat::new(operator, value),
        }
    }

    /// 创建公式规则
    pub fn formula(formula: String) -> Self {
        Self::Formula {
            config: FormulaFormat::new(formula),
        }
    }
}

/// 条件格式设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalFormat {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 规则范围
    #[serde(rename = "range")]
    pub range: Range,
    /// 条件格式规则
    #[serde(rename = "rule")]
    pub rule: ConditionalFormatRule,
}

impl ConditionalFormat {
    /// 创建条件格式
    pub fn new(sheet_id: String, range: Range, rule: ConditionalFormatRule) -> Self {
        Self {
            sheet_id,
            range,
            rule,
        }
    }

    /// 验证条件格式配置
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.sheet_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "工作表ID不能为空".to_string(),
            ));
        }

        // 验证范围格式
        if self.range.is_empty() {
            return Err(LarkAPIError::IllegalParamError("范围格式无效".to_string()));
        }

        Ok(())
    }
}

/// 设置条件格式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetConditionalFormatRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 条件格式设置
    #[serde(rename = "conditional_format")]
    pub conditional_format: ConditionalFormat,
}

impl SetConditionalFormatRequest {
    /// 创建设置条件格式请求
    pub fn new(spreadsheet_token: String, conditional_format: ConditionalFormat) -> Self {
        Self {
            spreadsheet_token,
            conditional_format,
        }
    }

    /// 构建器模式
    pub fn builder() -> SetConditionalFormatRequestBuilder {
        SetConditionalFormatRequestBuilder::default()
    }
}

/// 设置条件格式请求构建器
#[derive(Debug, Default)]
pub struct SetConditionalFormatRequestBuilder {
    spreadsheet_token: Option<String>,
    sheet_id: Option<String>,
    range: Option<Range>,
    rule: Option<ConditionalFormatRule>,
}

impl SetConditionalFormatRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(sheet_id);
        self
    }

    /// 设置范围
    pub fn range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置规则
    pub fn rule(mut self, rule: ConditionalFormatRule) -> Self {
        self.rule = Some(rule);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<SetConditionalFormatRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let sheet_id = self
            .sheet_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("工作表ID不能为空".to_string()))?;

        let range = self
            .range
            .ok_or_else(|| LarkAPIError::IllegalParamError("范围不能为空".to_string()))?;

        let rule = self
            .rule
            .ok_or_else(|| LarkAPIError::IllegalParamError("条件格式规则不能为空".to_string()))?;

        let conditional_format = ConditionalFormat::new(sheet_id, range, rule);
        conditional_format.validate()?;

        Ok(SetConditionalFormatRequest {
            spreadsheet_token,
            conditional_format,
        })
    }
}

/// 设置条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetConditionalFormatResponse {
    /// 条件格式ID
    #[serde(rename = "conditional_format_id")]
    pub conditional_format_id: String,
    /// 条件格式设置
    #[serde(rename = "conditional_format")]
    pub conditional_format: ConditionalFormat,
}

// 实现API响应特征
impl ApiResponseTrait for SetConditionalFormatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除条件格式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteConditionalFormatRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 条件格式ID
    #[serde(rename = "conditional_format_id")]
    pub conditional_format_id: String,
}

impl DeleteConditionalFormatRequest {
    /// 创建删除条件格式请求
    pub fn new(spreadsheet_token: String, conditional_format_id: String) -> Self {
        Self {
            spreadsheet_token,
            conditional_format_id,
        }
    }

    /// 构建器模式
    pub fn builder() -> DeleteConditionalFormatRequestBuilder {
        DeleteConditionalFormatRequestBuilder::default()
    }
}

/// 删除条件格式请求构建器
#[derive(Debug, Default)]
pub struct DeleteConditionalFormatRequestBuilder {
    spreadsheet_token: Option<String>,
    conditional_format_id: Option<String>,
}

impl DeleteConditionalFormatRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置条件格式ID
    pub fn conditional_format_id(mut self, conditional_format_id: String) -> Self {
        self.conditional_format_id = Some(conditional_format_id);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<DeleteConditionalFormatRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let conditional_format_id = self
            .conditional_format_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("条件格式ID不能为空".to_string()))?;

        Ok(DeleteConditionalFormatRequest {
            spreadsheet_token,
            conditional_format_id,
        })
    }
}

/// 删除条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteConditionalFormatResponse {
    /// 是否成功删除
    pub success: bool,
}

impl ApiResponseTrait for DeleteConditionalFormatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Sheets电子表格条件格式服务 v3
#[derive(Clone, Debug)]
pub struct ConditionalFormatService {
    config: openlark_core::config::Config,
}

impl ConditionalFormatService {
    /// 创建条件格式服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 设置条件格式
    ///
    /// 在指定工作表和范围中设置条件格式规则，支持多种条件类型。
    ///
    /// # 参数
    /// - `request`: 设置条件格式请求
    ///
    /// # 返回
    /// 返回设置后的条件格式信息，包括条件格式ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::conditional_format::*;
    /// use open_lark::service::sheets::v3::models::Range;
    ///
    /// // 创建范围对象
    /// let range = Range::from("A1".to_string(), "A10".to_string());
    ///
    /// // 创建数据条规则
    /// let rule = ConditionalFormatRule::data_bar(
    ///     Color::green(),
    ///     Color::red()
    /// );
    ///
    /// // 设置条件格式
    /// let request = SetConditionalFormatRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .sheet_id("sheet_id".to_string())
    ///     .range(range)
    ///     .rule(rule)
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.set_conditional_format(&request).await?;
    /// ```
    pub async fn set_conditional_format(
        &self,
        request: &SetConditionalFormatRequest,
    ) -> openlark_core::error::SDKResult<SetConditionalFormatResponse> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/conditional_format",
            self.config.base_url, request.spreadsheet_token
        );

        // 创建HTTP请求
        let api_request = ApiRequest::with_method_and_path(Method::POST, &url);

        // 发送请求并获取响应
        let response =
            Transport::<SetConditionalFormatResponse>::request(api_request, &self.config, None)
                .await?;

        // 检查响应是否成功
        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 删除条件格式
    ///
    /// 删除指定的条件格式规则。
    ///
    /// # 参数
    /// - `request`: 删除条件格式请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::conditional_format::*;
    ///
    /// // 删除条件格式
    /// let request = DeleteConditionalFormatRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .conditional_format_id("format_id".to_string())
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.delete_conditional_format(&request).await?;
    /// assert!(response.success);
    /// ```
    pub async fn delete_conditional_format(
        &self,
        request: &DeleteConditionalFormatRequest,
    ) -> openlark_core::error::SDKResult<DeleteConditionalFormatResponse> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/conditional_format/{}",
            self.config.base_url, request.spreadsheet_token, request.conditional_format_id
        );

        // 发送HTTP请求
        let client = reqwest::Client::new();
        let response = client
            .delete(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| LarkAPIError::RequestError(e.to_string()))?;

        let delete_response: openlark_core::api::Response<DeleteConditionalFormatResponse> =
            serde_json::from_str(
                &response
                    .text()
                    .await
                    .map_err(|e| LarkAPIError::RequestError(e.to_string()))?,
            )
            .map_err(|e| LarkAPIError::DeserializeError(e.to_string()))?;

        if delete_response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: delete_response.code(),
                msg: delete_response.msg().to_string(),
                error: None,
            });
        }

        delete_response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 设置条件格式构建器
    pub fn set_conditional_format_builder(&self) -> SetConditionalFormatRequestBuilder {
        SetConditionalFormatRequestBuilder::default()
    }

    /// 删除条件格式构建器
    pub fn delete_conditional_format_builder(&self) -> DeleteConditionalFormatRequestBuilder {
        DeleteConditionalFormatRequestBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(255, 0, 0);
        assert_eq!(color.red, 255);
        assert_eq!(color.green, 0);
        assert_eq!(color.blue, 0);
        assert_eq!(color.alpha, 1.0);

        let rgba = Color::rgba(255, 0, 0, 0.5);
        assert_eq!(rgba.alpha, 0.5);

        let hex_color = Color::hex("#FF0000").unwrap();
        assert_eq!(hex_color.red, 255);
        assert_eq!(hex_color.green, 0);
        assert_eq!(hex_color.blue, 0);
    }

    #[test]
    fn test_predefined_colors() {
        assert_eq!(Color::red().red, 255);
        assert_eq!(Color::red().green, 0);
        assert_eq!(Color::red().blue, 0);

        assert_eq!(Color::green().red, 0);
        assert_eq!(Color::green().green, 255);
        assert_eq!(Color::green().blue, 0);

        assert_eq!(Color::blue().red, 0);
        assert_eq!(Color::blue().green, 0);
        assert_eq!(Color::blue().blue, 255);
    }

    #[test]
    fn test_data_bar_format() {
        let min_color = Color::green();
        let max_color = Color::red();
        let data_bar = DataBarFormat::new(min_color, max_color)
            .show_value(true)
            .direction("left-to-right".to_string());

        assert_eq!(data_bar.min_color.red, 0);
        assert_eq!(data_bar.max_color.red, 255);
        assert_eq!(data_bar.show_value, Some(true));
        assert_eq!(data_bar.direction, Some("left-to-right".to_string()));
    }

    #[test]
    fn test_color_scale_format() {
        let min_color = Color::blue();
        let max_color = Color::red();
        let color_scale = ColorScaleFormat::two_color(min_color, max_color);
        assert_eq!(color_scale.colors.len(), 2);

        let mid_color = Color::yellow();
        let three_color = ColorScaleFormat::three_color(min_color, mid_color, max_color);
        assert_eq!(three_color.colors.len(), 3);

        let custom = color_scale.add_point(Color::green(), "number".to_string(), "50".to_string());
        assert_eq!(custom.colors.len(), 3);
    }

    #[test]
    fn test_icon_set_format() {
        let arrows = IconSetFormat::three_arrows();
        assert_eq!(arrows.icon_set, "3Arrows");
        assert_eq!(arrows.icons.len(), 3);

        let traffic_lights = IconSetFormat::three_traffic_lights();
        assert_eq!(traffic_lights.icon_set, "3TrafficLights1");
        assert_eq!(traffic_lights.icons.len(), 3);
    }

    #[test]
    fn test_single_condition_format() {
        let condition =
            SingleConditionFormat::new(ConditionalFormatOperator::Greater, "100".to_string())
                .background_color(Color::red())
                .text_color(Color::white());

        assert!(matches!(
            condition.operator,
            ConditionalFormatOperator::Greater
        ));
        assert_eq!(condition.value, "100");
        assert!(condition.background_color.is_some());
        assert!(condition.text_color.is_some());
    }

    #[test]
    fn test_formula_format() {
        let formula = FormulaFormat::new("=A1>B1".to_string())
            .background_color(Color::yellow())
            .text_color(Color::black());

        assert_eq!(formula.formula, "=A1>B1");
        assert!(formula.background_color.is_some());
        assert!(formula.text_color.is_some());
    }

    #[test]
    fn test_conditional_format_rule_creation() {
        // 测试数据条规则
        let data_bar_rule = ConditionalFormatRule::data_bar(Color::green(), Color::red());
        if let ConditionalFormatRule::DataBar { config } = data_bar_rule {
            assert_eq!(config.min_color.red, 0);
            assert_eq!(config.max_color.red, 255);
        }

        // 测试色阶规则
        let color_scale_rule =
            ConditionalFormatRule::two_color_scale(Color::blue(), Color::yellow());
        if let ConditionalFormatRule::ColorScale { config } = color_scale_rule {
            assert_eq!(config.colors.len(), 2);
        }

        // 测试图标集规则
        let icon_rule = ConditionalFormatRule::three_arrows();
        if let ConditionalFormatRule::IconSet { config } = icon_rule {
            assert_eq!(config.icon_set, "3Arrows");
        }

        // 测试单一条件规则
        let single_rule = ConditionalFormatRule::single_condition(
            ConditionalFormatOperator::Equal,
            "已完成".to_string(),
        );
        if let ConditionalFormatRule::SingleCondition { config } = single_rule {
            assert_eq!(config.value, "已完成");
        }

        // 测试公式规则
        let formula_rule = ConditionalFormatRule::formula("=A1>B1".to_string());
        if let ConditionalFormatRule::Formula { config } = formula_rule {
            assert_eq!(config.formula, "=A1>B1");
        }
    }

    #[test]
    fn test_conditional_format_creation() {
        use super::super::models::Range;

        let range = Range::from("A1".to_string(), "A10".to_string());
        let rule = ConditionalFormatRule::data_bar(Color::green(), Color::red());
        let conditional_format = ConditionalFormat::new("sheet123".to_string(), range, rule);

        assert_eq!(conditional_format.sheet_id, "sheet123");
        assert!(conditional_format.validate().is_ok());
    }

    #[test]
    fn test_conditional_format_validation() {
        use super::super::models::Range;

        let range = Range::from("A1".to_string(), "A10".to_string());
        let rule = ConditionalFormatRule::data_bar(Color::green(), Color::red());

        // 测试有效格式
        let valid_format = ConditionalFormat::new("sheet123".to_string(), range, rule);
        assert!(valid_format.validate().is_ok());

        // 测试空工作表ID
        let invalid_format = ConditionalFormat::new(
            "".to_string(),
            Range::from("A1".to_string(), "A10".to_string()),
            ConditionalFormatRule::data_bar(Color::green(), Color::red()),
        );
        assert!(invalid_format.validate().is_err());
    }

    #[test]
    fn test_set_conditional_format_request_builder() {
        use super::super::models::Range;

        let range = Range::from("A1".to_string(), "A10".to_string());
        let rule = ConditionalFormatRule::data_bar(Color::green(), Color::red());

        let request = SetConditionalFormatRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .range(range)
            .rule(rule)
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.conditional_format.sheet_id, "sheet123");
    }

    #[test]
    fn test_delete_conditional_format_request() {
        let request =
            DeleteConditionalFormatRequest::new("token123".to_string(), "format123".to_string());

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.conditional_format_id, "format123");
    }

    #[test]
    fn test_conditional_format_service_creation() {
        let config = openlark_core::config::openlark_core::config::Config::default();
        let service = ConditionalFormatService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_comprehensive_conditional_format_scenarios() {
        use super::super::models::Range;

        // 测试数据条场景
        let data_bar_request = SetConditionalFormatRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .range(Range::from("A1".to_string(), "A10".to_string()))
            .rule(ConditionalFormatRule::data_bar(
                Color::green(),
                Color::red(),
            ))
            .build()
            .unwrap();

        assert_eq!(data_bar_request.conditional_format.sheet_id, "sheet123");

        // 测试色阶场景
        let color_scale_request = SetConditionalFormatRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .range(Range::from("B1".to_string(), "B10".to_string()))
            .rule(ConditionalFormatRule::three_color_scale(
                Color::blue(),
                Color::yellow(),
                Color::red(),
            ))
            .build()
            .unwrap();

        assert_eq!(color_scale_request.conditional_format.sheet_id, "sheet123");

        // 测试图标集场景
        let icon_set_request = SetConditionalFormatRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .range(Range::from("C1".to_string(), "C10".to_string()))
            .rule(ConditionalFormatRule::three_traffic_lights())
            .build()
            .unwrap();

        assert_eq!(icon_set_request.conditional_format.sheet_id, "sheet123");

        // 测试单一条件场景
        let single_condition_request = SetConditionalFormatRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .range(Range::from("D1".to_string(), "D10".to_string()))
            .rule(ConditionalFormatRule::single_condition(
                ConditionalFormatOperator::Equal,
                "已完成".to_string(),
            ))
            .build()
            .unwrap();

        assert_eq!(
            single_condition_request.conditional_format.sheet_id,
            "sheet123"
        );

        // 测试公式场景
        let formula_request = SetConditionalFormatRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .range(Range::from("E1".to_string(), "E10".to_string()))
            .rule(ConditionalFormatRule::formula("=A1>B1".to_string()))
            .build()
            .unwrap();

        assert_eq!(formula_request.conditional_format.sheet_id, "sheet123");
    }

    #[test]
    fn test_conditional_format_serialization() {
        use serde_json;

        let rule = ConditionalFormatRule::data_bar(Color::green(), Color::red());
        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("DATA_BAR"));

        let single_rule = ConditionalFormatRule::single_condition(
            ConditionalFormatOperator::Greater,
            "100".to_string(),
        );
        let json2 = serde_json::to_string(&single_rule).unwrap();
        assert!(json2.contains("SINGLE_CONDITION"));
    }

    #[test]
    fn test_color_hex_validation() {
        // 测试有效十六进制
        assert!(Color::hex("#FF0000").is_ok());
        assert!(Color::hex("FF0000").is_ok());

        // 测试无效十六进制
        assert!(Color::hex("").is_err());
        assert!(Color::hex("#FF00").is_err()); // 长度错误
        assert!(Color::hex("#GG0000").is_err()); // 无效字符
    }
}

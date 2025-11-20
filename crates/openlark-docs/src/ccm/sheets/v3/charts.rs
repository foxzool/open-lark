//! Sheets电子表格图表服务 v3
//!
//! 提供飞书电子表格v3版本的图表管理功能，包括：
//! - 创建和删除图表
//! - 多种图表类型支持（柱状图、折线图、饼图、散点图等）
//! - 图表样式和配置管理
//! - 数据范围和系列配置
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    error::LarkAPIError,
    http::Transport,
};

use reqwest::Method;
use serde::{Deserialize, Serialize};

// 使用统一类型定义
use super::Range;

/// 图表类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    /// 柱状图
    #[serde(rename = "COLUMN")]
    Column,
    /// 条形图
    #[serde(rename = "BAR")]
    Bar,
    /// 折线图
    #[serde(rename = "LINE")]
    Line,
    /// 面积图
    #[serde(rename = "AREA")]
    Area,
    /// 饼图
    #[serde(rename = "PIE")]
    Pie,
    /// 环形图
    #[serde(rename = "DOUGHNUT")]
    Doughnut,
    /// 散点图
    #[serde(rename = "SCATTER")]
    Scatter,
    /// 气泡图
    #[serde(rename = "BUBBLE")]
    Bubble,
    /// 雷达图
    #[serde(rename = "RADAR")]
    Radar,
    /// 组合图
    #[serde(rename = "COMBO")]
    Combo,
}

/// 图表子类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartSubType {
    /// 二维图表
    #[serde(rename = "2D")]
    TwoD,
    /// 三维图表
    #[serde(rename = "3D")]
    ThreeD,
    /// 堆积图表
    #[serde(rename = "STACKED")]
    Stacked,
    /// 百分比堆积
    #[serde(rename = "PERCENT_STACKED")]
    PercentStacked,
    /// 簇状
    #[serde(rename = "CLUSTERED")]
    Clustered,
}

/// 图表标题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartTitle {
    /// 标题文本
    #[serde(rename = "text")]
    pub text: String,
    /// 字体大小
    #[serde(rename = "font_size")]
    pub font_size: Option<u32>,
    /// 字体加粗
    #[serde(rename = "bold")]
    pub bold: Option<bool>,
    /// 字体颜色
    #[serde(rename = "color")]
    pub color: Option<String>,
}

impl ChartTitle {
    /// 创建图表标题
    pub fn new(text: String) -> Self {
        Self {
            text,
            font_size: None,
            bold: None,
            color: None,
        }
    }

    /// 设置字体大小
    pub fn font_size(mut self, size: u32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// 设置字体加粗
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    /// 设置字体颜色
    pub fn color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }
}

/// 图例配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartLegend {
    /// 是否显示图例
    #[serde(rename = "show")]
    pub show: Option<bool>,
    /// 图例位置
    #[serde(rename = "position")]
    pub position: Option<LegendPosition>,
}

/// 图例位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendPosition {
    /// 顶部
    #[serde(rename = "TOP")]
    Top,
    /// 底部
    #[serde(rename = "BOTTOM")]
    Bottom,
    /// 左侧
    #[serde(rename = "LEFT")]
    Left,
    /// 右侧
    #[serde(rename = "RIGHT")]
    Right,
}

impl ChartLegend {
    /// 创建图例配置
    pub fn new() -> Self {
        Self {
            show: None,
            position: None,
        }
    }

    /// 设置是否显示
    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    /// 设置位置
    pub fn position(mut self, position: LegendPosition) -> Self {
        self.position = Some(position);
        self
    }
}

/// 坐标轴配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartAxis {
    /// 是否显示坐标轴
    #[serde(rename = "show")]
    pub show: Option<bool>,
    /// 轴标题
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// 最小值
    #[serde(rename = "min")]
    pub min: Option<f64>,
    /// 最大值
    #[serde(rename = "max")]
    pub max: Option<f64>,
    /// 是否显示网格线
    #[serde(rename = "show_gridlines")]
    pub show_gridlines: Option<bool>,
}

impl ChartAxis {
    /// 创建坐标轴配置
    pub fn new() -> Self {
        Self {
            show: None,
            title: None,
            min: None,
            max: None,
            show_gridlines: None,
        }
    }

    /// 设置是否显示
    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    /// 设置标题
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置最小值
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// 设置最大值
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// 设置是否显示网格线
    pub fn show_gridlines(mut self, show_gridlines: bool) -> Self {
        self.show_gridlines = Some(show_gridlines);
        self
    }
}

/// 数据系列配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartSeries {
    /// 系列名称
    #[serde(rename = "name")]
    pub name: String,
    /// 数据范围
    #[serde(rename = "data_range")]
    pub data_range: Range,
    /// 系列颜色
    #[serde(rename = "color")]
    pub color: Option<String>,
    /// 系列样式
    #[serde(rename = "style")]
    pub style: Option<HashMap<String, String>>,
}

impl ChartSeries {
    /// 创建数据系列
    pub fn new(name: String, data_range: Range) -> Self {
        Self {
            name,
            data_range,
            color: None,
            style: None,
        }
    }

    /// 设置颜色
    pub fn color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }

    /// 添加样式
    pub fn add_style(mut self, key: String, value: String) -> Self {
        if self.style.is_none() {
            self.style = Some(HashMap::new());
        }
        if let Some(ref mut style) = self.style {
            style.insert(key, value);
        }
        self
    }
}

/// 图表样式配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartStyle {
    /// 图表标题
    #[serde(rename = "title")]
    pub title: Option<ChartTitle>,
    /// 图例配置
    #[serde(rename = "legend")]
    pub legend: Option<ChartLegend>,
    /// X轴配置
    #[serde(rename = "x_axis")]
    pub x_axis: Option<ChartAxis>,
    /// Y轴配置
    #[serde(rename = "y_axis")]
    pub y_axis: Option<ChartAxis>,
    /// 背景颜色
    #[serde(rename = "background_color")]
    pub background_color: Option<String>,
    /// 边框颜色
    #[serde(rename = "border_color")]
    pub border_color: Option<String>,
}

impl ChartStyle {
    /// 创建图表样式
    pub fn new() -> Self {
        Self {
            title: None,
            legend: None,
            x_axis: None,
            y_axis: None,
            background_color: None,
            border_color: None,
        }
    }

    /// 设置标题
    pub fn title(mut self, title: ChartTitle) -> Self {
        self.title = Some(title);
        self
    }

    /// 设置图例
    pub fn legend(mut self, legend: ChartLegend) -> Self {
        self.legend = Some(legend);
        self
    }

    /// 设置X轴
    pub fn x_axis(mut self, x_axis: ChartAxis) -> Self {
        self.x_axis = Some(x_axis);
        self
    }

    /// 设置Y轴
    pub fn y_axis(mut self, y_axis: ChartAxis) -> Self {
        self.y_axis = Some(y_axis);
        self
    }

    /// 设置背景颜色
    pub fn background_color(mut self, color: String) -> Self {
        self.background_color = Some(color);
        self
    }

    /// 设置边框颜色
    pub fn border_color(mut self, color: String) -> Self {
        self.border_color = Some(color);
        self
    }
}

/// 图表配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    /// 图表类型
    #[serde(rename = "chart_type")]
    pub chart_type: ChartType,
    /// 图表子类型
    #[serde(rename = "chart_sub_type")]
    pub chart_sub_type: Option<ChartSubType>,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 数据范围
    #[serde(rename = "data_range")]
    pub data_range: Range,
    /// 数据系列列表
    #[serde(rename = "series")]
    pub series: Vec<ChartSeries>,
    /// 图表样式
    #[serde(rename = "style")]
    pub style: Option<ChartStyle>,
    /// 图表位置
    #[serde(rename = "position")]
    pub position: Option<ChartPosition>,
}

/// 图表位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartPosition {
    /// 起始行
    #[serde(rename = "start_row")]
    pub start_row: u32,
    /// 起始列
    #[serde(rename = "start_column")]
    pub start_column: u32,
    /// 宽度（列数）
    #[serde(rename = "width")]
    pub width: u32,
    /// 高度（行数）
    #[serde(rename = "height")]
    pub height: u32,
}

impl ChartPosition {
    /// 创建图表位置
    pub fn new(start_row: u32, start_column: u32, width: u32, height: u32) -> Self {
        Self {
            start_row,
            start_column,
            width,
            height,
        }
    }
}

impl ChartConfig {
    /// 创建图表配置
    pub fn new(
        chart_type: ChartType,
        sheet_id: String,
        data_range: Range,
        series: Vec<ChartSeries>,
    ) -> Self {
        Self {
            chart_type,
            chart_sub_type: None,
            sheet_id,
            data_range,
            series,
            style: None,
            position: None,
        }
    }

    /// 设置子类型
    pub fn sub_type(mut self, sub_type: ChartSubType) -> Self {
        self.chart_sub_type = Some(sub_type);
        self
    }

    /// 设置样式
    pub fn style(mut self, style: ChartStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// 设置位置
    pub fn position(mut self, position: ChartPosition) -> Self {
        self.position = Some(position);
        self
    }

    /// 验证图表配置
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.sheet_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "工作表ID不能为空".to_string(),
            ));
        }

        if self.series.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "至少需要一个数据系列".to_string(),
            ));
        }

        // 验证系列配置
        for series in &self.series {
            if series.name.is_empty() {
                return Err(LarkAPIError::IllegalParamError(
                    "系列名称不能为空".to_string(),
                ));
            }
        }

        if let Some(ref position) = self.position {
            if position.width == 0 || position.height == 0 {
                return Err(LarkAPIError::IllegalParamError(
                    "图表宽度和高度必须大于0".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// 创建图表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChartRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 图表配置
    #[serde(rename = "chart_config")]
    pub chart_config: ChartConfig,
}

impl CreateChartRequest {
    /// 创建创建图表请求
    pub fn new(spreadsheet_token: String, chart_config: ChartConfig) -> Self {
        Self {
            spreadsheet_token,
            chart_config,
        }
    }

    /// 构建器模式
    pub fn builder() -> CreateChartRequestBuilder {
        CreateChartRequestBuilder::default()
    }
}

/// 创建图表请求构建器
#[derive(Debug, Default)]
pub struct CreateChartRequestBuilder {
    spreadsheet_token: Option<String>,
    chart_type: Option<ChartType>,
    sheet_id: Option<String>,
    data_range: Option<Range>,
    series: Vec<ChartSeries>,
    chart_sub_type: Option<ChartSubType>,
    style: Option<ChartStyle>,
    position: Option<ChartPosition>,
}

impl CreateChartRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置图表类型
    pub fn chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = Some(chart_type);
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(sheet_id);
        self
    }

    /// 设置数据范围
    pub fn data_range(mut self, data_range: Range) -> Self {
        self.data_range = Some(data_range);
        self
    }

    /// 添加数据系列
    pub fn add_series(mut self, series: ChartSeries) -> Self {
        self.series.push(series);
        self
    }

    /// 批量添加数据系列
    pub fn series(mut self, series: Vec<ChartSeries>) -> Self {
        self.series.extend(series);
        self
    }

    /// 设置子类型
    pub fn sub_type(mut self, sub_type: ChartSubType) -> Self {
        self.chart_sub_type = Some(sub_type);
        self
    }

    /// 设置样式
    pub fn style(mut self, style: ChartStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// 设置位置
    pub fn position(mut self, position: ChartPosition) -> Self {
        self.position = Some(position);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<CreateChartRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let chart_type = self
            .chart_type
            .ok_or_else(|| LarkAPIError::IllegalParamError("图表类型不能为空".to_string()))?;

        let sheet_id = self
            .sheet_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("工作表ID不能为空".to_string()))?;

        let data_range = self
            .data_range
            .ok_or_else(|| LarkAPIError::IllegalParamError("数据范围不能为空".to_string()))?;

        let mut chart_config = ChartConfig::new(chart_type, sheet_id, data_range, self.series);

        if let Some(sub_type) = self.chart_sub_type {
            chart_config = chart_config.sub_type(sub_type);
        }
        if let Some(style) = self.style {
            chart_config = chart_config.style(style);
        }
        if let Some(position) = self.position {
            chart_config = chart_config.position(position);
        }

        chart_config.validate()?;

        Ok(CreateChartRequest {
            spreadsheet_token,
            chart_config,
        })
    }
}

/// 创建图表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChartResponse {
    /// 图表ID
    #[serde(rename = "chart_id")]
    pub chart_id: String,
    /// 图表配置
    #[serde(rename = "chart_config")]
    pub chart_config: ChartConfig,
}

// 实现API响应特征
impl ApiResponseTrait for CreateChartResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除图表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteChartRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 图表ID
    #[serde(rename = "chart_id")]
    pub chart_id: String,
}

impl DeleteChartRequest {
    /// 创建删除图表请求
    pub fn new(spreadsheet_token: String, chart_id: String) -> Self {
        Self {
            spreadsheet_token,
            chart_id,
        }
    }

    /// 构建器模式
    pub fn builder() -> DeleteChartRequestBuilder {
        DeleteChartRequestBuilder::default()
    }
}

/// 删除图表请求构建器
#[derive(Debug, Default)]
pub struct DeleteChartRequestBuilder {
    spreadsheet_token: Option<String>,
    chart_id: Option<String>,
}

impl DeleteChartRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置图表ID
    pub fn chart_id(mut self, chart_id: String) -> Self {
        self.chart_id = Some(chart_id);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<DeleteChartRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let chart_id = self
            .chart_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("图表ID不能为空".to_string()))?;

        Ok(DeleteChartRequest {
            spreadsheet_token,
            chart_id,
        })
    }
}

/// 删除图表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteChartResponse {
    /// 是否成功删除
    pub success: bool,
}

// 实现API响应特征
impl ApiResponseTrait for DeleteChartResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Sheets电子表格图表服务 v3
#[derive(Clone, Debug)]
pub struct ChartService {
    config: openlark_core::config::Config,
}

impl ChartService {
    /// 创建图表服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 创建图表
    ///
    /// 在指定工作表中创建图表，支持多种图表类型和自定义配置。
    ///
    /// # 参数
    /// - `request`: 创建图表请求
    ///
    /// # 返回
    /// 返回创建后的图表信息，包括图表ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::charts::*;
    /// use open_lark::service::sheets::v3::models::Range;
    ///
    /// // 创建数据范围
    /// let data_range = Range::from("A1".to_string(), "B10".to_string());
    ///
    /// // 创建数据系列
    /// let series1 = ChartSeries::new(
    ///     "销售额".to_string(),
    ///     Range::from("A1".to_string(), "A10".to_string())
    /// ).color("#FF0000".to_string());
    ///
    /// let series2 = ChartSeries::new(
    ///     "利润".to_string(),
    ///     Range::from("B1".to_string(), "B10".to_string())
    /// ).color("#00FF00".to_string());
    ///
    /// // 设置图表位置
    /// let position = ChartPosition::new(1, 3, 8, 10);
    ///
    /// // 创建图表
    /// let request = CreateChartRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .chart_type(ChartType::Column)
    ///     .sheet_id("sheet_id".to_string())
    ///     .data_range(data_range)
    ///     .add_series(series1)
    ///     .add_series(series2)
    ///     .position(position)
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.create_chart(&request).await?;
    /// ```
    pub async fn create_chart(
        &self,
        request: &CreateChartRequest,
    ) -> openlark_core::error::SDKResult<CreateChartResponse> {
        use openlark_core::{api::ApiRequest, api::Response, error::LarkAPIError, http::Transport};

        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/charts",
            request.spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response: Response<CreateChartResponse> =
            Transport::request(api_request, &self.config, None).await?;

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

    /// 删除图表
    ///
    /// 删除指定的图表。
    ///
    /// # 参数
    /// - `request`: 删除图表请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::charts::*;
    ///
    /// // 删除图表
    /// let request = DeleteChartRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .chart_id("chart_id".to_string())
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.delete_chart(&request).await?;
    /// assert!(response.success);
    /// ```
    pub async fn delete_chart(
        &self,
        request: &DeleteChartRequest,
    ) -> openlark_core::error::SDKResult<DeleteChartResponse> {
        use openlark_core::{api::ApiRequest, api::Response, error::LarkAPIError, http::Transport};

        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/charts/{}",
            request.spreadsheet_token, request.chart_id
        );

        let api_request = ApiRequest::with_method_and_path(Method::DELETE, &endpoint);

        let response: Response<DeleteChartResponse> =
            Transport::request(api_request, &self.config, None).await?;

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

    /// 创建图表构建器
    pub fn create_chart_builder(&self) -> CreateChartRequestBuilder {
        CreateChartRequestBuilder::default()
    }

    /// 删除图表构建器
    pub fn delete_chart_builder(&self) -> DeleteChartRequestBuilder {
        DeleteChartRequestBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_title_creation() {
        let title = ChartTitle::new("销售图表".to_string())
            .font_size(16)
            .bold(true)
            .color("#FF0000".to_string());

        assert_eq!(title.text, "销售图表");
        assert_eq!(title.font_size, Some(16));
        assert_eq!(title.bold, Some(true));
        assert_eq!(title.color, Some("#FF0000".to_string()));
    }

    #[test]
    fn test_chart_legend_creation() {
        let legend = ChartLegend::new()
            .show(true)
            .position(LegendPosition::Bottom);

        assert_eq!(legend.show, Some(true));
        assert_eq!(legend.position, Some(LegendPosition::Bottom));
    }

    #[test]
    fn test_chart_axis_creation() {
        let axis = ChartAxis::new()
            .show(true)
            .title("销售额".to_string())
            .min(0.0)
            .max(1000.0)
            .show_gridlines(true);

        assert_eq!(axis.show, Some(true));
        assert_eq!(axis.title, Some("销售额".to_string()));
        assert_eq!(axis.min, Some(0.0));
        assert_eq!(axis.max, Some(1000.0));
        assert_eq!(axis.show_gridlines, Some(true));
    }

    #[test]
    fn test_chart_series_creation() {
        use super::super::models::Range;

        let data_range = Range::from("A1".to_string(), "A10".to_string());
        let series = ChartSeries::new("销售额".to_string(), data_range)
            .color("#FF0000".to_string())
            .add_style("line_width".to_string(), "3".to_string());

        assert_eq!(series.name, "销售额");
        assert_eq!(series.color, Some("#FF0000".to_string()));
        assert!(series.style.is_some());
        if let Some(ref style) = series.style {
            assert_eq!(style.get("line_width"), Some(&"3".to_string()));
        }
    }

    #[test]
    fn test_chart_style_creation() {
        let title = ChartTitle::new("销售图表".to_string());
        let legend = ChartLegend::new().position(LegendPosition::Top);
        let x_axis = ChartAxis::new().title("月份".to_string());
        let y_axis = ChartAxis::new().title("销售额".to_string());

        let style = ChartStyle::new()
            .title(title)
            .legend(legend)
            .x_axis(x_axis)
            .y_axis(y_axis)
            .background_color("#FFFFFF".to_string())
            .border_color("#CCCCCC".to_string());

        assert!(style.title.is_some());
        assert!(style.legend.is_some());
        assert!(style.x_axis.is_some());
        assert!(style.y_axis.is_some());
        assert_eq!(style.background_color, Some("#FFFFFF".to_string()));
        assert_eq!(style.border_color, Some("#CCCCCC".to_string()));
    }

    #[test]
    fn test_chart_position_creation() {
        let position = ChartPosition::new(1, 3, 8, 10);

        assert_eq!(position.start_row, 1);
        assert_eq!(position.start_column, 3);
        assert_eq!(position.width, 8);
        assert_eq!(position.height, 10);
    }

    #[test]
    fn test_chart_config_creation() {
        use super::super::models::Range;

        let data_range = Range::from("A1".to_string(), "B10".to_string());
        let series1 = ChartSeries::new(
            "销售额".to_string(),
            Range::from("A1".to_string(), "A10".to_string()),
        );
        let series2 = ChartSeries::new(
            "利润".to_string(),
            Range::from("B1".to_string(), "B10".to_string()),
        );

        let config = openlark_core::config::Config::new(
            ChartType::Column,
            "sheet123".to_string(),
            data_range,
            vec![series1, series2],
        )
        .sub_type(ChartSubType::Clustered)
        .style(ChartStyle::new())
        .position(ChartPosition::new(1, 3, 8, 10));

        assert_eq!(config.sheet_id, "sheet123");
        assert_eq!(config.series.len(), 2);
        assert!(config.chart_sub_type.is_some());
        assert!(config.style.is_some());
        assert!(config.position.is_some());
    }

    #[test]
    fn test_chart_config_validation() {
        use super::super::models::Range;

        let data_range = Range::from("A1".to_string(), "B10".to_string());
        let series = ChartSeries::new(
            "销售额".to_string(),
            Range::from("A1".to_string(), "A10".to_string()),
        );

        // 测试有效配置
        let valid_config = openlark_core::config::Config::new(
            ChartType::Line,
            "sheet123".to_string(),
            data_range,
            vec![series],
        );
        assert!(valid_config.validate().is_ok());

        // 测试空工作表ID
        let invalid_config = openlark_core::config::Config::new(
            ChartType::Line,
            "".to_string(),
            Range::from("A1".to_string(), "B10".to_string()),
            vec![],
        );
        assert!(invalid_config.validate().is_err());

        // 测试空系列列表
        let invalid_config2 = openlark_core::config::Config::new(
            ChartType::Line,
            "sheet123".to_string(),
            Range::from("A1".to_string(), "B10".to_string()),
            vec![],
        );
        assert!(invalid_config2.validate().is_err());
    }

    #[test]
    fn test_create_chart_request_builder() {
        use super::super::models::Range;

        let data_range = Range::from("A1".to_string(), "B10".to_string());
        let series = ChartSeries::new(
            "销售额".to_string(),
            Range::from("A1".to_string(), "A10".to_string()),
        );

        let request = CreateChartRequest::builder()
            .spreadsheet_token("token123".to_string())
            .chart_type(ChartType::Column)
            .sheet_id("sheet123".to_string())
            .data_range(data_range)
            .add_series(series)
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.chart_config.sheet_id, "sheet123");
        assert_eq!(request.chart_config.series.len(), 1);
    }

    #[test]
    fn test_delete_chart_request() {
        let request = DeleteChartRequest::new("token123".to_string(), "chart123".to_string());

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.chart_id, "chart123");
    }

    #[test]
    fn test_delete_chart_request_builder() {
        let request = DeleteChartRequest::builder()
            .spreadsheet_token("token123".to_string())
            .chart_id("chart123".to_string())
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.chart_id, "chart123");
    }

    #[test]
    fn test_chart_service_creation() {
        let config = openlark_core::config::openlark_core::config::Config::default();
        let service = ChartService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_comprehensive_chart_scenarios() {
        use super::super::models::Range;

        // 测试柱状图场景
        let column_chart_request = CreateChartRequest::builder()
            .spreadsheet_token("token123".to_string())
            .chart_type(ChartType::Column)
            .sheet_id("sheet123".to_string())
            .data_range(Range::from("A1".to_string(), "B10".to_string()))
            .add_series(ChartSeries::new(
                "销售额".to_string(),
                Range::from("A1".to_string(), "A10".to_string()),
            ))
            .position(ChartPosition::new(1, 3, 8, 10))
            .build()
            .unwrap();

        assert_eq!(
            column_chart_request.chart_config.chart_type,
            ChartType::Column
        );

        // 测试折线图场景
        let line_chart_request = CreateChartRequest::builder()
            .spreadsheet_token("token123".to_string())
            .chart_type(ChartType::Line)
            .sheet_id("sheet123".to_string())
            .data_range(Range::from("A1".to_string(), "B10".to_string()))
            .add_series(ChartSeries::new(
                "趋势".to_string(),
                Range::from("A1".to_string(), "A10".to_string()),
            ))
            .style(ChartStyle::new())
            .build()
            .unwrap();

        assert_eq!(line_chart_request.chart_config.chart_type, ChartType::Line);

        // 测试饼图场景
        let pie_chart_request = CreateChartRequest::builder()
            .spreadsheet_token("token123".to_string())
            .chart_type(ChartType::Pie)
            .sheet_id("sheet123".to_string())
            .data_range(Range::from("A1".to_string(), "B10".to_string()))
            .add_series(ChartSeries::new(
                "占比".to_string(),
                Range::from("A1".to_string(), "A10".to_string()),
            ))
            .sub_type(ChartSubType::TwoD)
            .build()
            .unwrap();

        assert_eq!(pie_chart_request.chart_config.chart_type, ChartType::Pie);
        assert_eq!(
            pie_chart_request.chart_config.chart_sub_type,
            Some(ChartSubType::TwoD)
        );

        // 测试多系列图表
        let multi_series_request = CreateChartRequest::builder()
            .spreadsheet_token("token123".to_string())
            .chart_type(ChartType::Combo)
            .sheet_id("sheet123".to_string())
            .data_range(Range::from("A1".to_string(), "C10".to_string()))
            .series(vec![
                ChartSeries::new(
                    "销售额".to_string(),
                    Range::from("A1".to_string(), "A10".to_string()),
                ),
                ChartSeries::new(
                    "利润".to_string(),
                    Range::from("B1".to_string(), "B10".to_string()),
                ),
                ChartSeries::new(
                    "成本".to_string(),
                    Range::from("C1".to_string(), "C10".to_string()),
                ),
            ])
            .style(
                ChartStyle::new()
                    .title(ChartTitle::new("财务分析".to_string()))
                    .legend(ChartLegend::new().position(LegendPosition::Bottom)),
            )
            .build()
            .unwrap();

        assert_eq!(multi_series_request.chart_config.series.len(), 3);
        assert_eq!(
            multi_series_request.chart_config.chart_type,
            ChartType::Combo
        );
    }

    #[test]
    fn test_chart_serialization() {
        use serde_json;

        let config = openlark_core::config::Config::new(
            ChartType::Column,
            "sheet123".to_string(),
            super::super::models::Range::from("A1".to_string(), "B10".to_string()),
            vec![],
        );
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("COLUMN"));
        assert!(json.contains("sheet123"));
    }

    #[test]
    fn test_chart_types() {
        let chart_types = vec![
            ChartType::Column,
            ChartType::Bar,
            ChartType::Line,
            ChartType::Area,
            ChartType::Pie,
            ChartType::Doughnut,
            ChartType::Scatter,
            ChartType::Bubble,
            ChartType::Radar,
            ChartType::Combo,
        ];

        for chart_type in chart_types {
            let config = openlark_core::config::Config::new(
                chart_type,
                "sheet123".to_string(),
                super::super::models::Range::from("A1".to_string(), "B10".to_string()),
                vec![ChartSeries::new(
                    "test".to_string(),
                    super::super::models::Range::from("A1".to_string(), "A10".to_string()),
                )],
            );
            assert!(config.validate().is_ok());
        }
    }
}

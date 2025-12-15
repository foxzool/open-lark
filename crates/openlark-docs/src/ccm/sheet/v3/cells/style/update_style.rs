use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 更新单元格样式请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateStyleRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 范围信息
    pub range: Option<String>,
    /// 起始行
    pub start_row: Option<i32>,
    /// 起始列
    pub start_column: Option<i32>,
    /// 结束行
    pub end_row: Option<i32>,
    /// 结束列
    pub end_column: Option<i32>,
    /// 样式信息
    pub style: CellStyle,
}

/// 单元格样式
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CellStyle {
    /// 字体样式
    pub font: Option<FontStyle>,
    /// 背景颜色
    pub background_color: Option<ColorStyle>,
    /// 文本样式
    pub text_style: Option<TextStyle>,
    /// 边框样式
    pub border: Option<BorderStyle>,
    /// 对齐方式
    pub alignment: Option<AlignmentStyle>,
}

/// 字体样式
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FontStyle {
    /// 字体名称
    pub name: Option<String>,
    /// 字体大小
    pub size: Option<i32>,
    /// 是否粗体
    pub bold: Option<bool>,
    /// 是否斜体
    pub italic: Option<bool>,
    /// 是否删除线
    pub strikethrough: Option<bool>,
    /// 字体颜色
    pub color: Option<ColorStyle>,
}

/// 颜色样式
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ColorStyle {
    /// RGB颜色
    pub rgb_color: Option<RgbColor>,
    /// 主题颜色
    pub theme_color: Option<String>,
}

/// RGB颜色
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RgbColor {
    /// 红色值 (0-255)
    pub red: f32,
    /// 绿色值 (0-255)
    pub green: f32,
    /// 蓝色值 (0-255)
    pub blue: f32,
}

/// 文本样式
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextStyle {
    /// 文本装饰
    pub decoration: Option<String>,
    /// 文本旋转
    pub rotation: Option<i32>,
    /// 文本换行
    pub wrap_text: Option<bool>,
    /// 文本缩进
    pub indent: Option<i32>,
}

/// 边框样式
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BorderStyle {
    /// 上边框
    pub top: Option<Border>,
    /// 下边框
    pub bottom: Option<Border>,
    /// 左边框
    pub left: Option<Border>,
    /// 右边框
    pub right: Option<Border>,
}

/// 边框信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Border {
    /// 边框样式
    pub style: Option<String>,
    /// 边框颜色
    pub color: Option<ColorStyle>,
    /// 边框宽度
    pub width: Option<i32>,
}

/// 对齐方式
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlignmentStyle {
    /// 水平对齐
    pub horizontal: Option<String>,
    /// 垂直对齐
    pub vertical: Option<String>,
}

/// 更新单元格样式响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct UpdateStyleResponse {
    /// 电子表格属性
    pub spreadsheet: SpreadsheetProperties,
    /// 操作结果
    pub result: String,
}

/// 电子表格属性
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SpreadsheetProperties {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
}

/// 工作表信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SheetInfo {
    /// 工作表属性
    pub properties: SheetPropertiesInfo,
    /// 数据区域
    pub data: Option<Vec<GridData>>,
}

/// 工作表属性信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SheetPropertiesInfo {
    /// 工作表ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表类型
    pub sheet_type: String,
    /// 网格属性
    pub grid_properties: Option<GridPropertiesInfo>,
}

/// 网格属性信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GridPropertiesInfo {
    /// 行数
    pub row_count: i32,
    /// 列数
    pub column_count: i32,
    /// 冻结行数
    pub frozen_row_count: i32,
    /// 冻结列数
    pub frozen_column_count: i32,
}

/// 网格数据
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GridData {
    /// 起始行
    pub start_row: i32,
    /// 起始列
    pub start_column: i32,
    /// 行数据
    pub row_data: Vec<RowData>,
}

/// 行数据
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RowData {
    /// 行号
    pub row_number: i32,
    /// 单元格数据
    pub values: Vec<CellData>,
}

/// 单元格数据
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CellData {
    /// 行号
    pub row_index: i32,
    /// 列号
    pub column_index: i32,
    /// 单元格值
    pub value: Option<serde_json::Value>,
    /// 格式
    pub format: Option<String>,
    /// 样式
    pub style: Option<CellStyleInfo>,
}

/// 单元格样式信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CellStyleInfo {
    /// 字体样式
    pub font: Option<FontStyleInfo>,
    /// 背景颜色
    pub background_color: Option<ColorStyleInfo>,
    /// 对齐方式
    pub alignment: Option<AlignmentStyleInfo>,
}

/// 字体样式信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct FontStyleInfo {
    /// 字体名称
    pub name: Option<String>,
    /// 字体大小
    pub size: Option<i32>,
    /// 是否粗体
    pub bold: Option<bool>,
    /// 是否斜体
    pub italic: Option<bool>,
}

/// 颜色样式信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ColorStyleInfo {
    /// RGB颜色
    pub rgb_color: Option<RgbColorInfo>,
}

/// RGB颜色信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RgbColorInfo {
    /// 红色值
    pub red: f32,
    /// 绿色值
    pub green: f32,
    /// 蓝色值
    pub blue: f32,
}

/// 对齐方式信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AlignmentStyleInfo {
    /// 水平对齐
    pub horizontal: Option<String>,
    /// 垂直对齐
    pub vertical: Option<String>,
}

impl ApiResponseTrait for UpdateStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateStyleRequest {
    /// 创建新的更新单元格样式请求构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = token.into();
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, id: impl Into<String>) -> Self {
        self.sheet_id = id.into();
        self
    }

    /// 设置范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 设置起始行
    pub fn start_row(mut self, row: i32) -> Self {
        self.start_row = Some(row);
        self
    }

    /// 设置起始列
    pub fn start_column(mut self, column: i32) -> Self {
        self.start_column = Some(column);
        self
    }

    /// 设置结束行
    pub fn end_row(mut self, row: i32) -> Self {
        self.end_row = Some(row);
        self
    }

    /// 设置结束列
    pub fn end_column(mut self, column: i32) -> Self {
        self.end_column = Some(column);
        self
    }

    /// 设置样式信息
    pub fn style(mut self, style: CellStyle) -> Self {
        self.style = style;
        self
    }

    /// 设置范围（便捷方法）
    pub fn cell_range(mut self, start_row: i32, start_column: i32, end_row: i32, end_column: i32) -> Self {
        self.start_row = Some(start_row);
        self.start_column = Some(start_column);
        self.end_row = Some(end_row);
        self.end_column = Some(end_column);
        self
    }
}

impl CellStyle {
    /// 创建新的单元格样式构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置字体样式
    pub fn font(mut self, font: FontStyle) -> Self {
        self.font = Some(font);
        self
    }

    /// 设置背景颜色
    pub fn background_color(mut self, color: ColorStyle) -> Self {
        self.background_color = Some(color);
        self
    }

    /// 设置文本样式
    pub fn text_style(mut self, style: TextStyle) -> Self {
        self.text_style = Some(style);
        self
    }

    /// 设置边框样式
    pub fn border(mut self, border: BorderStyle) -> Self {
        self.border = Some(border);
        self
    }

    /// 设置对齐方式
    pub fn alignment(mut self, alignment: AlignmentStyle) -> Self {
        self.alignment = Some(alignment);
        self
    }
}

impl FontStyle {
    /// 创建新的字体样式构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置字体名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置字体大小
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }

    /// 设置是否粗体
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    /// 设置是否斜体
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    /// 设置是否删除线
    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    /// 设置字体颜色
    pub fn color(mut self, color: ColorStyle) -> Self {
        self.color = Some(color);
        self
    }
}

impl ColorStyle {
    /// 创建新的颜色样式构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置RGB颜色
    pub fn rgb_color(mut self, rgb: RgbColor) -> Self {
        self.rgb_color = Some(rgb);
        self
    }

    /// 设置主题颜色
    pub fn theme_color(mut self, theme: impl Into<String>) -> Self {
        self.theme_color = Some(theme.into());
        self
    }
}

impl RgbColor {
    /// 创建新的RGB颜色构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置红色值
    pub fn red(mut self, red: f32) -> Self {
        self.red = red;
        self
    }

    /// 设置绿色值
    pub fn green(mut self, green: f32) -> Self {
        self.green = green;
        self
    }

    /// 设置蓝色值
    pub fn blue(mut self, blue: f32) -> Self {
        self.blue = blue;
        self
    }
}

impl AlignmentStyle {
    /// 创建新的对齐方式构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置水平对齐
    pub fn horizontal(mut self, horizontal: impl Into<String>) -> Self {
        self.horizontal = Some(horizontal.into());
        self
    }

    /// 设置垂直对齐
    pub fn vertical(mut self, vertical: impl Into<String>) -> Self {
        self.vertical = Some(vertical.into());
        self
    }
}

/// 更新单元格样式
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/style
pub async fn update_style(
    request: UpdateStyleRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UpdateStyleResponse>> {
    // 构建请求体
    let body = json!(request);

    // 创建API请求
    let mut api_request: ApiRequest<UpdateStyleResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/style", SheetsApiV3, request.spreadsheet_token))
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_style_request_builder() {
        let style = CellStyle::new()
            .font(
                FontStyle::new()
                    .name("微软雅黑")
                    .size(12)
                    .bold(true)
                    .color(
                        ColorStyle::new()
                            .rgb_color(RgbColor::new().red(255.0))
                    )
            )
            .background_color(
                ColorStyle::new()
                    .rgb_color(RgbColor::new().red(240.0).green(240.0).blue(240.0))
            )
            .text_style(TextStyle::new().wrap_text(true))
            .alignment(
                AlignmentStyle::new()
                    .horizontal("CENTER")
                    .vertical("MIDDLE")
            );

        let request = UpdateStyleRequest::new()
            .spreadsheet_token("test_token")
            .sheet_id("test_sheet")
            .cell_range(0, 0, 0, 5)
            .style(style);

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "test_sheet");
        assert_eq!(request.start_row, Some(0));
        assert_eq!(request.start_column, Some(0));
        assert_eq!(request.end_row, Some(0));
        assert_eq!(request.end_column, Some(5));
        assert!(request.style.font.is_some());
        assert!(request.style.background_color.is_some());
        assert!(request.style.text_style.is_some());
        assert!(request.style.alignment.is_some());
    }
}
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};
#[derive(Serialize, Debug, Default)]
pub(crate) struct ValueRangeRequest {
    /// 插入范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见
    /// 在线表格开发指南，range所表示的范围需要大于等于values占用的范围。
    pub(crate) range: String,
    /// 需要写入的值，如要写入公式、超链接、email、@人等，可详看附录sheet 支持写入数据类型
    pub(crate) values: Value,
}

#[derive(Deserialize, Debug, Default)]
#[allow(dead_code)]
pub struct ValueRangeResponse {
    /// 插入维度
    #[serde(rename = "majorDimension")]
    pub major_dimension: String,
    /// 插入范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见
    /// 在线表格开发指南，range所表示的范围需要大于等于values占用的范围。
    pub range: String,
    /// 需要写入的值，如要写入公式、超链接、email、@人等，可详看附录sheet 支持写入数据类型
    pub values: Value,
    /// sheet 的版本号
    pub revision: i32,
}

/// 更新数据响应体
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct UpdateSheetDataResponse {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreed_sheet_token: String,
    /// 写入的范围
    #[serde(rename = "tableRange")]
    pub table_range: String,
    /// sheet 的版本号
    pub revision: i32,
    /// 追加数据的范围、行列数等
    pub updates: SheetDataUpdates,
}

impl ApiResponseTrait for UpdateSheetDataResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 追加数据的范围、行列数等
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SheetDataUpdates {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreed_sheet_token: String,
    /// 写入的范围
    #[serde(rename = "updatedRange")]
    pub updated_range: String,
    /// 写入的行数
    #[serde(rename = "updatedRows")]
    pub updated_rows: i32,
    /// 写入的列数
    #[serde(rename = "updatedColumns")]
    pub updated_columns: i32,
    /// 写入的单元格总数
    #[serde(rename = "updatedCells")]
    pub updated_cells: i32,
    /// sheet 的版本号
    pub revision: Option<i32>,
}

impl ApiResponseTrait for SheetDataUpdates {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 值与范围
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ReadRangeValueRange {
    /// 插入维度
    #[serde(rename = "majorDimension")]
    pub major_dimension: String,
    /// 返回数据的范围，为空时表示查询范围没有数据
    pub range: String,
    /// sheet 的版本号
    pub revision: i32,
    /// 查询得到的值
    pub values: Value,
}

#[derive(Debug, Serialize, Default)]
pub struct CellStyle {
    /// 字体相关样式
    pub(crate) font: StyleFont,
    /// 文本装饰 ，0 默认，1 下划线，2 删除线 ，3 下划线和删除线
    #[serde(rename = "textDecoration")]
    pub(crate) text_decoration: Option<i32>,
    /// 数字格式，详见附录 sheet支持数字格式类型
    pub(crate) formatter: Option<String>,
    /// 水平对齐，0 左对齐，1 中对齐，2 右对齐
    #[serde(rename = "hAlign")]
    pub(crate) h_align: Option<i32>,
    #[serde(rename = "vAlign")]
    /// 垂直对齐， 0 上对齐，1 中对齐， 2 下对齐
    pub(crate) v_align: Option<i32>,
    /// 字体颜色
    #[serde(rename = "foreColor")]
    pub(crate) fore_color: Option<String>,
    /// 背景颜色
    #[serde(rename = "backColor")]
    pub(crate) back_color: Option<String>,
    /// 边框类型，可选 "FULL_BORDER"，"OUTER_BORDER"，"INNER_BORDER"，"NO_BORDER"，"LEFT_BORDER"，"
    /// RIGHT_BORDER"，"TOP_BORDER"，"BOTTOM_BORDER"
    #[serde(rename = "borderType")]
    pub(crate) border_type: Option<String>,
    /// 边框颜色
    #[serde(rename = "borderColor")]
    pub(crate) border_color: Option<String>,
    /// 是否清除所有格式,默认 false
    pub(crate) clean: bool,
}

impl CellStyle {
    pub fn builder() -> CellStyleBuilder {
        CellStyleBuilder::default()
    }
}

#[derive(Default)]
pub struct CellStyleBuilder {
    request: CellStyle,
}

impl CellStyleBuilder {
    pub fn font(mut self, font: StyleFont) -> Self {
        self.request.font = font;
        self
    }

    pub fn text_decoration(mut self, text_decoration: i32) -> Self {
        self.request.text_decoration = Some(text_decoration);
        self
    }

    pub fn formatter(mut self, formatter: impl ToString) -> Self {
        self.request.formatter = Some(formatter.to_string());
        self
    }

    pub fn h_align(mut self, h_align: i32) -> Self {
        self.request.h_align = Some(h_align);
        self
    }

    pub fn v_align(mut self, v_align: i32) -> Self {
        self.request.v_align = Some(v_align);
        self
    }

    pub fn fore_color(mut self, fore_color: impl ToString) -> Self {
        self.request.fore_color = Some(fore_color.to_string());
        self
    }

    pub fn back_color(mut self, back_color: impl ToString) -> Self {
        self.request.back_color = Some(back_color.to_string());
        self
    }

    pub fn border_type(mut self, border_type: impl ToString) -> Self {
        self.request.border_type = Some(border_type.to_string());
        self
    }

    pub fn border_color(mut self, border_color: impl ToString) -> Self {
        self.request.border_color = Some(border_color.to_string());
        self
    }

    pub fn clean(mut self, clean: bool) -> Self {
        self.request.clean = clean;
        self
    }

    pub fn build(self) -> CellStyle {
        self.request
    }
}

/// 字体相关样式
#[derive(Debug, Serialize, Default)]
pub struct StyleFont {
    /// 是否加粗
    bold: Option<bool>,
    /// 是否斜体
    italic: Option<bool>,
    /// 字体大小 字号大小为9~36 行距固定为1.5，如:10pt/1.5
    #[serde(rename = "fontSize")]
    font_size: Option<String>,
    /// 清除 font 格式,默认 false
    clean: Option<bool>,
}

impl StyleFont {
    pub fn builder() -> StyleFontBuilder {
        StyleFontBuilder::default()
    }
}

#[derive(Default)]
pub struct StyleFontBuilder {
    font: StyleFont,
}

impl StyleFontBuilder {
    pub fn bold(mut self, bold: bool) -> Self {
        self.font.bold = Some(bold);
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.font.italic = Some(italic);
        self
    }

    pub fn font_size(mut self, font_size: impl ToString) -> Self {
        self.font.font_size = Some(font_size.to_string());
        self
    }

    pub fn clean(mut self, clean: bool) -> Self {
        self.font.clean = Some(clean);
        self
    }

    pub fn build(self) -> StyleFont {
        self.font
    }
}

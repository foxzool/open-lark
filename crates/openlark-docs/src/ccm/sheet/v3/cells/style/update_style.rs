use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 更新单元格样式请求
#[derive(Debug, Serialize, Default)]
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
#[derive(Debug, Serialize, Default)]
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
#[derive(Debug, Serialize, Default)]
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
#[derive(Debug, Serialize, Default)]
pub struct ColorStyle {
    /// RGB颜色
    pub rgb_color: Option<RgbColor>,
    /// 主题颜色
    pub theme_color: Option<String>,
}

/// RGB颜色
#[derive(Debug, Serialize, Default)]
pub struct RgbColor {
    /// 红色值 (0-255)
    pub red: f32,
    /// 绿色值 (0-255)
    pub green: f32,
    /// 蓝色值 (0-255)
    pub blue: f32,
}

/// 文本样式
#[derive(Debug, Serialize, Default)]
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
#[derive(Debug, Serialize, Default)]
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
#[derive(Debug, Serialize, Default)]
pub struct Border {
    /// 边框样式
    pub style: Option<String>,
    /// 边框颜色
    pub color: Option<ColorStyle>,
    /// 边框宽度
    pub width: Option<i32>,
}

/// 对齐方式
#[derive(Debug, Serialize, Default)]
pub struct AlignmentStyle {
    /// 水平对齐
    pub horizontal: Option<String>,
    /// 垂直对齐
    pub vertical: Option<String>,
}

/// 更新单元格样式响应
#[derive(Debug, Deserialize, Default)]
pub struct UpdateStyleResponse {
    /// 电子表格属性
    pub spreadsheet: SpreadsheetProperties,
    /// 操作结果
    pub result: String,
}

/// 电子表格属性
#[derive(Debug, Deserialize, Default)]
pub struct SpreadsheetProperties {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
}

/// 工作表信息
#[derive(Debug, Deserialize, Default)]
pub struct SheetInfo {
    /// 工作表属性
    pub properties: SheetPropertiesInfo,
    /// 数据区域
    pub data: Option<Vec<GridData>>,
}

/// 工作表属性信息
#[derive(Debug, Deserialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
pub struct GridData {
    /// 起始行
    pub start_row: i32,
    /// 起始列
    pub start_column: i32,
    /// 行数据
    pub row_data: Vec<RowData>,
}

/// 行数据
#[derive(Debug, Deserialize, Default)]
pub struct RowData {
    /// 行号
    pub row_number: i32,
    /// 单元格数据
    pub values: Vec<CellData>,
}

/// 单元格数据
#[derive(Debug, Deserialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
pub struct CellStyleInfo {
    /// 字体样式
    pub font: Option<FontStyleInfo>,
    /// 背景颜色
    pub background_color: Option<ColorStyleInfo>,
    /// 对齐方式
    pub alignment: Option<AlignmentStyleInfo>,
}

/// 字体样式信息
#[derive(Debug, Deserialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
pub struct ColorStyleInfo {
    /// RGB颜色
    pub rgb_color: Option<RgbColorInfo>,
}

/// RGB颜色信息
#[derive(Debug, Deserialize, Default)]
pub struct RgbColorInfo {
    /// 红色值
    pub red: f32,
    /// 绿色值
    pub green: f32,
    /// 蓝色值
    pub blue: f32,
}

/// 对齐方式信息
#[derive(Debug, Deserialize, Default)]
pub struct AlignmentStyleInfo {
    /// 水平对齐
    pub horizontal: Option<String>,
    /// 垂直对齐
    pub vertical: Option<String>,
}

/// 更新单元格样式
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/style
pub async fn update_style(
    request: UpdateStyleRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<UpdateStyleResponse>> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/style",
        config.base_url, request.spreadsheet_token
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::POST,
        headers: vec![],
        query_params: vec![],
        body: Some(serde_json::to_vec(&request)?),
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_update_style() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = UpdateStyleRequest {
            spreadsheet_token: "test_spreadsheet_token".to_string(),
            sheet_id: "test_sheet_id".to_string(),
            start_row: Some(0),
            start_column: Some(0),
            end_row: Some(0),
            end_column: Some(5),
            style: CellStyle {
                font: Some(FontStyle {
                    name: Some("微软雅黑".to_string()),
                    size: Some(12),
                    bold: Some(true),
                    italic: None,
                    strikethrough: None,
                    color: Some(ColorStyle {
                        rgb_color: Some(RgbColor {
                            red: 255.0,
                            green: 0.0,
                            blue: 0.0,
                        }),
                        theme_color: None,
                    }),
                }),
                background_color: Some(ColorStyle {
                    rgb_color: Some(RgbColor {
                        red: 240.0,
                        green: 240.0,
                        blue: 240.0,
                    }),
                    theme_color: None,
                }),
                text_style: Some(TextStyle {
                    wrap_text: Some(true),
                    rotation: None,
                    decoration: None,
                    indent: None,
                }),
                border: None,
                alignment: Some(AlignmentStyle {
                    horizontal: Some("CENTER".to_string()),
                    vertical: Some("MIDDLE".to_string()),
                }),
            },
        };

        let result = update_style(request, &config, None).await;
        assert!(result.is_ok());
    }
}
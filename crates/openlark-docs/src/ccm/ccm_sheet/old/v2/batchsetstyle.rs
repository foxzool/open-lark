/// 批量设置单元格样式
///
/// 批量为电子表格中的多个单元格范围设置样式。
/// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v2/cells-format/batch-set-style
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 批量设置单元格样式请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSetStyleParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 样式更新列表
    #[serde(rename = "styleUpdateList")]
    pub style_update_list: Vec<StyleUpdate>,
}

/// 样式更新
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleUpdate {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 起始行索引（从0开始）
    #[serde(rename = "startRowIndex")]
    pub start_row_index: i32,
    /// 结束行索引（不包含）
    #[serde(rename = "endRowIndex")]
    pub end_row_index: i32,
    /// 起始列索引（从0开始）
    #[serde(rename = "startColumnIndex")]
    pub start_column_index: i32,
    /// 结束列索引（不包含）
    #[serde(rename = "endColumnIndex")]
    pub end_column_index: i32,
    /// 样式信息
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<CellStyle>,
}

/// 单元格样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellStyle {
    /// 文本样式
    #[serde(rename = "textStyle", skip_serializing_if = "Option::is_none")]
    pub text_style: Option<TextStyle>,
    /// 背景颜色
    #[serde(rename = "backgroundColor", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,
    /// 水平对齐
    #[serde(
        rename = "horizontalAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_alignment: Option<String>,
    /// 垂直对齐
    #[serde(rename = "verticalAlignment", skip_serializing_if = "Option::is_none")]
    pub vertical_alignment: Option<String>,
    /// 自动换行
    #[serde(rename = "wrapStrategy", skip_serializing_if = "Option::is_none")]
    pub wrap_strategy: Option<String>,
    /// 文本方向
    #[serde(rename = "textDirection", skip_serializing_if = "Option::is_none")]
    pub text_direction: Option<String>,
    /// 边框样式
    #[serde(rename = "borders", skip_serializing_if = "Option::is_none")]
    pub borders: Option<Borders>,
}

/// 文本样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    /// 字体大小
    #[serde(rename = "fontSize", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i32>,
    /// 是否加粗
    #[serde(rename = "bold", skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// 是否斜体
    #[serde(rename = "italic", skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// 是否删除线
    #[serde(rename = "strikethrough", skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    /// 是否下划线
    #[serde(rename = "underline", skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    /// 字体家族
    #[serde(rename = "fontFamily", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    /// 前景颜色
    #[serde(rename = "foregroundColor", skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<Color>,
}

/// 颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    /// 红色分量 (0-1)
    #[serde(rename = "red")]
    pub red: f32,
    /// 绿色分量 (0-1)
    #[serde(rename = "green")]
    pub green: f32,
    /// 蓝色分量 (0-1)
    #[serde(rename = "blue")]
    pub blue: f32,
    /// 透明度 (0-1)
    #[serde(rename = "alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,
}

/// 边框样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Borders {
    /// 上边框
    #[serde(rename = "top", skip_serializing_if = "Option::is_none")]
    pub top: Option<Border>,
    /// 下边框
    #[serde(rename = "bottom", skip_serializing_if = "Option::is_none")]
    pub bottom: Option<Border>,
    /// 左边框
    #[serde(rename = "left", skip_serializing_if = "Option::is_none")]
    pub left: Option<Border>,
    /// 右边框
    #[serde(rename = "right", skip_serializing_if = "Option::is_none")]
    pub right: Option<Border>,
}

/// 边框
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Border {
    /// 边框样式
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    /// 边框宽度
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 边框颜色
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
}

/// 批量设置单元格样式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSetStyleResponse {
    /// 操作结果
    pub data: Option<BatchSetStyleResult>,
}

/// 批量设置样式结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSetStyleResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 更新的样式列表
    #[serde(rename = "styleUpdateList")]
    pub style_update_list: Vec<StyleUpdateResult>,
}

/// 样式更新结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleUpdateResult {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 起始行索引
    #[serde(rename = "startRowIndex")]
    pub start_row_index: i32,
    /// 结束行索引
    #[serde(rename = "endRowIndex")]
    pub end_row_index: i32,
    /// 起始列索引
    #[serde(rename = "startColumnIndex")]
    pub start_column_index: i32,
    /// 结束列索引
    #[serde(rename = "endColumnIndex")]
    pub end_column_index: i32,
    /// 应用的样式
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<CellStyle>,
}

impl ApiResponseTrait for BatchSetStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量设置单元格样式请求
pub struct BatchSetStyleRequest {
    config: Config,
}

impl BatchSetStyleRequest {
    /// 创建批量设置单元格样式请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v2/cells-format/batch-set-style
    pub async fn execute(self, params: BatchSetStyleParams) -> SDKResult<BatchSetStyleResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");
        validate_required!(params.style_update_list, "样式更新列表不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::StylesBatchUpdate(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<BatchSetStyleResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_value(params).map_err(|e| {
            openlark_core::error::validation_error(
                "参数序列化失败",
                &format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

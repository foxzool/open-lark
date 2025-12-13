/// 设置单元格样式
///
/// 根据 spreadsheetToken 和 styleRequest 设置指定单元格范围的样式。
/// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/cells-format/set-style
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 设置单元格样式请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStyleParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 样式请求
    pub style: StyleRequest,
}

/// 样式请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleRequest {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 范围
    pub range: GridRange,
    /// 样式
    pub style: CellStyle,
}

/// 网格范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridRange {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 起始行索引（从0开始）
    #[serde(rename = "startRowIndex")]
    pub start_row_index: Option<i32>,
    /// 结束行索引（不包含）
    #[serde(rename = "endRowIndex")]
    pub end_row_index: Option<i32>,
    /// 起始列索引（从0开始）
    #[serde(rename = "startColumnIndex")]
    pub start_column_index: Option<i32>,
    /// 结束列索引（不包含）
    #[serde(rename = "endColumnIndex")]
    pub end_column_index: Option<i32>,
}

/// 单元格样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellStyle {
    /// 文本格式
    #[serde(rename = "textFormat")]
    pub text_format: Option<TextFormat>,
    /// 背景颜色
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<Color>,
    /// 边框
    pub borders: Option<Borders>,
    /// 水平对齐
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<String>,
    /// 垂直对齐
    #[serde(rename = "verticalAlignment")]
    pub vertical_alignment: Option<String>,
    /// 文本换行
    #[serde(rename = "wrapStrategy")]
    pub wrap_strategy: Option<String>,
}

/// 文本格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextFormat {
    /// 字体
    pub font: Option<String>,
    /// 字号
    pub size: Option<i32>,
    /// 粗体
    pub bold: Option<bool>,
    /// 斜体
    pub italic: Option<bool>,
    /// 删除线
    #[serde(rename = "strikethrough")]
    pub strikethrough: Option<bool>,
    /// 下划线
    pub underline: Option<bool>,
    /// 前景色
    #[serde(rename = "foregroundColor")]
    pub foreground_color: Option<Color>,
}

/// 颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    /// 红色分量 (0-1)
    pub red: f32,
    /// 绿色分量 (0-1)
    pub green: f32,
    /// 蓝色分量 (0-1)
    pub blue: f32,
}

/// 边框
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Borders {
    /// 上边框
    pub top: Option<Border>,
    /// 下边框
    pub bottom: Option<Border>,
    /// 左边框
    pub left: Option<Border>,
    /// 右边框
    pub right: Option<Border>,
}

/// 边框样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Border {
    /// 边框样式
    pub style: Option<String>,
    /// 边框颜色
    pub color: Option<Color>,
    /// 边框宽度
    pub width: Option<i32>,
}

/// 设置单元格样式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStyleResponse {
    /// 操作结果
    pub data: Option<SetStyleResult>,
}

/// 设置样式操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStyleResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 操作是否成功
    pub success: bool,
    /// 设置的样式范围
    #[serde(rename = "styleRange")]
    pub style_range: Option<GridRange>,
}

impl ApiResponseTrait for SetStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设置单元格样式请求
pub struct SetStyleRequest {
    config: Config,
}

impl SetStyleRequest {
    /// 创建设置单元格样式请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/cells-format/set-style
    pub async fn execute(self, params: SetStyleParams) -> SDKResult<SetStyleResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::Style(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<SetStyleResponse> = ApiRequest::post(
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

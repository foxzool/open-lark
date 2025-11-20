//! 单元格样式操作服务
//!
//! 提供飞书电子表格v2版本的单元格样式设置功能，包括：
//! - 批量设置单元格样式
//! - 单元格字体样式设置
//! - 背景颜色和边框设置
//! - 文本对齐和格式化
//! - 数字格式设置

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 文本样式设置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextStyle {
    /// 字体名称
    #[serde(rename = "fontFamily", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    /// 字体大小
    #[serde(rename = "fontSize", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i32>,
    /// 是否粗体
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
    /// 字体颜色
    #[serde(rename = "foregroundColor", skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<Color>,
}

impl TextStyle {
    /// 创建新的文本样式
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置字体名称
    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = Some(font_family.into());
        self
    }

    /// 设置字体大小
    pub fn font_size(mut self, size: i32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// 设置为粗体
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    /// 设置为斜体
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    /// 设置删除线
    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    /// 设置下划线
    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = Some(underline);
        self
    }

    /// 设置字体颜色
    pub fn foreground_color(mut self, color: Color) -> Self {
        self.foreground_color = Some(color);
        self
    }
}

/// 单元格边框设置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Borders {
    /// 上边框
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<Border>,
    /// 下边框
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<Border>,
    /// 左边框
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<Border>,
    /// 右边框
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<Border>,
}

impl Borders {
    /// 创建新的边框设置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置上边框
    pub fn top(mut self, border: Border) -> Self {
        self.top = Some(border);
        self
    }

    /// 设置下边框
    pub fn bottom(mut self, border: Border) -> Self {
        self.bottom = Some(border);
        self
    }

    /// 设置左边框
    pub fn left(mut self, border: Border) -> Self {
        self.left = Some(border);
        self
    }

    /// 设置右边框
    pub fn right(mut self, border: Border) -> Self {
        self.right = Some(border);
        self
    }

    /// 设置所有边框
    pub fn all(mut self, border: Border) -> Self {
        self.top = Some(border.clone());
        self.bottom = Some(border.clone());
        self.left = Some(border.clone());
        self.right = Some(border);
        self
    }
}

/// 单个边框设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Border {
    /// 边框样式
    #[serde(rename = "borderStyle")]
    pub border_style: BorderStyle,
    /// 边框颜色
    #[serde(rename = "borderColor", skip_serializing_if = "Option::is_none")]
    pub border_color: Option<Color>,
}

impl Border {
    /// 创建新的边框
    pub fn new(border_style: BorderStyle) -> Self {
        Self {
            border_style,
            border_color: None,
        }
    }

    /// 设置边框颜色
    pub fn color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }
}

/// 边框样式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BorderStyle {
    /// 无边框
    None,
    /// 细实线
    Thin,
    /// 中等实线
    Medium,
    /// 粗实线
    Thick,
    /// 虚线
    Dotted,
    /// 点划线
    Dashed,
    /// 双线
    Double,
}

/// 颜色设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    /// 红色分量 (0-255)
    #[serde(rename = "red")]
    pub red: f32,
    /// 绿色分量 (0-255)
    #[serde(rename = "green")]
    pub green: f32,
    /// 蓝色分量 (0-255)
    #[serde(rename = "blue")]
    pub blue: f32,
    /// 透明度 (0-1)
    #[serde(rename = "alpha", skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,
}

impl Color {
    /// 从RGB值创建颜色
    pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: None,
        }
    }

    /// 从RGBA值创建颜色
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: Some(alpha),
        }
    }

    /// 从十六进制颜色字符串创建颜色 (如 "#FF0000")
    pub fn hex(hex_color: &str) -> Result<Self, String> {
        if !hex_color.starts_with('#') {
            return Err("颜色格式必须以 # 开头".to_string());
        }

        let hex = &hex_color[1..];
        if hex.len() != 6 {
            return Err("十六进制颜色必须是6位".to_string());
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "无效的红色分量")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "无效的绿色分量")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "无效的蓝色分量")?;

        Ok(Self::rgb(r as f32, g as f32, b as f32))
    }

    /// 常用颜色
    pub fn black() -> Self {
        Self::rgb(0.0, 0.0, 0.0)
    }
    pub fn white() -> Self {
        Self::rgb(255.0, 255.0, 255.0)
    }
    pub fn red() -> Self {
        Self::rgb(255.0, 0.0, 0.0)
    }
    pub fn green() -> Self {
        Self::rgb(0.0, 255.0, 0.0)
    }
    pub fn blue() -> Self {
        Self::rgb(0.0, 0.0, 255.0)
    }
}

/// 文本对齐方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TextAlignment {
    /// 左对齐
    Left,
    /// 居中对齐
    Center,
    /// 右对齐
    Right,
}

/// 垂直对齐方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerticalAlignment {
    /// 顶部对齐
    Top,
    /// 中间对齐
    Middle,
    /// 底部对齐
    Bottom,
}

/// 单元格样式设置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CellStyle {
    /// 文本样式
    #[serde(rename = "textStyle", skip_serializing_if = "Option::is_none")]
    pub text_style: Option<TextStyle>,
    /// 背景颜色
    #[serde(rename = "backgroundColor", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,
    /// 边框
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borders: Option<Borders>,
    /// 水平对齐
    #[serde(
        rename = "horizontalAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_alignment: Option<TextAlignment>,
    /// 垂直对齐
    #[serde(rename = "verticalAlignment", skip_serializing_if = "Option::is_none")]
    pub vertical_alignment: Option<VerticalAlignment>,
    /// 文本换行
    #[serde(rename = "wrapText", skip_serializing_if = "Option::is_none")]
    pub wrap_text: Option<bool>,
    /// 数字格式
    #[serde(rename = "numberFormat", skip_serializing_if = "Option::is_none")]
    pub number_format: Option<String>,
}

impl CellStyle {
    /// 创建新的单元格样式
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文本样式
    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = Some(text_style);
        self
    }

    /// 设置背景颜色
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    /// 设置边框
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = Some(borders);
        self
    }

    /// 设置水平对齐
    pub fn horizontal_alignment(mut self, alignment: TextAlignment) -> Self {
        self.horizontal_alignment = Some(alignment);
        self
    }

    /// 设置垂直对齐
    pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = Some(alignment);
        self
    }

    /// 设置文本换行
    pub fn wrap_text(mut self, wrap: bool) -> Self {
        self.wrap_text = Some(wrap);
        self
    }

    /// 设置数字格式
    pub fn number_format(mut self, format: impl Into<String>) -> Self {
        self.number_format = Some(format.into());
        self
    }
}

/// 样式更新项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleUpdate {
    /// 范围
    #[serde(rename = "range")]
    pub range: String,
    /// 样式
    #[serde(rename = "style")]
    pub style: CellStyle,
    /// 样式ID (用于更新特定样式)
    #[serde(rename = "styleId", skip_serializing_if = "Option::is_none")]
    pub style_id: Option<String>,
}

impl StyleUpdate {
    /// 创建新的样式更新
    pub fn new(range: impl Into<String>, style: CellStyle) -> Self {
        Self {
            range: range.into(),
            style,
            style_id: None,
        }
    }

    /// 设置样式ID
    pub fn style_id(mut self, style_id: impl Into<String>) -> Self {
        self.style_id = Some(style_id.into());
        self
    }
}

/// 批量样式更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateStylesRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 样式更新列表
    pub styles: Vec<StyleUpdate>,
}

impl BatchUpdateStylesRequest {
    /// 创建新的批量样式更新请求
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            styles: vec![],
        }
    }

    /// 添加样式更新
    pub fn add_style(mut self, style_update: StyleUpdate) -> Self {
        self.styles.push(style_update);
        self
    }

    /// 添加多个样式更新
    pub fn add_styles(mut self, style_updates: Vec<StyleUpdate>) -> Self {
        self.styles.extend(style_updates);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }

        if self.styles.is_empty() {
            return Err("至少需要添加一个样式更新".to_string());
        }

        if self.styles.len() > 500 {
            return Err("单次批量更新最多支持500个样式".to_string());
        }

        for (index, style_update) in self.styles.iter().enumerate() {
            if style_update.range.trim().is_empty() {
                return Err(format!("第{}个样式更新的范围不能为空", index + 1));
            }
        }

        Ok(())
    }
}

/// 批量样式更新响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateStylesResponse {
    /// 响应数据
    pub data: BatchUpdateStylesData,
}

/// 批量样式更新数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateStylesData {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 更新的样式信息
    #[serde(rename = "styleUpdates")]
    pub style_updates: Vec<StyleUpdateInfo>,
}

/// 样式更新信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleUpdateInfo {
    /// 范围
    #[serde(rename = "range")]
    pub range: String,
    /// 样式ID
    #[serde(rename = "styleId")]
    pub style_id: String,
}

impl Default for BatchUpdateStylesResponse {
    fn default() -> Self {
        Self {
            data: BatchUpdateStylesData {
                spreadsheet_token: String::new(),
                style_updates: vec![],
            },
        }
    }
}

impl ApiResponseTrait for BatchUpdateStylesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 单元格样式操作服务
#[derive(Clone, Debug)]
pub struct StyleOperationsService {
    config: Config,
}

impl StyleOperationsService {
    /// 创建样式操作服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量更新单元格样式
    ///
    /// 批量设置指定范围内单元格的样式，支持字体、颜色、边框、对齐等多种样式设置。
    ///
    /// # 参数
    /// - `request`: 批量样式更新请求
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// // 创建样式
    /// let style = CellStyle::new()
    ///     .text_style(TextStyle::new()
    ///         .font_size(14)
    ///         .bold(true)
    ///         .foreground_color(Color::red()))
    ///     .background_color(Color::hex("#FFFF00")?)
    ///     .horizontal_alignment(TextAlignment::Center)
    ///     .wrap_text(true);
    ///
    /// let style_update = StyleUpdate::new("A1:D10", style);
    ///
    /// let request = BatchUpdateStylesRequest::new("spreadsheet_token")
    ///     .add_style(style_update);
    ///
    /// let response = service.batch_update_styles(request, None).await?;
    /// println!("更新了 {} 个样式", response.data.style_updates.len());
    /// ```
    pub async fn batch_update_styles(
        &self,
        request: BatchUpdateStylesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<BatchUpdateStylesResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::PUT,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/styles_batch_update",
                &request.spreadsheet_token
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(&request))?;

        // 发送请求
        let api_resp =
            Transport::<BatchUpdateStylesResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 创建批量样式更新构建器
    pub fn batch_update_builder(
        &self,
        spreadsheet_token: impl Into<String>,
    ) -> BatchUpdateStylesBuilder {
        BatchUpdateStylesBuilder::new(self.config.clone() spreadsheet_token.into())
    }
}

/// 批量样式更新构建器
#[derive(Clone, Debug)]
pub struct BatchUpdateStylesBuilder {
    config: Config,
    spreadsheet_token: String,
    styles: Vec<StyleUpdate>,
}

impl BatchUpdateStylesBuilder {
    /// 创建新的批量样式更新构建器实例
    pub fn new(config: Config, spreadsheet_token: String) -> Self {
        Self {
            config,
            spreadsheet_token,
            styles: vec![],
        }
    }

    /// 添加样式更新
    pub fn add_style(mut self, range: impl Into<String>, style: CellStyle) -> Self {
        let style_update = StyleUpdate::new(range, style);
        self.styles.push(style_update);
        self
    }

    /// 添加多个样式更新
    pub fn add_styles(mut self, style_updates: Vec<StyleUpdate>) -> Self {
        self.styles.extend(style_updates);
        self
    }

    /// 执行批量更新请求
    pub async fn execute(self) -> SDKResult<Response<BatchUpdateStylesResponse>> {
        let request = BatchUpdateStylesRequest::new(self.spreadsheet_token).add_styles(self.styles);

        let service = StyleOperationsService {
            config: self.config,
        };
        service.batch_update_styles(request, None).await
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let rgb_color = Color::rgb(255.0, 0.0, 0.0);
        assert_eq!(rgb_color.red, 255.0);
        assert_eq!(rgb_color.green, 0.0);
        assert_eq!(rgb_color.blue, 0.0);

        let rgba_color = Color::rgba(100.0, 150.0, 200.0, 0.5);
        assert_eq!(rgba_color.alpha, Some(0.5));

        let hex_color = Color::hex("#FF0000").unwrap();
        assert_eq!(hex_color.red, 255.0);
        assert_eq!(hex_color.green, 0.0);
        assert_eq!(hex_color.blue, 0.0);
    }

    #[test]
    fn test_color_presets() {
        let black = Color::black();
        assert_eq!(black.red, 0.0);
        assert_eq!(black.green, 0.0);
        assert_eq!(black.blue, 0.0);

        let white = Color::white();
        assert_eq!(white.red, 255.0);
        assert_eq!(white.green, 255.0);
        assert_eq!(white.blue, 255.0);
    }

    #[test]
    fn test_text_style_creation() {
        let text_style = TextStyle::new()
            .font_family("Arial")
            .font_size(12)
            .bold(true)
            .italic(false)
            .foreground_color(Color::blue());

        assert_eq!(text_style.font_family, Some("Arial".to_string()));
        assert_eq!(text_style.font_size, Some(12));
        assert_eq!(text_style.bold, Some(true));
        assert_eq!(text_style.italic, Some(false));
        assert!(text_style.foreground_color.is_some());
    }

    #[test]
    fn test_borders_creation() {
        let border = Border::new(BorderStyle::Thin).color(Color::black());
        let borders = Borders::new()
            .top(border.clone())
            .bottom(border.clone())
            .left(border.clone())
            .right(border);

        assert!(borders.top.is_some());
        assert!(borders.bottom.is_some());
        assert!(borders.left.is_some());
        assert!(borders.right.is_some());
    }

    #[test]
    fn test_cell_style_creation() {
        let style = CellStyle::new()
            .text_style(TextStyle::new().font_size(14).bold(true))
            .background_color(Color::hex("#FFFF00").unwrap())
            .horizontal_alignment(TextAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .wrap_text(true)
            .number_format("#,##0.00");

        assert!(style.text_style.is_some());
        assert!(style.background_color.is_some());
        assert_eq!(style.horizontal_alignment, Some(TextAlignment::Center));
        assert_eq!(style.vertical_alignment, Some(VerticalAlignment::Middle));
        assert_eq!(style.wrap_text, Some(true));
        assert_eq!(style.number_format, Some("#,##0.00".to_string()));
    }

    #[test]
    fn test_style_update_creation() {
        let style = CellStyle::new().text_style(TextStyle::new().bold(true));
        let style_update = StyleUpdate::new("A1:D10", style).style_id("style_123");

        assert_eq!(style_update.range, "A1:D10");
        assert!(style_update.style_id.is_some());
        assert_eq!(style_update.style_id.unwrap(), "style_123");
    }

    #[test]
    fn test_batch_update_styles_request() {
        let style1 = CellStyle::new().background_color(Color::red());
        let style2 = CellStyle::new().text_style(TextStyle::new().italic(true));

        let update1 = StyleUpdate::new("A1:A5", style1);
        let update2 = StyleUpdate::new("B1:B5", style2);

        let request = BatchUpdateStylesRequest::new("test_token")
            .add_style(update1)
            .add_style(update2);

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.styles.len(), 2);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_batch_update_styles_request_validation() {
        // 测试正常请求
        let valid_request = BatchUpdateStylesRequest::new("token")
            .add_style(StyleUpdate::new("A1:B2", CellStyle::new()));
        assert!(valid_request.validate().is_ok());

        // 测试空token
        let empty_token_request = BatchUpdateStylesRequest::new("")
            .add_style(StyleUpdate::new("A1:B2", CellStyle::new()));
        assert!(empty_token_request.validate().is_err());

        // 测试空样式列表
        let empty_styles_request = BatchUpdateStylesRequest::new("token");
        assert!(empty_styles_request.validate().is_err());

        // 测试过多样式
        let mut too_many_request = BatchUpdateStylesRequest::new("token");
        for _ in 0..501 {
            too_many_request =
                too_many_request.add_style(StyleUpdate::new("A1:B2", CellStyle::new()));
        }
        assert!(too_many_request.validate().is_err());
    }

    #[test]
    fn test_batch_update_styles_builder() {
        let config = openlark_core::config::Config::default();
        let style1 = CellStyle::new().background_color(Color::red());
        let style2 = CellStyle::new().text_style(TextStyle::new().bold(true));

        let builder = BatchUpdateStylesBuilder::new(config, "test_token".to_string())
            .add_style("A1:D5", style1)
            .add_style("F1:F10", style2);

        assert_eq!(builder.spreadsheet_token, "test_token");
        assert_eq!(builder.styles.len(), 2);
        assert!(!format!("{:?}", builder).is_empty());
    }

    #[test]
    fn test_style_operations_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = StyleOperationsService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_serialization() {
        let original_style = CellStyle::new()
            .text_style(TextStyle::new().font_size(14).bold(true))
            .background_color(Color::rgb(255.0, 0.0, 0.0));

        let serialized = serde_json::to_string(&original_style).unwrap();
        let deserialized: CellStyle = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.text_style.is_some());
        assert!(deserialized.background_color.is_some());

        let text_style = deserialized.text_style.unwrap();
        assert_eq!(text_style.font_size, Some(14));
        assert_eq!(text_style.bold, Some(true));
    }

    #[test]
    fn test_border_style_serialization() {
        let thin_border = BorderStyle::Thin;
        let serialized = serde_json::to_string(&thin_border).unwrap();
        assert_eq!(serialized, "\"THIN\"");

        let dashed_border = BorderStyle::Dashed;
        let serialized = serde_json::to_string(&dashed_border).unwrap();
        assert_eq!(serialized, "\"DASHED\"");
    }

    #[test]
    fn test_text_alignment_serialization() {
        let center = TextAlignment::Center;
        let serialized = serde_json::to_string(&center).unwrap();
        assert_eq!(serialized, "\"CENTER\"");

        let right = TextAlignment::Right;
        let serialized = serde_json::to_string(&right).unwrap();
        assert_eq!(serialized, "\"RIGHT\"");
    }

    #[test]
    fn test_complex_style_combination() {
        let complex_style = CellStyle::new()
            .text_style(
                TextStyle::new()
                    .font_family("Microsoft YaHei")
                    .font_size(16)
                    .bold(true)
                    .italic(true)
                    .underline(true)
                    .strikethrough(false)
                    .foreground_color(Color::hex("#1E88E5").unwrap()),
            )
            .background_color(Color::rgba(245.0, 245.0, 245.0, 0.8))
            .borders(
                Borders::new()
                    .all(Border::new(BorderStyle::Medium).color(Color::hex("#9E9E9E").unwrap())),
            )
            .horizontal_alignment(TextAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .wrap_text(true)
            .number_format("¥#,##0.00");

        // 验证所有设置都正确
        assert!(complex_style.text_style.is_some());
        assert!(complex_style.background_color.is_some());
        assert!(complex_style.borders.is_some());
        assert_eq!(complex_style.wrap_text, Some(true));
        assert_eq!(complex_style.number_format, Some("¥#,##0.00".to_string()));

        let text_style = complex_style.text_style.unwrap();
        assert_eq!(text_style.font_family, Some("Microsoft YaHei".to_string()));
        assert_eq!(text_style.font_size, Some(16));
        assert_eq!(text_style.bold, Some(true));
        assert_eq!(text_style.italic, Some(true));
        assert_eq!(text_style.underline, Some(true));
        assert_eq!(text_style.strikethrough, Some(false));
    }

    #[test]
    fn test_border_edge_cases() {
        // 测试无边框
        let none_border = Border::new(BorderStyle::None);
        assert_eq!(none_border.border_style, BorderStyle::None);

        // 测试双线边框
        let double_border = Border::new(BorderStyle::Double).color(Color::green());
        assert_eq!(double_border.border_style, BorderStyle::Double);
        assert!(double_border.border_color.is_some());

        // 测试部分边框设置
        let partial_borders = Borders::new()
            .top(Border::new(BorderStyle::Thin))
            .bottom(Border::new(BorderStyle::Thick));

        assert!(partial_borders.top.is_some());
        assert!(partial_borders.bottom.is_some());
        assert!(partial_borders.left.is_none());
        assert!(partial_borders.right.is_none());
    }
}

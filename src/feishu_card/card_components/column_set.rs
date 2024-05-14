use serde::{Deserialize, Serialize};

/// 多列布局的参数
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardColumnSet {
    /// 多列布局容器的标识，固定取值：column_set。
    pub tag: String,
    /// 移动端和 PC 端的窄屏幕下，各列的自适应方式。取值：
    ///
    /// - none：不做布局上的自适应，在窄屏幕下按比例压缩列宽度。
    /// - stretch：列布局变为行布局，且每列（行）宽度强制拉伸为 100%，所有列自适应为上下堆叠排布。
    /// - flow：列流式排布（自动换行），当一行展示不下一列时，自动换至下一行展示。
    /// - bisect：两列等分布局。
    /// - trisect：三列等分布局。
    ///
    /// 默认值：none。
    pub flex_model: FeishuCardFlexMode,
    /// 多列布局的背景色样式。取值：
    ///
    /// - default：默认的白底样式，dark mode 下为黑底。
    /// - grey：灰底样式。
    ///
    /// 当存在多列布局的嵌套时，上层多列布局的颜色覆盖下层多列布局的颜色。
    pub background_style: Option<FeishuCardBackgroundStyle>,
    /// 多列布局内，各列之间的水平分栏间距。取值：
    ///
    /// default：默认间距。
    /// small：窄间距。
    pub horizontal_spacing: Option<FeishuCardHorizontalSpacing>,
    /// 多列布局容器内，各个列容器的配置信息
    pub columns: Option<Vec<FeishuCardColumn>>,
}

impl Default for FeishuCardColumnSet {
    fn default() -> Self {
        FeishuCardColumnSet {
            tag: "column_set".to_string(),
            flex_model: FeishuCardFlexMode::None,
            background_style: None,
            horizontal_spacing: None,
            columns: None,
        }
    }
}

/// 移动端和 PC 端的窄屏幕下，各列的自适应方式
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FeishuCardFlexMode {
    /// 不做布局上的自适应，在窄屏幕下按比例压缩列宽度。
    #[default]
    None,
    /// 列布局变为行布局，且每列（行）宽度强制拉伸为 100%，所有列自适应为上下堆叠排布。
    Stretch,
    /// 列流式排布（自动换行），当一行展示不下一列时，自动换至下一行展示。
    Flow,
    /// 两列等分布局。
    Bisect,
    /// 三列等分布局。
    Trisect,
}

/// 多列布局的背景色样式。取值：
///
/// - default：默认的白底样式，dark mode 下为黑底。
/// - grey：灰底样式。
///
/// 当存在多列布局的嵌套时，上层多列布局的颜色覆盖下层多列布局的颜色。
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FeishuCardBackgroundStyle {
    /// 默认的白底样式，dark mode 下为黑底。
    #[default]
    Default,
    /// 灰底样式。
    Grey,
}

/// 多列布局内，各列之间的水平分栏间距。取值：
///
/// - default：默认间距。
/// - small：窄间距。
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FeishuCardHorizontalSpacing {
    #[default]
    Default,
    Small,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardColumn {
    pub tag: String,
    pub width: Option<String>,
    pub weight: Option<String>,
    pub vertical_align: Option<String>,
    pub elements: Option<Vec<FeishuCardColumnSetElement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FeishuCardColumnSetElement {}

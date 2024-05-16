use serde::{Deserialize, Serialize};
use crate::feishu_card::card_components::interactive_components::button::FeishuCardButton;

/// 多列布局的参数
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardColumnSet {
    /// 多列布局容器的标识，固定取值：column_set。
    tag: String,
    /// 各列之间的水平分栏间距。取值：
    ///
    /// - default：默认间距，8px
    /// - small：窄间距，4px
    /// - large：大间距，12px
    /// - [0,28px]：自定义间距
    horizontal_spacing: Option<String>,
    /// 列容器水平对齐的方式。可取值：
    ///
    /// left：左对齐
    /// center：居中对齐
    /// right：右对齐
    horizontal_align: Option<String>,
    /// 列的外边距。值的取值范围为 [0,28]px。可选值：
    ///
    /// - 单值，如 "10px"，表示列的四个外边距都为 10 px。
    /// - 多值，如 "4px 12px 4px 12px"，表示列的上、右、下、左的外边距分别为 4px，12px，4px，12px。四个值必填，使用空格间隔。
    /// 注意：首行列的上外边距强制为 0，末行列的下外边距强制为 0。
    margin: Option<String>,
    /// 移动端和 PC 端的窄屏幕下，各列的自适应方式。取值：
    ///
    /// - none：不做布局上的自适应，在窄屏幕下按比例压缩列宽度。
    /// - stretch：列布局变为行布局，且每列（行）宽度强制拉伸为 100%，所有列自适应为上下堆叠排布。
    /// - flow：列流式排布（自动换行），当一行展示不下一列时，自动换至下一行展示。
    /// - bisect：两列等分布局。
    /// - trisect：三列等分布局。
    ///
    /// 默认值：none。
    flex_mode: String,
    /// 分栏的背景色样式。可取值：
    ///
    /// default：默认的白底样式，客户端深色主题下为黑底样式
    /// 卡片支持的颜色枚举值和 RGBA 语法自定义颜色。参考颜色枚举值。
    /// 注意：当存在分栏的嵌套时，上层分栏的颜色覆盖下层分栏的颜色。
    background_style: Option<String>,
    /// 多列布局容器内，各个列容器的配置信息
    columns: Option<Vec<FeishuCardColumSetColumn>>,
}

impl Default for FeishuCardColumnSet {
    fn default() -> Self {
        FeishuCardColumnSet {
            tag: "column_set".to_string(),
            background_style: None,
            horizontal_spacing: None,
            horizontal_align: None,
            columns: None,
            margin: None,
            flex_mode: "".to_string(),
        }
    }
}

///分栏中列（column
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardColumSetColumn {
    /// 列的标签，固定取值为 column。
    tag: String,

    /// 列的背景色样式。可取值：
    ///
    /// - default：默认的白底样式，客户端深色主题下为黑底样式
    /// - 卡片支持的颜色枚举值和 RGBA 语法自定义颜色。参考颜色枚举值
    background_style: Option<String>,

    /// 列宽度。仅 flex_mode 为 none 时，生效此属性。取值：
    ///
    /// auto：列宽度与列内元素宽度一致
    /// weighted：列宽度按 weight 参数定义的权重分布
    /// 具体数值，如 100px。取值范围为 [16,600]px。V7.4 及以上版本支持该枚举
    width: Option<String>,
    /// 当 width 字段取值为 weighted 时生效，表示当前列的宽度占比。取值范围为 1 ~ 5 之间的整数。
    weight: Option<String>,
    /// 列垂直居中的方式。可取值：
    ///
    /// - top：上对齐
    /// - center：居中对齐
    /// - bottom：下对齐
    vertical_align: Option<String>,
    /// 列内组件的纵向间距。取值：
    ///
    /// default：默认间距，8px
    /// medium：中等间距
    /// large：大间距
    /// 具体数值，如 8px。取值范围为 [0,28]px
    vertical_spacing: Option<String>,
    /// 列的内边距。值的取值范围为 [0,28]px。可选值：
    ///
    /// - 单值，如 "10px"，表示列的四个外边距都为 10 px。
    /// - 多值，如 "4px 12px 4px 12px"，表示列的上、右、下、左的外边距分别为 4px，12px，4px，12px。四个值必填，使用空格间隔。
    padding: Option<String>,
    /// 列容器中内嵌的组件。可内嵌组件参考上文嵌套关系
    elements: Option<Vec<FeishuCardColumnSetElement>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FeishuCardColumnSetElement {
    Button(FeishuCardButton)
}

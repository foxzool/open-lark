use serde::{Deserialize, Serialize};

use crate::card::{components::CardElement, href::FeishuCardHrefVal};

/// 多列布局的参数
#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnSetContainer {
    /// 多列布局容器的标识，固定取值：column_set。
    tag: String,
    /// 各列之间的水平分栏间距。取值：
    ///
    /// - default：默认间距，8px
    /// - small：窄间距，4px
    /// - large：大间距，12px
    /// - [0,28px]：自定义间距
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_spacing: Option<String>,
    /// 列容器水平对齐的方式。可取值：
    ///
    /// left：左对齐
    /// center：居中对齐
    /// right：右对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_align: Option<String>,
    /// 列的外边距。值的取值范围为 [0,28]px。可选值：
    ///
    /// - 单值，如 "10px"，表示列的四个外边距都为 10 px。
    /// - 多值，如 "4px 12px 4px 12px"，表示列的上、右、下、左的外边距分别为
    ///   4px，12px，4px，12px。四个值必填，使用空格间隔。
    ///
    /// 注意：首行列的上外边距强制为 0，末行列的下外边距强制为 0。
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    background_style: Option<String>,
    /// 多列布局容器内，各个列容器的配置信息
    columns: Vec<Column>,
    /// 设置点击分栏时的交互配置。当前仅支持跳转交互。如果布局容器内有交互组件，
    /// 则优先响应交互组件定义的交互。
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ColumnAction>,
}

impl Default for ColumnSetContainer {
    fn default() -> Self {
        ColumnSetContainer {
            tag: "column_set".to_string(),
            background_style: None,
            horizontal_spacing: None,
            horizontal_align: None,
            columns: vec![],
            margin: None,
            flex_mode: "".to_string(),
            action: None,
        }
    }
}

impl ColumnSetContainer {
    pub fn new() -> Self {
        ColumnSetContainer::default()
    }

    pub fn horizontal_spacing(mut self, horizontal_spacing: impl ToString) -> Self {
        self.horizontal_spacing = Some(horizontal_spacing.to_string());
        self
    }

    pub fn horizontal_align(mut self, horizontal_align: impl ToString) -> Self {
        self.horizontal_align = Some(horizontal_align.to_string());
        self
    }

    pub fn margin(mut self, margin: impl ToString) -> Self {
        self.margin = Some(margin.to_string());
        self
    }

    pub fn flex_mode(mut self, flex_mode: impl ToString) -> Self {
        self.flex_mode = flex_mode.to_string();
        self
    }

    pub fn background_style(mut self, background_style: impl ToString) -> Self {
        self.background_style = Some(background_style.to_string());
        self
    }

    pub fn columns(mut self, columns: Vec<Column>) -> Self {
        self.columns = columns;
        self
    }

    pub fn action(mut self, action: ColumnAction) -> Self {
        self.action = Some(action);
        self
    }
}

/// 分栏中列column
#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    /// 列的标签，固定取值为 column。
    tag: String,
    /// 列的背景色样式。可取值：
    ///
    /// - default：默认的白底样式，客户端深色主题下为黑底样式
    /// - 卡片支持的颜色枚举值和 RGBA 语法自定义颜色。参考颜色枚举值
    #[serde(skip_serializing_if = "Option::is_none")]
    background_style: Option<String>,
    /// 列宽度。仅 flex_mode 为 none 时，生效此属性。取值：
    ///
    /// auto：列宽度与列内元素宽度一致
    /// weighted：列宽度按 weight 参数定义的权重分布
    /// 具体数值，如 100px。取值范围为 [16,600]px。V7.4 及以上版本支持该枚举
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    /// 当 width 字段取值为 weighted 时生效，表示当前列的宽度占比。取值范围为 1 ~ 5 之间的整数。\
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<u32>,
    /// 列垂直居中的方式。可取值：
    ///
    /// - top：上对齐
    /// - center：居中对齐
    /// - bottom：下对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_align: Option<String>,
    /// 列内组件的纵向间距。取值：
    ///
    /// - default：默认间距，8px
    /// - medium：中等间距
    /// - large：大间距
    /// - 具体数值，如 8px。取值范围为 [0,28]px
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_spacing: Option<String>,
    /// 列的内边距。值的取值范围为 [0,28]px。可选值：
    ///
    /// - 单值，如 "10px"，表示列的四个外边距都为 10 px。
    /// - 多值，如 "4px 12px 4px 12px"，表示列的上、右、下、左的外边距分别为
    ///   4px，12px，4px，12px。四个值必填，使用空格间隔。
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<String>,
    /// 列容器中内嵌的组件。可内嵌组件参考上文嵌套关系
    elements: Vec<CardElement>,
    /// 设置点击列时的交互配置。当前仅支持跳转交互。如果布局容器内有交互组件，
    /// 则优先响应交互组件定义的交互。
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ColumnAction>,
}

impl Default for Column {
    fn default() -> Self {
        Column {
            tag: "column".to_string(),
            background_style: None,
            width: None,
            weight: None,
            vertical_align: None,
            vertical_spacing: None,
            padding: None,
            elements: vec![],
            action: None,
        }
    }
}

impl Column {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn background_style(mut self, background_style: impl ToString) -> Self {
        self.background_style = Some(background_style.to_string());
        self
    }

    pub fn width(mut self, width: impl ToString) -> Self {
        self.width = Some(width.to_string());
        self
    }

    pub fn weight(mut self, weight: u32) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn vertical_align(mut self, vertical_align: impl ToString) -> Self {
        self.vertical_align = Some(vertical_align.to_string());
        self
    }

    pub fn vertical_spacing(mut self, vertical_spacing: impl ToString) -> Self {
        self.vertical_spacing = Some(vertical_spacing.to_string());
        self
    }

    pub fn padding(mut self, padding: impl ToString) -> Self {
        self.padding = Some(padding.to_string());
        self
    }

    pub fn elements(mut self, elements: Vec<CardElement>) -> Self {
        self.elements = elements;
        self
    }

    pub fn action(mut self, action: ColumnAction) -> Self {
        self.action = Some(action);
        self
    }
}

/// 设置点击分栏时的交互配置。当前仅支持跳转交互。如果布局容器内有交互组件，
/// 则优先响应交互组件定义的交互。
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ColumnAction {
    /// 配置各个端的链接地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_url: Option<FeishuCardHrefVal>,
}

impl ColumnAction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn multi_url(mut self, multi_url: FeishuCardHrefVal) -> Self {
        self.multi_url = Some(multi_url);
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::{
        components::containers::column_set::{Column, ColumnAction, ColumnSetContainer},
        href::FeishuCardHrefVal,
    };

    #[test]
    fn test_column_set() {
        let column_set = ColumnSetContainer::new()
            .horizontal_spacing("large")
            .horizontal_align("left")
            .margin("0px")
            .flex_mode("none")
            .background_style("default")
            .action(
                ColumnAction::new().multi_url(
                    FeishuCardHrefVal::new()
                        .url("https://open.feishu.cn")
                        .pc_url("https://open.feishu.com")
                        .ios_url("https://developer.apple.com/")
                        .android_url("https://developer.android.com/"),
                ),
            )
            .columns(vec![Column::new()
                .background_style("default")
                .width("auto")
                .weight(1)
                .vertical_align("center")
                .vertical_spacing("4px")
                .padding("8px")
                .action(
                    ColumnAction::new().multi_url(
                        FeishuCardHrefVal::new()
                            .url("https://www.baidu.com")
                            .pc_url("https://www.baidu.com")
                            .ios_url("https://www.google.com")
                            .android_url("https://www.apple.com.cn"),
                    ),
                )]);

        let expect = json!({
          "tag": "column_set", // 分栏的标签。
          "horizontal_spacing": "large", // 分栏中列容器之间的间距。默认值 default（为 8px）。
          "horizontal_align": "left", // 列容器水平对齐的方式。默认值 left。
          "margin": "0px", // 列容器的外边距。
          "flex_mode": "none", // 移动端和 PC 端的窄屏幕下，各列的自适应方式。默认值 none。
          "background_style": "default", // 分栏的背景色样式。默认值 default。
          "action": { // 在此处设置点击分栏时的交互配置。
            "multi_url": {
              "url": "https://open.feishu.cn",
              "pc_url": "https://open.feishu.com",
              "ios_url": "https://developer.apple.com/",
              "android_url": "https://developer.android.com/"
            }
          },
          "columns": [
            // 列配置
            {
              "tag": "column",
              "background_style": "default", // 列的背景色样式。默认值 default。
              "width": "auto", // 列容器的宽度。默认值 auto。
              "weight": 1, // 当 width 取值 weighted 时生效，表示当前列的宽度占比。
              "vertical_align": "center", // 列垂直居中的方式。
              "vertical_spacing": "4px", // 列内子组件纵向间距。默认值 default（8px）。
              "padding": "8px", // 列容器的内边距。默认值 0px。
              "action": {
                // 在此处设置点击列时的交互配置。
                "multi_url": {
                  "url": "https://www.baidu.com",
                  "pc_url": "https://www.baidu.com",
                  "ios_url": "https://www.google.com",
                  "android_url": "https://www.apple.com.cn"
                }
              },
              "elements": [] // 列容器内嵌的组件，不支持内嵌表格组件、多图混排组件和表单容器。
            }
          ]
        });

        assert_eq!(json!(column_set), expect);
    }
}

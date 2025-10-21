use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardTable {
    /// 组件的标签。表格组件的固定取值为 table。
    tag: String,
    /// 每页最大展示的数据行数。支持 [1,10] 整数。
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    /// 表格的行高。单元格高度如无法展示一整行内容，则上下裁剪内容。可取值：
    ///
    /// low：低
    /// middle：中
    /// high：高
    /// [32,124]px：自定义行高，单位为像素，如 40px。取值范围是 [32,124]
    #[serde(skip_serializing_if = "Option::is_none")]
    row_height: Option<String>,
    /// 表头样式风格。
    #[serde(skip_serializing_if = "Option::is_none")]
    header_style: Option<FeishuCardTableHeaderStyle>,
    /// 列对象数组
    columns: Vec<FeishuCardTableColumn>,
    /// 行对象数组。与列定义对应的数据。用 "name":VALUE
    /// 的形式，定义每一行的数据内容。name即你自定义的列标记。
    rows: Value,
}

impl Default for FeishuCardTable {
    fn default() -> Self {
        FeishuCardTable {
            tag: "table".to_string(),
            page_size: None,
            row_height: None,
            header_style: None,
            columns: vec![],
            rows: Value::Null,
        }
    }
}

/// 用于设置表头的样式、风格等。header_style 的子字段如下表所示。
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardTableHeaderStyle {
    /// 表头文本对齐方式。可取值：
    ///
    /// - left：左对齐
    /// - center：居中对齐
    /// - right：右对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    text_align: Option<String>,
    /// 表头文本大小。可取值：
    ///
    /// normal：正文（14px）
    /// heading：标题（16px）
    #[serde(skip_serializing_if = "Option::is_none")]
    text_size: Option<String>,
    /// 表头背景色。可取值：
    ///
    /// - grey：灰色
    /// - none：无背景色
    #[serde(skip_serializing_if = "Option::is_none")]
    background_style: Option<String>,
    /// 文本颜色。可取值：
    ///
    /// - default：客户端浅色主题模式下为黑色；客户端深色主题模式下为白色
    /// - grey：灰色
    #[serde(skip_serializing_if = "Option::is_none")]
    text_color: Option<String>,
    /// 表头文本是否加粗。可取值：
    ///
    /// - true：加粗
    /// - false：不加粗
    #[serde(skip_serializing_if = "Option::is_none")]
    bold: Option<bool>,
    /// 表头文本是否加粗。可取值：
    /// 表头文本的行数。支持大于等于 1 的整数。
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<i32>,
}

impl FeishuCardTableHeaderStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text_align(mut self, text_align: &str) -> Self {
        self.text_align = Some(text_align.to_string());
        self
    }

    pub fn text_size(mut self, text_size: &str) -> Self {
        self.text_size = Some(text_size.to_string());
        self
    }

    pub fn background_style(mut self, background_style: &str) -> Self {
        self.background_style = Some(background_style.to_string());
        self
    }

    pub fn text_color(mut self, text_color: &str) -> Self {
        self.text_color = Some(text_color.to_string());
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    pub fn lines(mut self, lines: i32) -> Self {
        self.lines = Some(lines);
        self
    }
}

/// 用于定义表格的列。最多支持添加 10 列，超出 10 列的内容不展示。
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardTableColumn {
    /// 自定义列的标记。用于唯一指定行数据对象数组中，需要将数据填充至这一行的具体哪个单元格中。
    name: String,
    /// 在表头展示的列名称。不填或为空则不展示列名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    /// 列宽度。可取值：
    ///
    /// - auto：自适应内容宽度
    /// - 自定义宽度：自定义表格的列宽度，如 120px。取值范围是 [80px,600px] 的整数
    /// - 自定义宽度百分比：自定义列宽度占当前表格画布宽度的百分比（表格画布宽度 =
    ///   卡片宽度-卡片左右内边距），如 25%。取值范围是 [1%,100%]
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,
    /// 列内数据对齐方式。可选值：
    ///
    /// - left：左对齐
    /// - center：居中对齐
    /// - right：右对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_align: Option<String>,
    /// 列数据类型。可选值：
    ///
    /// - text：不带格式的普通文本
    /// - options：选项标签
    /// - number：数字。默认在单元格中右对齐展示。若选择该数据类型，你可继续在 column 中添加 format
    ///   字段，设置数字的子格式属性
    data_type: String,
    /// 该字段仅当 data_type 为 number
    /// 时生效，你可以在该字段内选择设置小数点位数、货币单位以及千分位样式。
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<FeishuCardTableColumnFormat>,
}

/// 设置小数点位数、货币单位以及千分位样式。
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeishuCardTableColumnFormat {
    /// 数字的小数点位数。默认不限制小数点位数，原样透传展示开发者输入的数字。可填 0~10
    /// 的整数。小数点位数为 0 表示取整数。
    #[serde(skip_serializing_if = "Option::is_none")]
    precision: Option<i32>,
    /// 数字前的货币单位。不填或为空不展示。可填 1 个字符的货币单位文本，如 “¥”。
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    /// 是否生效按千分位逗号分割的数字样式。
    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<bool>,
}

impl FeishuCardTableColumnFormat {
    pub fn new() -> Self {
        FeishuCardTableColumnFormat::default()
    }

    pub fn precision(mut self, precision: i32) -> Self {
        self.precision = Some(precision);
        self
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_string());
        self
    }

    pub fn separator(mut self, separator: bool) -> Self {
        self.separator = Some(separator);
        self
    }
}

impl Default for FeishuCardTableColumn {
    fn default() -> Self {
        FeishuCardTableColumn {
            name: "".to_string(),
            display_name: None,
            width: None,
            horizontal_align: None,
            data_type: "".to_string(),
            format: None,
        }
    }
}

impl FeishuCardTableColumn {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn display_name(mut self, display_name: &str) -> Self {
        self.display_name = Some(display_name.to_string());
        self
    }

    pub fn width(mut self, width: &str) -> Self {
        self.width = Some(width.to_string());
        self
    }

    pub fn horizontal_align(mut self, horizontal_align: &str) -> Self {
        self.horizontal_align = Some(horizontal_align.to_string());
        self
    }

    pub fn data_type(mut self, data_type: &str) -> Self {
        self.data_type = data_type.to_string();
        self
    }

    pub fn format(mut self, format: FeishuCardTableColumnFormat) -> Self {
        self.format = Some(format);
        self
    }
}

impl FeishuCardTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_zie(mut self, page_zie: i32) -> Self {
        self.page_size = Some(page_zie);
        self
    }

    pub fn row_height(mut self, row_height: &str) -> Self {
        self.row_height = Some(row_height.to_string());
        self
    }

    pub fn header_style(mut self, header_style: FeishuCardTableHeaderStyle) -> Self {
        self.header_style = Some(header_style);
        self
    }

    pub fn columns(mut self, columns: Vec<FeishuCardTableColumn>) -> Self {
        self.columns = columns;
        self
    }

    pub fn rows(mut self, rows: Value) -> Self {
        self.rows = rows;
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::components::content_components::table::{
        FeishuCardTable, FeishuCardTableColumn, FeishuCardTableColumnFormat,
        FeishuCardTableHeaderStyle,
    };

    #[test]
    fn test_table() {
        let table = FeishuCardTable::new()
            .page_zie(1)
            .row_height("middle")
            .header_style(
                FeishuCardTableHeaderStyle::new()
                    .bold(true)
                    .background_style("grey")
                    .lines(1)
                    .text_size("heading")
                    .text_align("center")
                ,
            )
            .columns(vec![
                FeishuCardTableColumn::new()
                    .name("customer_name")
                    .display_name("客户名称")
                    .data_type("text"),
                FeishuCardTableColumn::new()
                    .name("customer_scale")
                    .display_name("客户规模")
                    .data_type("options")
                    .width("90px"),
                FeishuCardTableColumn::new()
                    .name("customer_arr")
                    .display_name("ARR(万元)")
                    .data_type("number")
                    .format(
                        FeishuCardTableColumnFormat::new()
                            .precision(2)
                            .symbol("¥"),
                    )
                    .width("120px"),
                FeishuCardTableColumn::new()
                    .name("customer_year")
                    .display_name("签约年限")
                    .data_type("text"),
            ])
            .rows(json!([
                    {
                        "customer_name": "飞书消息卡片是飞书中的一种功能，它允许用户通过机器人或应用以结构化（JSON）的方式发送和接收消息。",
                        "customer_scale": [
                            {
                                "text": "S2",
                                "color": "green"
                            }
                        ],
                        "customer_arr": 26.57774928467545,
                        "customer_year": "2年"
                    }
                ]));

        let json = json!( {
            "tag": "table",
            "page_size": 1,
            "row_height": "middle",
            "header_style": {
                "bold": true,
                "background_style": "grey",
                "lines": 1,
                "text_size": "heading",
                "text_align": "center"
            },
            "columns": [
                {
                    "name": "customer_name",
                    "display_name": "客户名称",
                    "data_type": "text"
                },
                {
                    "name": "customer_scale",
                    "display_name": "客户规模",
                    "data_type": "options",
                    "width": "90px"
                },
                {
                    "name": "customer_arr",
                    "display_name": "ARR(万元)",
                    "data_type": "number",
                    "format": {
                        "symbol": "¥",
                        "precision": 2
                    },
                    "width": "120px"
                },
                {
                    "name": "customer_year",
                    "display_name": "签约年限",
                    "data_type": "text"
                }
            ],
            "rows": [
                {
                    "customer_name": "飞书消息卡片是飞书中的一种功能，它允许用户通过机器人或应用以结构化（JSON）的方式发送和接收消息。",
                    "customer_scale": [
                        {
                            "text": "S2",
                            "color": "green"
                        }
                    ],
                    "customer_arr": 26.57774928467545,
                    "customer_year": "2年"
                }
            ]
        });

        assert_eq!(serde_json::to_value(&table).unwrap(), json);
    }
}

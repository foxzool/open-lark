use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardChart {
    /// 组件的标签，图标组件的标签为固定值 chart。
    tag: String,
    /// 图表的宽高比。支持以下比例：
    ///
    /// - 1:1
    /// - 2:1
    /// - 4:3
    /// - 16:9
    #[serde(skip_serializing_if = "Option::is_none")]
    aspect_ratio: Option<String>,
    /// 图表的主题样式。当图表内存在多个颜色时，可使用该字段调整颜色样式。若你在 chart_spec
    /// 字段中声明了样式类属性，该字段无效。
    ///
    /// - brand：默认样式，与飞书客户端主题样式一致。
    /// - rainbow：同色系彩虹色。
    /// - complementary：互补色。
    /// - converse：反差色。
    /// - primary：主色。
    #[serde(skip_serializing_if = "Option::is_none")]
    color_theme: Option<String>,
    /// 基于 VChart 的图表定义
    chart_spec: Value,
    /// 图标是否可在独立窗口查看。可取值：
    ///
    /// true：默认值。
    /// PC 端：图表可在独立飞书窗口查看
    /// 移动端：图表可在点击后全屏查看
    /// false：
    /// PC 端：图表不支持在独立飞书窗口查看
    /// 移动端：图表不支持在点击后全屏查看
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<bool>,
    /// 图表组件的高度，可取值：
    ///
    /// auto：默认值，高度将根据宽高比自动计算。
    /// [1,999]px：自定义固定图表高度，此时宽高比属性 aspect_ratio 失效。
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<String>,
}

impl Default for FeishuCardChart {
    fn default() -> Self {
        Self {
            tag: "chart".to_string(),
            aspect_ratio: None,
            color_theme: None,
            chart_spec: Value::Null,
            preview: None,
            height: None,
        }
    }
}

impl FeishuCardChart {
    pub fn new() -> Self {
        FeishuCardChart::default()
    }

    pub fn aspect_ratio(mut self, aspect_ratio: &str) -> Self {
        self.aspect_ratio = Some(aspect_ratio.to_string());
        self
    }

    pub fn color_theme(mut self, color_theme: &str) -> Self {
        self.color_theme = Some(color_theme.to_string());
        self
    }

    pub fn chart_spec(mut self, chart_spec: Value) -> Self {
        self.chart_spec = chart_spec;
        self
    }

    pub fn preview(mut self, preview: bool) -> Self {
        self.preview = Some(preview);
        self
    }

    pub fn height(mut self, height: &str) -> Self {
        self.height = Some(height.to_string());
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::components::content_components::chart::FeishuCardChart;

    #[test]
    fn test_chart() {
        let chart = FeishuCardChart::new()
            .aspect_ratio("1:1")
            .color_theme("brand")
            .chart_spec(json!({
                "series": [
                    {
                        "type": "bar",
                        "data": [1, 2, 3, 4, 5]
                    }
                ]
            }))
            .preview(true)
            .height("auto");
        let json = json!({
            "tag": "chart",
            "aspect_ratio": "1:1",
            "color_theme": "brand",
            "chart_spec": {
                "series": [
                    {
                        "type": "bar",
                        "data": [1, 2, 3, 4, 5]
                    }
                ]
            },
            "preview": true,
            "height": "auto"
        });
        assert_eq!(serde_json::to_value(&chart).unwrap(), json);
    }
}

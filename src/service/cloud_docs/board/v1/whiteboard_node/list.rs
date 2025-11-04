use crate::core::SDKResult;use reqwest::Method;
use serde::{Deserialize, Serialize};

use open_lark_core::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::BOARD_V1_WHITEBOARD_NODES,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 获取画板所有节点请求
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListWhiteboardNodesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 画板token
    #[serde(skip)]
    pub whiteboard_token: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListWhiteboardNodesRequest {
    /// 创建新的获取画板节点列表请求
    pub fn new(whiteboard_token: impl Into<String>) -> Self {
        Self {
            api_request: ApiRequest::new(),
            whiteboard_token: whiteboard_token.into(),
            page_size: None,
            page_token: None,
        }
    }

    /// 创建构建器
    pub fn builder() -> ListWhiteboardNodesRequestBuilder {
        ListWhiteboardNodesRequestBuilder::default()
    }
}

/// 获取画板所有节点请求构建器
#[derive(Debug, Clone, Default)]
pub struct ListWhiteboardNodesRequestBuilder {
    request: ListWhiteboardNodesRequest,
}

impl ListWhiteboardNodesRequestBuilder {
    /// 设置画板token
    pub fn whiteboard_token(mut self, token: impl Into<String>) -> Self {
        self.request.whiteboard_token = token.into();
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, size: i32) -> Self {
        self.request.page_size = Some(size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.request.page_token = Some(token.into());
        self
    }

    /// 小页面（20个节点）
    pub fn small_page(self) -> Self {
        self.page_size(20)
    }

    /// 中等页面（50个节点）
    pub fn medium_page(self) -> Self {
        self.page_size(50)
    }

    /// 大页面（100个节点）
    pub fn large_page(self) -> Self {
        self.page_size(100)
    }

    /// 构建请求
    pub fn build(self) -> ListWhiteboardNodesRequest {
        self.request
    }
}

/// 画板节点类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    /// 形状节点
    Shape,
    /// 文本节点
    Text,
    /// 便签节点
    StickyNote,
    /// 图片节点
    Image,
    /// 连接线节点
    Line,
    /// 自由绘制节点
    Freehand,
    /// 表格节点
    Table,
    /// 框架节点
    Frame,
    /// 组合节点
    Group,
    /// 未知类型
    #[serde(other)]
    Unknown,
}

impl NodeType {
    /// 判断是否为绘图类型
    pub fn is_drawing_type(&self) -> bool {
        matches!(self, NodeType::Shape | NodeType::Line | NodeType::Freehand)
    }

    /// 判断是否为内容类型
    pub fn is_content_type(&self) -> bool {
        matches!(self, NodeType::Text | NodeType::StickyNote | NodeType::Image | NodeType::Table)
    }

    /// 判断是否为容器类型
    pub fn is_container_type(&self) -> bool {
        matches!(self, NodeType::Frame | NodeType::Group)
    }

    /// 获取类型描述
    pub fn description(&self) -> &'static str {
        match self {
            NodeType::Shape => "形状",
            NodeType::Text => "文本",
            NodeType::StickyNote => "便签",
            NodeType::Image => "图片",
            NodeType::Line => "连接线",
            NodeType::Freehand => "自由绘制",
            NodeType::Table => "表格",
            NodeType::Frame => "框架",
            NodeType::Group => "组合",
            NodeType::Unknown => "未知",
        }
    }

    /// 获取类型分类
    pub fn category(&self) -> &'static str {
        if self.is_drawing_type() {
            "绘图元素"
        } else if self.is_content_type() {
            "内容元素"
        } else if self.is_container_type() {
            "容器元素"
        } else {
            "其他元素"
        }
    }
}

/// 画板节点样式
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeStyle {
    /// 颜色
    pub color: Option<String>,
    /// 填充颜色
    pub fill_color: Option<String>,
    /// 字体大小
    pub font_size: Option<f64>,
    /// 字体粗细
    pub font_weight: Option<String>,
    /// 透明度
    pub opacity: Option<f64>,
    /// 线条粗细
    pub stroke_width: Option<f64>,
    /// 线条样式
    pub stroke_style: Option<String>,
}

impl NodeStyle {
    /// 判断是否有颜色设置
    pub fn has_color(&self) -> bool {
        self.color.is_some() || self.fill_color.is_some()
    }

    /// 判断是否有字体设置
    pub fn has_font_settings(&self) -> bool {
        self.font_size.is_some() || self.font_weight.is_some()
    }

    /// 判断是否有线条设置
    pub fn has_stroke_settings(&self) -> bool {
        self.stroke_width.is_some() || self.stroke_style.is_some()
    }

    /// 获取样式摘要
    pub fn style_summary(&self) -> Vec<String> {
        let mut summary = Vec::new();

        if let Some(ref color) = self.color {
            summary.push(format!("颜色: {color}"));
        }
        if let Some(ref fill_color) = self.fill_color {
            summary.push(format!("填充: {fill_color}"));
        }
        if let Some(font_size) = self.font_size {
            summary.push(format!("字体: {font_size:.0}px"));
        }
        if let Some(opacity) = self.opacity {
            summary.push(format!("透明度: {:.0}%", opacity * 100.0));
        }
        if let Some(stroke_width) = self.stroke_width {
            summary.push(format!("线宽: {stroke_width:.0}px"));
        }

        summary
    }
}

/// 画板节点位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePosition {
    /// X坐标
    pub x: f64,
    /// Y坐标
    pub y: f64,
    /// 宽度
    pub width: f64,
    /// 高度
    pub height: f64,
    /// 旋转角度
    pub rotation: Option<f64>,
}

impl NodePosition {
    /// 计算面积
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// 获取中心点
    pub fn center(&self) -> (f64, f64) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// 判断是否与另一个位置重叠
    pub fn overlaps_with(&self, other: &NodePosition) -> bool {
        !(self.x + self.width <= other.x
            || other.x + other.width <= self.x
            || self.y + self.height <= other.y
            || other.y + other.height <= self.y)
    }

    /// 计算到另一个位置的距离
    pub fn distance_to(&self, other: &NodePosition) -> f64 {
        let (cx1, cy1) = self.center();
        let (cx2, cy2) = other.center();
        ((cx2 - cx1).powi(2) + (cy2 - cy1).powi(2)).sqrt()
    }

    /// 获取边界描述
    pub fn bounds_description(&self) -> String {
        if let Some(rotation) = self.rotation {
            format!(
                "位置: ({:.1},{:.1}) 大小: {:.1}×{:.1} 旋转: {:.1}°",
                self.x, self.y, self.width, self.height, rotation
            )
        } else {
            format!(
                "位置: ({:.1},{:.1}) 大小: {:.1}×{:.1}",
                self.x, self.y, self.width, self.height
            )
        }
    }
}

/// 画板节点内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeContent {
    /// 文本内容
    pub text: Option<String>,
    /// 图片URL
    pub image_url: Option<String>,
    /// 图片大小
    pub image_size: Option<i64>,
    /// 连接点信息
    pub connections: Option<Vec<serde_json::Value>>,
    /// 其他自定义属性
    pub properties: Option<serde_json::Value>,
}

impl NodeContent {
    /// 判断是否有文本内容
    pub fn has_text(&self) -> bool {
        self.text.as_ref().map_or(false, |t| !t.is_empty())
    }

    /// 判断是否有图片
    pub fn has_image(&self) -> bool {
        self.image_url.is_some()
    }

    /// 判断是否有连接
    pub fn has_connections(&self) -> bool {
        self.connections.as_ref().map_or(false, |c| !c.is_empty())
    }

    /// 获取内容摘要
    pub fn content_summary(&self) -> String {
        let mut parts = Vec::new();

        if self.has_text() {
            let text = self.text.as_ref().unwrap();
            if text.len() > 20 {
                parts.push(format!("文本: {}...", &text[..20]));
            } else {
                parts.push(format!("文本: {text}"));
            }
        }

        if self.has_image() {
            if let Some(size) = self.image_size {
                parts.push(format!("图片: {size}字节"));
            } else {
                parts.push("图片".to_string());
            }
        }

        if let Some(ref connections) = self.connections {
            parts.push(format!("连接: {}个", connections.len()));
        }

        if parts.is_empty() {
            "无内容".to_string()
        } else {
            parts.join(", ")
        }
    }

    /// 获取文本长度
    pub fn text_length(&self) -> usize {
        self.text.as_ref().map_or(0, |t| t.len())
    }
}

/// 画板节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteboardNode {
    /// 节点ID
    pub node_id: String,
    /// 节点类型
    pub node_type: NodeType,
    /// 节点位置和尺寸
    pub position: NodePosition,
    /// 节点样式
    pub style: Option<NodeStyle>,
    /// 节点内容
    pub content: Option<NodeContent>,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 更新时间
    pub update_time: Option<i64>,
    /// 创建者ID
    pub creator_id: Option<String>,
    /// 最后编辑者ID
    pub last_editor_id: Option<String>,
    /// 节点层级
    pub z_index: Option<i32>,
    /// 是否锁定
    pub locked: Option<bool>,
    /// 是否可见
    pub visible: Option<bool>,
}

impl WhiteboardNode {
    /// 判断是否被锁定
    pub fn is_locked(&self) -> bool {
        self.locked.unwrap_or(false)
    }

    /// 判断是否可见
    pub fn is_visible(&self) -> bool {
        self.visible.unwrap_or(true)
    }

    /// 判断是否有内容
    pub fn has_content(&self) -> bool {
        self.content.as_ref().map_or(false, |c| c.has_text() || c.has_image() || c.has_connections())
    }

    /// 判断是否有样式
    pub fn has_style(&self) -> bool {
        self.style.as_ref().map_or(false, |s| {
            s.has_color() || s.has_font_settings() || s.has_stroke_settings()
        })
    }

    /// 获取更新时间格式化字符串
    pub fn update_time_formatted(&self) -> Option<String> {
        self.update_time.map(|timestamp| {
            let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
                .unwrap_or_else(|| chrono::Utc::now());
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    /// 获取节点摘要
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();

        parts.push(format!("类型: {}", self.node_type.description()));
        parts.push(self.position.bounds_description());

        if let Some(ref style) = self.style {
            let style_info = style.style_summary();
            if !style_info.is_empty() {
                parts.push(format!("样式: {}", style_info.join(", ")));
            }
        }

        if let Some(ref content) = self.content {
            parts.push(format!("内容: {}", content.content_summary()));
        }

        if self.is_locked() {
            parts.push("已锁定".to_string());
        }

        if !self.is_visible() {
            parts.push("已隐藏".to_string());
        }

        parts.join(" | ")
    }

    /// 获取节点状态
    pub fn status(&self) -> Vec<String> {
        let mut status = Vec::new();

        status.push(format!("ID: {}", self.node_id));
        status.push(format!(
            "类型: {} ({})",
            self.node_type.description(),
            self.node_type.category(),
        ));

        if self.is_locked() {
            status.push("🔒 已锁定".to_string());
        }

        if !self.is_visible() {
            status.push("👁 已隐藏".to_string());
        }

        if let Some(z_index) = self.z_index {
            status.push(format!("层级: {z_index}"));
        }

        status
    }

    /// 计算节点复杂度（基于内容和样式）
    pub fn complexity_score(&self) -> u32 {
        let mut score = 1; // 基础分数

        // 根据节点类型加分
        score += match self.node_type {
            NodeType::Shape | NodeType::Line => 1,
            NodeType::Text | NodeType::StickyNote => 2,
            NodeType::Image => 3,
            NodeType::Table => 4,
            NodeType::Frame | NodeType::Group => 5,
            NodeType::Freehand => 6,
            NodeType::Unknown => 0,
        };

        // 根据样式设置加分
        if let Some(ref style) = self.style {
            if style.has_color() {
                score += 1;
            }
            if style.has_font_settings() {
                score += 1;
            }
            if style.has_stroke_settings() {
                score += 1;
            }
        }

        // 根据内容加分
        if let Some(ref content) = self.content {
            if content.has_text() {
                score += content.text_length() as u32 / 10;
            }
            if content.has_image() {
                score += 2;
            }
            if content.has_connections() {
                score += 3;
            }
        }

        // 面积影响复杂度
        score += (self.position.area() / 10000.0) as u32;

        score
    }
}

/// 获取画板所有节点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWhiteboardNodesResponse {
    /// 节点列表
    pub items: Vec<WhiteboardNode>,
    /// 是否还有更多
    pub has_more: bool,
    /// 下一页标记
    pub page_token: Option<String>,
    /// 总数量
    pub total: Option<i32>,
}

impl ApiResponseTrait for ListWhiteboardNodesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListWhiteboardNodesResponse {
    /// 获取节点数量
    pub fn node_count(&self) -> usize {
        self.items.len()
    }

    /// 获取响应摘要
    pub fn summary(&self) -> String {
        let mut summary = vec![format!("节点数量: {}", self.node_count())];

        // 统计节点类型
        let mut type_counts = std::collections::HashMap::new();
        let mut locked_count = 0;
        let mut hidden_count = 0;

        for node in &self.items {
            *type_counts.entry(node.node_type.clone()).or_insert(0) += 1;

            if node.is_locked() {
                locked_count += 1;
            }

            if !node.is_visible() {
                hidden_count += 1;
            }
        }

        if !type_counts.is_empty() {
            let type_info: Vec<String> = type_counts
                .into_iter()
                .map(|(node_type, count)| format!("{}: {}个", node_type.description(), count))
                .collect();
            summary.push(format!("类型分布: {}", type_info.join(", ")));
        }

        if locked_count > 0 {
            summary.push(format!("锁定节点: {locked_count}"));
        }

        if hidden_count > 0 {
            summary.push(format!("隐藏节点: {hidden_count}"));
        }

        if self.has_more {
            summary.push("还有更多节点".to_string());
        }

        summary.join(" | ")
    }

    /// 获取分页信息
    pub fn pagination_info(&self) -> String {
        if self.has_more {
            if let Some(ref token) = self.page_token {
                format!(
                    "当前页: {}个节点，还有更多 (下一页token: {})",
                    self.node_count(),
                    token,
                )
            } else {
                format!("当前页: {}个节点，还有更多", self.node_count())
            }
        } else if let Some(total) = self.total {
            format!("全部{total}个节点已加载完成")
        } else {
            format!("全部{}个节点已加载完成", self.node_count())
        }
    }
}

// 实现Builder模式的宏调用
impl_executable_builder_owned!(
    ListWhiteboardNodesRequestBuilder,
    crate::service::cloud_docs::board::BoardService,
    ListWhiteboardNodesRequest,
    BaseResponse<ListWhiteboardNodesResponse>,
    list_nodes,
);

/// 获取画板所有节点
pub async fn list_whiteboard_nodes(
    request: ListWhiteboardNodesRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListWhiteboardNodesResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);

    let mut path = BOARD_V1_WHITEBOARD_NODES.replace("{}", &request.whiteboard_token);

    // 添加查询参数
    let mut query_params = Vec::new();

    if let Some(page_size) = request.page_size {
        query_params.push(format!("page_size={page_size}"));
    }

    if let Some(ref page_token) = request.page_token {
        query_params.push(format!("page_token={page_token}"));
    }

    if !query_params.is_empty() {
        path.push('?');
        path.push_str(&query_params.join("&"));
    }

    api_req.set_api_path(path);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_whiteboard_nodes_request_builder() {
        let request = ListWhiteboardNodesRequest::builder()
            .whiteboard_token("wbdcnxxxxxx")
            .medium_page()
            .build();

        assert_eq!(request.whiteboard_token, "wbdcnxxxxxx");
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_node_type_methods() {
        assert!(NodeType::Shape.is_drawing_type());
        assert!(NodeType::Text.is_content_type());
        assert!(NodeType::Frame.is_container_type());

        assert_eq!(NodeType::Shape.description(), "形状");
        assert_eq!(NodeType::Shape.category(), "绘图元素");
    }

    #[test]
    fn test_node_position_methods() {
        let pos1 = NodePosition {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            rotation: None,
        };

        let pos2 = NodePosition {
            x: 50.0,
            y: 50.0,
            width: 100.0,
            height: 100.0,
            rotation: None,
        };

        assert_eq!(pos1.area(), 10000.0);
        assert_eq!(pos1.center(), (50.0, 50.0));
        assert!(pos1.overlaps_with(&pos2));
        assert!(pos1.distance_to(&pos2) > 0.0);
    }

    #[test]
    fn test_whiteboard_node_methods() {
        let node = WhiteboardNode {
            node_id: "node123".to_string(),
            node_type: NodeType::Text,
            position: NodePosition {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 50.0,
                rotation: None,
            },
            style: None,
            content: Some(NodeContent {
                text: Some("Hello".to_string()),
                image_url: None,
                image_size: None,
                connections: None,
                properties: None,
            }),
            create_time: None,
            update_time: None,
            creator_id: None,
            last_editor_id: None,
            z_index: Some(1),
            locked: Some(false),
            visible: Some(true),
        };

        assert!(!node.is_locked());
        assert!(node.is_visible());
        assert!(node.has_content());
        assert!(!node.has_style());
        assert!(node.complexity_score() > 0);
    }
}
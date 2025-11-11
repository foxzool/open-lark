//! 画板API v1版本
//!
//! 实现所有画板相关的API接口，共4个：
//! - 画板主题管理 (1个API)
//! - 画板缩略图获取 (1个API)
//! - 节点管理 (2个API)

use openlark_core::config::Config;
use openlark_core::prelude::*;
use serde::{Deserialize, Serialize};

/// 画板服务 v1版本
#[derive(Debug, Clone)]
pub struct BoardServiceV1 {
    pub config: Config,
}

impl BoardServiceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 画板管理 ====================

    /// 获取画板主题
    pub async fn get_whiteboard_theme(
        &self,
        _request: &GetWhiteboardThemeRequest,
    ) -> SDKResult<WhiteboardThemeResponse> {
        // 模拟实现
        Ok(WhiteboardThemeResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(WhiteboardThemeData {
                theme_id: "test_theme_id".to_string(),
                theme_name: "默认主题".to_string(),
                background_color: "#FFFFFF".to_string(),
                grid_color: "#E5E5E5".to_string(),
                created_time: "2024-01-01T00:00:00Z".to_string(),
                updated_time: "2024-01-01T00:00:00Z".to_string(),
            }),
        })
    }

    /// 获取画板缩略图片
    pub async fn download_whiteboard_as_image(
        &self,
        _request: &DownloadWhiteboardAsImageRequest,
    ) -> SDKResult<WhiteboardImageResponse> {
        // 模拟实现
        Ok(WhiteboardImageResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(WhiteboardImageData {
                image_url: "https://example.com/whiteboard_thumbnail.png".to_string(),
                image_width: 1920,
                image_height: 1080,
                file_size: 1024000,
                mime_type: "image/png".to_string(),
                created_time: "2024-01-01T00:00:00Z".to_string(),
            }),
        })
    }

    // ==================== 节点管理 ====================

    /// 创建节点
    pub async fn create_whiteboard_node(
        &self,
        _request: &CreateWhiteboardNodeRequest,
    ) -> SDKResult<WhiteboardNodeResponse> {
        // 模拟实现
        Ok(WhiteboardNodeResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(WhiteboardNodeData {
                node_id: "test_node_id".to_string(),
                node_type: "text".to_string(),
                content: "测试节点内容".to_string(),
                x: 100.0,
                y: 100.0,
                width: 200.0,
                height: 100.0,
                rotation: 0.0,
                created_time: "2024-01-01T00:00:00Z".to_string(),
                updated_time: "2024-01-01T00:00:00Z".to_string(),
            }),
        })
    }

    /// 获取所有节点
    pub async fn list_whiteboard_nodes(
        &self,
        _request: &ListWhiteboardNodesRequest,
    ) -> SDKResult<WhiteboardNodesResponse> {
        // 模拟实现
        Ok(WhiteboardNodesResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(WhiteboardNodesData {
                nodes: vec![],
                page_token: None,
                has_more: false,
                total: 0,
            }),
        })
    }
}

// ==================== 数据模型定义 ====================

// 画板主题相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWhiteboardThemeRequest {
    pub whiteboard_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteboardThemeResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<WhiteboardThemeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteboardThemeData {
    pub theme_id: String,
    pub theme_name: String,
    pub background_color: String,
    pub grid_color: String,
    pub created_time: String,
    pub updated_time: String,
}

// 画板缩略图相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadWhiteboardAsImageRequest {
    pub whiteboard_id: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub quality: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteboardImageResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<WhiteboardImageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteboardImageData {
    pub image_url: String,
    pub image_width: i32,
    pub image_height: i32,
    pub file_size: i64,
    pub mime_type: String,
    pub created_time: String,
}

// 节点管理相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWhiteboardNodeRequest {
    pub whiteboard_id: String,
    pub node_type: String,
    pub content: String,
    pub x: f64,
    pub y: f64,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub rotation: Option<f64>,
    pub style: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWhiteboardNodesRequest {
    pub whiteboard_id: String,
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
    pub node_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteboardNodeResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<WhiteboardNodeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteboardNodeData {
    pub node_id: String,
    pub node_type: String,
    pub content: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub rotation: f64,
    pub created_time: String,
    pub updated_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteboardNodesResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<WhiteboardNodesData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteboardNodesData {
    pub nodes: Vec<WhiteboardNodeData>,
    pub page_token: Option<String>,
    pub has_more: bool,
    pub total: i32,
}

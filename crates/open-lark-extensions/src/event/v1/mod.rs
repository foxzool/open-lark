//! 事件API v1版本
//!
//! 实现所有事件相关的API接口，共1个：
//! - 事件出口IP地址获取 (1个API)

use open_lark_core::core::config::Config;
use open_lark_core::prelude::*;
use serde::{Deserialize, Serialize};

/// 事件服务 v1版本
#[derive(Debug, Clone)]
pub struct EventServiceV1 {
    pub config: Config,
}

impl EventServiceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 事件管理 ====================

    /// 获取事件出口IP地址
    pub async fn get_outbound_ip(
        &self,
        _request: &GetOutboundIpRequest,
    ) -> SDKResult<OutboundIpResponse> {
        // 模拟实现
        Ok(OutboundIpResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(OutboundIpData {
                ips: vec![
                    "203.107.45.100".to_string(),
                    "203.107.45.101".to_string(),
                    "203.107.45.102".to_string(),
                ],
                updated_time: "2024-01-01T00:00:00Z".to_string(),
            }),
        })
    }
}

// ==================== 数据模型定义 ====================

// 事件出口IP相关
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutboundIpRequest {
    // 请求参数为空，只包含路径参数
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundIpResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<OutboundIpData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundIpData {
    pub ips: Vec<String>,
    pub updated_time: String,
}

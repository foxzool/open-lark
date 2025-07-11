//! 画板模块
//!
//! 提供画板相关功能：
//! - 画板节点管理

pub mod v1;

use std::sync::Arc;

use crate::core::{config::Config, req_option::RequestOption, SDKResult};

use self::v1::*;

/// 画板服务
pub struct BoardService {
    config: Arc<Config>,
    pub whiteboard: v1::whiteboard::WhiteboardService,
}

impl BoardService {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            whiteboard: v1::whiteboard::WhiteboardService::new((*config).clone()),
            config,
        }
    }

    /// 获取画板所有节点
    pub async fn list_nodes(
        &self,
        request: ListWhiteboardNodesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<ListWhiteboardNodesResponse>> {
        list_whiteboard_nodes(request, &self.config, option).await
    }
}

//! 画板模块
//!
//! 提供画板相关功能：
//! - 画板节点管理

pub mod v1;

use crate::core::{config::Config, req_option::RequestOption, SDKResult};

use self::v1::*;

/// 画板服务
pub struct BoardService {
    config: Config,
    pub whiteboard: v1::whiteboard::WhiteboardService,
}

impl BoardService {
    pub fn new(config: Config) -> Self {
        Self {
            whiteboard: v1::whiteboard::WhiteboardService::new(config.clone()),
            config,
        }
    }

    /// 使用共享配置（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            whiteboard: v1::whiteboard::WhiteboardService::new(shared.as_ref().clone()),
            config: (*shared).clone(),
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

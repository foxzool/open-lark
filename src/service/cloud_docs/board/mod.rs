#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 画板模块,
//!,
//! 提供画板相关功能：,
//! - 画板节点管理,
pub mod v1;
use crate::core::{config::Config, req_option::RequestOption, SDKResult};
use self::v1::*;
/// 画板服务
pub struct BoardService {
    config: Config,
}    pub whiteboard: v1::whiteboard::WhiteboardService}
impl BoardService {
}
/// 使用共享配置（实验性）
    pub fn new_from_shared() -> Self {
Self {
            whiteboard: v1::whiteboard::WhiteboardService::new(shared.as_ref().clone()),
            config: (*shared).clone()}
}
/// 获取画板所有节点
    pub async fn list_nodes(
        &self,
        request: ListWhiteboardNodesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<ListWhiteboardNodesResponse>> {
        list_whiteboard_nodes(request, &self.config, option).await}
}

/// 移动知识空间节点
///
/// 此接口用于在知识空间中移动节点到新的父节点下，重新组织知识库的层级结构。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/move

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};

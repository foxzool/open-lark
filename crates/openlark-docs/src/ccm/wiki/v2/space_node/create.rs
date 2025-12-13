/// 创建知识空间节点
///
/// 此接口用于在知识空间中创建新的文档节点。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};

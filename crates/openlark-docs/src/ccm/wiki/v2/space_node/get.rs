/// 获取知识空间节点详情
///
/// 此接口用于获取知识空间节点的详细信息，包括节点标题、类型、创建时间等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};

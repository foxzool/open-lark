/// 更新知识空间节点标题
///
/// 此接口用于更新知识空间节点的标题，支持修改文档或文件夹的名称。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/update_title

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
